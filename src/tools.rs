use crate::error::{DecodeError, Result};
use std::path::PathBuf;

pub fn locate_tools() -> Result<(PathBuf, PathBuf)> {
    // Try local copy first / 首先尝试本地副本
    let local_gst = PathBuf::from("./dolby-tools/gstreamer/bin/gst-launch-1.0");
    let local_plugins = PathBuf::from("./dolby-tools/gst-plugins");

    if local_gst.exists() && local_plugins.exists() {
        println!("使用本地 GStreamer 工具/Using local Dolby tools");
        return Ok((local_gst, local_plugins));
    }

    // Fallback to system Dolby Reference Player / 回退到系统安装的播放器
    let drp_base = PathBuf::from("/Applications/Dolby/Dolby Reference Player.app/Contents");

    if !drp_base.exists() {
        return Err(DecodeError::ToolsNotFound(
            "未找到 Dolby Reference Player，请从 https://professional.dolby.com/product/media-processing-and-delivery/drp---dolby-reference-player/ 安装/Dolby Reference Player not found. Install it from https://professional.dolby.com/product/media-processing-and-delivery/drp---dolby-reference-player/"
                .to_string(),
        ));
    }

    let gst_launch =
        drp_base.join("Frameworks/GStreamer.framework/Versions/1_22/Resources/bin/gst-launch-1.0");
    let gst_plugins = drp_base.join("PlugIns/gst-plugins");

    if !gst_launch.exists() || !gst_plugins.exists() {
        return Err(DecodeError::ToolsNotFound(
            "未找到 Dolby Reference Player 组件/Dolby Reference Player components not found"
                .to_string(),
        ));
    }

    println!("使用系统安装的 GStreamer/Using system Dolby Reference Player");
    Ok((gst_launch, gst_plugins))
}
