use crate::error::{DecodeError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub id: u32,
    pub names: Vec<String>,
}

pub fn get_config(config_name: &str) -> Result<ChannelConfig> {
    let mut configs = HashMap::new();

    // 2.0
    configs.insert(
        "2.0",
        ChannelConfig {
            id: 0,
            names: vec!["L".to_string(), "R".to_string()],
        },
    );

    // 3.1
    configs.insert(
        "3.1",
        ChannelConfig {
            id: 3,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
            ],
        },
    );

    // 5.1
    configs.insert(
        "5.1",
        ChannelConfig {
            id: 7,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
            ],
        },
    );

    // 7.1
    configs.insert(
        "7.1",
        ChannelConfig {
            id: 11,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
            ],
        },
    );

    // 9.1
    configs.insert(
        "9.1",
        ChannelConfig {
            id: 12,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Lw".to_string(),
                "Rw".to_string(),
            ],
        },
    );

    // 5.1.2
    configs.insert(
        "5.1.2",
        ChannelConfig {
            id: 13,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Ltm".to_string(),
                "Rtm".to_string(),
            ],
        },
    );

    // 5.1.4
    configs.insert(
        "5.1.4",
        ChannelConfig {
            id: 14,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Ltf".to_string(),
                "Rtf".to_string(),
                "Ltr".to_string(),
                "Rtr".to_string(),
            ],
        },
    );

    // 7.1.2
    configs.insert(
        "7.1.2",
        ChannelConfig {
            id: 15,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Ltm".to_string(),
                "Rtm".to_string(),
            ],
        },
    );

    // 7.1.4
    configs.insert(
        "7.1.4",
        ChannelConfig {
            id: 16,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Ltf".to_string(),
                "Rtf".to_string(),
                "Ltr".to_string(),
                "Rtr".to_string(),
            ],
        },
    );

    // 7.1.6
    configs.insert(
        "7.1.6",
        ChannelConfig {
            id: 17,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Ltf".to_string(),
                "Rtf".to_string(),
                "Ltm".to_string(),
                "Rtm".to_string(),
                "Ltr".to_string(),
                "Rtr".to_string(),
            ],
        },
    );

    // 9.1.2
    configs.insert(
        "9.1.2",
        ChannelConfig {
            id: 18,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Lw".to_string(),
                "Rw".to_string(),
                "Ltm".to_string(),
                "Rtm".to_string(),
            ],
        },
    );

    // 9.1.4
    configs.insert(
        "9.1.4",
        ChannelConfig {
            id: 19,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Lw".to_string(),
                "Rw".to_string(),
                "Ltf".to_string(),
                "Rtf".to_string(),
                "Ltr".to_string(),
                "Rtr".to_string(),
            ],
        },
    );

    // 9.1.6
    configs.insert(
        "9.1.6",
        ChannelConfig {
            id: 20,
            names: vec![
                "L".to_string(),
                "R".to_string(),
                "C".to_string(),
                "LFE".to_string(),
                "Ls".to_string(),
                "Rs".to_string(),
                "Lrs".to_string(),
                "Rrs".to_string(),
                "Lw".to_string(),
                "Rw".to_string(),
                "Ltf".to_string(),
                "Rtf".to_string(),
                "Ltm".to_string(),
                "Rtm".to_string(),
                "Ltr".to_string(),
                "Rtr".to_string(),
            ],
        },
    );

    configs.get(config_name)
        .cloned()
        .ok_or_else(|| DecodeError::InvalidChannelConfig(
            format!("未知声道配置/Unknown channel configuration: {config_name}. 支持的配置/Supported: 2.0, 3.1, 5.1, 7.1, 9.1, 5.1.2, 5.1.4, 7.1.2, 7.1.4, 7.1.6, 9.1.2, 9.1.4, 9.1.6")
        ))
}
