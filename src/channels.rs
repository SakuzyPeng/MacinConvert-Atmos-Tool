use crate::error::{DecodeError, Result};

#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub name: String,
    pub id: u32,
    pub names: Vec<String>,
}

struct ChannelDef {
    name: &'static str,
    id: u32,
    names: &'static [&'static str],
}

const CONFIGS: &[ChannelDef] = &[
    ChannelDef {
        name: "2.0",
        id: 0,
        names: &["L", "R"],
    },
    ChannelDef {
        name: "3.1",
        id: 3,
        names: &["L", "R", "C", "LFE"],
    },
    ChannelDef {
        name: "5.1",
        id: 7,
        names: &["L", "R", "C", "LFE", "Ls", "Rs"],
    },
    ChannelDef {
        name: "7.1",
        id: 11,
        names: &["L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs"],
    },
    ChannelDef {
        name: "9.1",
        id: 12,
        names: &["L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Lw", "Rw"],
    },
    ChannelDef {
        name: "5.1.2",
        id: 13,
        names: &["L", "R", "C", "LFE", "Ls", "Rs", "Ltm", "Rtm"],
    },
    ChannelDef {
        name: "5.1.4",
        id: 14,
        names: &["L", "R", "C", "LFE", "Ls", "Rs", "Ltf", "Rtf", "Ltr", "Rtr"],
    },
    ChannelDef {
        name: "7.1.2",
        id: 15,
        names: &["L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Ltm", "Rtm"],
    },
    ChannelDef {
        name: "7.1.4",
        id: 16,
        names: &[
            "L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Ltf", "Rtf", "Ltr", "Rtr",
        ],
    },
    ChannelDef {
        name: "7.1.6",
        id: 17,
        names: &[
            "L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Ltf", "Rtf", "Ltm", "Rtm", "Ltr",
            "Rtr",
        ],
    },
    ChannelDef {
        name: "9.1.2",
        id: 18,
        names: &[
            "L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Lw", "Rw", "Ltm", "Rtm",
        ],
    },
    ChannelDef {
        name: "9.1.4",
        id: 19,
        names: &[
            "L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Lw", "Rw", "Ltf", "Rtf", "Ltr", "Rtr",
        ],
    },
    ChannelDef {
        name: "9.1.6",
        id: 20,
        names: &[
            "L", "R", "C", "LFE", "Ls", "Rs", "Lrs", "Rrs", "Lw", "Rw", "Ltf", "Rtf", "Ltm", "Rtm",
            "Ltr", "Rtr",
        ],
    },
];

pub fn get_config(config_name: &str) -> Result<ChannelConfig> {
    // 处理特殊的"auto"配置 / Handle special "auto" configuration
    // 在此模式下，解码器将不会指定 out-ch-config，使用文件的原生声道配置
    // In this mode, the decoder won't specify out-ch-config, using the file's native configuration
    if config_name.eq_ignore_ascii_case("auto") {
        return Ok(ChannelConfig {
            name: "auto".to_string(),
            id: u32::MAX, // 使用特殊的 id 标记 / Use special id as marker
            names: vec![],
        });
    }

    if let Some(def) = CONFIGS
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(config_name))
    {
        return Ok(ChannelConfig {
            name: def.name.to_string(),
            id: def.id,
            names: def.names.iter().map(|s| (*s).to_string()).collect(),
        });
    }
    let supported = CONFIGS
        .iter()
        .map(|d| d.name)
        .collect::<Vec<_>>()
        .join(", ");
    Err(DecodeError::InvalidChannelConfig(format!(
        "未知声道配置/Unknown channel configuration: {config_name}. 支持的配置/Supported: {supported}, auto（自动检测文件原生声道 / auto-detect file's native channels）"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 有效配置返回正确的 name/id/names / Valid configs return correct name/id/names
    #[test]
    fn get_config_returns_expected_known_configs() {
        let c20 = get_config("2.0").unwrap();
        assert_eq!(c20.name, "2.0");
        assert_eq!(c20.id, 0);
        assert_eq!(c20.names, vec!["L", "R"]);

        let c51 = get_config("5.1").unwrap();
        assert_eq!(c51.name, "5.1");
        assert_eq!(c51.id, 7);
        assert_eq!(c51.names.len(), 6);

        let c916 = get_config("9.1.6").unwrap();
        assert_eq!(c916.name, "9.1.6");
        assert_eq!(c916.id, 20);
        assert_eq!(c916.names.len(), 16);
    }

    // "auto" 返回特殊 id 与空声道列表 / "auto" returns sentinel id and empty channel list
    #[test]
    fn get_config_auto_is_case_insensitive_with_sentinel() {
        for name in ["auto", "Auto", "AUTO"] {
            let cfg = get_config(name).unwrap();
            assert_eq!(cfg.name, "auto");
            assert_eq!(cfg.id, u32::MAX);
            assert!(cfg.names.is_empty());
        }
    }

    // 未知配置返回 InvalidChannelConfig / Unknown config returns InvalidChannelConfig
    #[test]
    fn get_config_unknown_returns_error() {
        let err = get_config("4.2.0").unwrap_err();
        assert!(matches!(err, DecodeError::InvalidChannelConfig(_)));
    }

    // 健康检查：名称与 id 均无重复 / Health check: no duplicate names or ids
    #[test]
    fn config_table_has_no_duplicate_names_or_ids() {
        for (i, a) in CONFIGS.iter().enumerate() {
            for b in &CONFIGS[i + 1..] {
                assert_ne!(
                    a.name, b.name,
                    "重复配置名/Duplicate config name: {}",
                    a.name
                );
                assert_ne!(a.id, b.id, "重复配置 id/Duplicate config id: {}", a.id);
            }
        }
    }
}
