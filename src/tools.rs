use crate::error::{DecodeError, Result};
use std::env;
use std::path::{Path, PathBuf};

pub fn locate_tools(cli_base: Option<&Path>) -> Result<(PathBuf, PathBuf)> {
    // 0) CLI override / 命令行参数优先
    if let Some(base) = cli_base {
        let gst = base.join("gstreamer/bin/gst-launch-1.0");
        let plugins = base.join("gst-plugins");
        if gst.exists() && plugins.exists() {
            println!("使用命令行指定的 dolby-tools 目录/Using dolby-tools from --dolby-tools");
            return Ok((gst, plugins));
        } else {
            return Err(DecodeError::ToolsNotFound(
                format!(
                    "--dolby-tools 路径无效，应包含: gstreamer/bin/gst-launch-1.0 与 gst-plugins/Invalid --dolby-tools path; expected layout with gstreamer/bin/gst-launch-1.0 and gst-plugins: {}",
                    base.display()
                ),
            ));
        }
    }

    // 1) Explicit env overrides / 显式环境变量覆盖
    if let (Ok(gst_launch_s), Ok(gst_plugins_s)) =
        (env::var("MCAT_GST_LAUNCH"), env::var("MCAT_GST_PLUGINS"))
    {
        let gst_launch = PathBuf::from(gst_launch_s);
        let gst_plugins = PathBuf::from(gst_plugins_s);
        if gst_launch.exists() && gst_plugins.exists() {
            println!(
                "使用环境变量中的 GStreamer 路径/Using GStreamer paths from environment variables"
            );
            return Ok((gst_launch, gst_plugins));
        }
    }

    // 2) Base directory via env / 通过环境变量指定基目录
    if let Ok(base) = env::var("MCAT_DOLBY_TOOLS") {
        let base = PathBuf::from(base);
        let env_gst = base.join("gstreamer/bin/gst-launch-1.0");
        let env_plugins = base.join("gst-plugins");
        if env_gst.exists() && env_plugins.exists() {
            println!(
                "使用环境变量指定的 dolby-tools 目录/Using dolby-tools from environment variable"
            );
            return Ok((env_gst, env_plugins));
        }
    }

    // 3) Try alongside executable: <exe_dir>/dolby-tools / 可执行文件同目录的 dolby-tools
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let exe_gst = exe_dir.join("dolby-tools/gstreamer/bin/gst-launch-1.0");
            let exe_plugins = exe_dir.join("dolby-tools/gst-plugins");
            if exe_gst.exists() && exe_plugins.exists() {
                println!(
                    "使用可执行文件同目录的 dolby-tools/Using dolby-tools next to the executable"
                );
                return Ok((exe_gst, exe_plugins));
            }
        }
    }

    // 4) Fallback to system Dolby Reference Player / 回退到系统安装的播放器
    let drp_base = PathBuf::from("/Applications/Dolby/Dolby Reference Player.app/Contents");

    if !drp_base.exists() {
        return Err(DecodeError::ToolsNotFound(
            "未找到 Dolby 工具；请设置 MCAT_DOLBY_TOOLS 或安装 Dolby Reference Player/Dolby tools not found; set MCAT_DOLBY_TOOLS or install Dolby Reference Player"
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
