use crate::error::{DecodeError, Result};

#[derive(Debug, Clone)]
pub struct ChannelConfig {
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
    if let Some(def) = CONFIGS
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(config_name))
    {
        return Ok(ChannelConfig {
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
        "未知声道配置/Unknown channel configuration: {config_name}. 支持的配置/Supported: {supported}"
    )))
}
