use crate::channels::ChannelConfig;
use crate::error::{DecodeError, Result};
use crate::format::AudioFormat;
use rayon::{prelude::*, ThreadPoolBuilder};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn find_gst_scanner(gst_launch: &Path) -> Option<PathBuf> {
    let bin_dir = gst_launch.parent()?;
    let root = bin_dir.parent()?;
    let candidates = [
        root.join("libexec/gst-plugin-scanner"),
        root.join("libexec/gstreamer-1.0/gst-plugin-scanner"),
    ];
    candidates.into_iter().find(|cand| cand.exists())
}

#[allow(clippy::too_many_arguments)]
pub fn decode(
    input_file: &Path,
    output_base: Option<&PathBuf>,
    gst_launch: &Path,
    gst_plugins: &Path,
    audio_format: AudioFormat,
    channel_config: &ChannelConfig,
    single: bool,
    no_numbers: bool,
) -> Result<Vec<PathBuf>> {
    // 处理 "auto" 模式：先解码第一个声道来获取实际声道数 / Handle "auto" mode: first decode one channel to get actual count
    if channel_config.id == u32::MAX {
        return decode_auto(
            input_file,
            output_base,
            gst_launch,
            gst_plugins,
            audio_format,
            single,
            no_numbers,
        );
    }

    let mut decoded_files = Vec::new();
    let mut handles = Vec::new();
    let gst_scanner = find_gst_scanner(gst_launch);

    for (channel_id, channel_name) in channel_config.names.iter().enumerate() {
        let suffix = if no_numbers {
            format!(".{channel_name}.wav")
        } else {
            format!(".{:02}_{channel_name}.wav", channel_id + 1)
        };

        let out_path = output_base.map_or_else(
            || input_file.with_extension(&suffix[1..]),
            |base| base.with_extension(&suffix[1..]),
        );

        // 若输出已存在，先删除，避免下游 filesink 行为受影响 / Remove existing output to avoid sink quirks
        if out_path.exists() {
            let _ = std::fs::remove_file(&out_path);
        }
        decoded_files.push(out_path.clone());

        let command = build_gstreamer_command(
            input_file,
            &out_path,
            channel_id,
            channel_config.id,
            gst_launch,
            gst_plugins,
            audio_format,
        );

        if single {
            println!(
                "正在解码声道 {}：{}/Decoding channel {}：{}",
                channel_id + 1,
                channel_name,
                channel_id + 1,
                channel_name
            );
            execute_command(&command, gst_scanner.as_deref())?;
        } else {
            handles.push((channel_id + 1, channel_name.clone(), command));
        }
    }

    // Execute parallel decoding (rayon) / 执行并行解码（rayon）
    if !single && !handles.is_empty() {
        println!(
            "并行解码 {} 个声道/Decoding {} channels in parallel",
            handles.len(),
            handles.len()
        );
        // 线程数 = 环境变量 MCAT_MAX_PAR 或 CPU 数，且至少 2，不超过声道数 / threads = env or CPUs, min 2, <= channels
        // 默认并发设为 4，更符合当前解码/IO 性能特性；当 CPU 少于 4 时退化为 CPU 数且至少 2
        let default_threads = num_cpus::get().clamp(2, 4);
        let requested_threads = std::env::var("MCAT_MAX_PAR")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&n| n >= 1)
            .unwrap_or(default_threads);
        let max_parallel = std::cmp::min(requested_threads, handles.len());
        let pool = ThreadPoolBuilder::new()
            .num_threads(max_parallel)
            .build()
            .map_err(|e| {
                DecodeError::GStreamerFailed(format!(
                    "创建线程池失败/Failed to build thread pool: {e}"
                ))
            })?;

        pool.install(|| -> Result<()> {
            handles
                .par_iter()
                .map(|(id, name, command)| {
                    println!("正在解码声道 {id}：{name}/Decoding channel {id}：{name}");
                    execute_command(command, gst_scanner.as_deref())
                })
                .collect::<Result<()>>()
        })?;
    }

    Ok(decoded_files)
}

fn decode_auto(
    input_file: &Path,
    output_base: Option<&PathBuf>,
    gst_launch: &Path,
    gst_plugins: &Path,
    audio_format: AudioFormat,
    single: bool,
    no_numbers: bool,
) -> Result<Vec<PathBuf>> {
    let mut decoded_files = Vec::new();
    let mut handles = Vec::new();
    let gst_scanner = find_gst_scanner(gst_launch);

    // 自动模式：尝试解码最多 32 个声道（通常文件不会这么多）
    // Auto mode: try decoding up to 32 channels (files typically don't have this many)
    println!(
        "自动模式：检测文件的原生声道配置/Auto mode: detecting file's native channel configuration"
    );

    const MAX_AUTO_CHANNELS: usize = 32;
    for channel_id in 0..MAX_AUTO_CHANNELS {
        let channel_name = format!("CH{channel_id}");
        let suffix = if no_numbers {
            format!(".{channel_name}.wav")
        } else {
            format!(".{:02}_{channel_name}.wav", channel_id + 1)
        };

        let out_path = output_base.map_or_else(
            || input_file.with_extension(&suffix[1..]),
            |base| base.with_extension(&suffix[1..]),
        );

        // 若输出已存在，先删除，避免下游 filesink 行为受影响 / Remove existing output to avoid sink quirks
        if out_path.exists() {
            let _ = std::fs::remove_file(&out_path);
        }
        decoded_files.push(out_path.clone());

        let command = build_gstreamer_command_auto(
            input_file,
            &out_path,
            channel_id,
            gst_launch,
            gst_plugins,
            audio_format,
        );

        if single {
            println!(
                "正在解码声道 {}：{}/Decoding channel {}：{}",
                channel_id, &channel_name, channel_id, &channel_name
            );
            match execute_command(&command, gst_scanner.as_deref()) {
                Ok(()) => {
                    // 成功了，继续下一个声道 / Success, continue to next channel
                }
                Err(_) => {
                    // 解码失败，说明没有这个声道了，删除输出文件并停止
                    // Decode failed, this channel doesn't exist, remove output and stop
                    let _ = std::fs::remove_file(&out_path);
                    decoded_files.pop();
                    println!("已检测到 {channel_id} 个声道/Detected {channel_id} channels");
                    break;
                }
            }
        } else {
            handles.push((channel_id, channel_name, command));
        }
    }

    // 并行模式处理 / Handle parallel mode
    if !single && !handles.is_empty() {
        // 在并行模式下，我们需要逐个尝试声道直到失败
        // In parallel mode, we need to try channels one by one until one fails
        // 为了简化，我们还是逐个处理 / For simplicity, process one by one
        println!(
            "自动模式不支持并行解码，转换为顺序解码/Auto mode doesn't support parallel decoding, switching to sequential"
        );
        for (channel_id, channel_name, command) in handles {
            println!(
                "正在解码声道 {}：{}/Decoding channel {}：{}",
                channel_id, &channel_name, channel_id, &channel_name
            );
            match execute_command(&command, gst_scanner.as_deref()) {
                Ok(()) => {
                    // 成功 / Success
                }
                Err(_) => {
                    // 失败，停止 / Failed, stop
                    let target_suffix = format!(".{:02}_{channel_name}.wav", channel_id + 1);
                    if let Some(pos) = decoded_files
                        .iter()
                        .position(|p| p.ends_with(&target_suffix))
                    {
                        let _ = std::fs::remove_file(&decoded_files[pos]);
                        decoded_files.pop();
                    }
                    println!("已检测到 {channel_id} 个声道/Detected {channel_id} channels");
                    break;
                }
            }
        }
    }

    Ok(decoded_files)
}

fn build_gstreamer_command_auto(
    input_file: &Path,
    output_file: &Path,
    channel_id: usize,
    gst_launch: &Path,
    gst_plugins: &Path,
    format: AudioFormat,
) -> Vec<String> {
    let mut cmd = vec![
        gst_launch.to_string_lossy().to_string(),
        "--gst-plugin-path".to_string(),
        gst_plugins.to_string_lossy().to_string(),
        "filesrc".to_string(),
        format!("location={}", input_file.display()),
        "!".to_string(),
    ];

    match format {
        AudioFormat::Eac3 => {
            cmd.extend(vec!["dlbac3parse".to_string(), "!".to_string()]);
        }
        AudioFormat::TrueHD => {
            cmd.extend(vec![
                "dlbtruehdparse".to_string(),
                "align-major-sync=false".to_string(),
                "!".to_string(),
            ]);
        }
    }

    // dlbaudiodecbin with max out-ch-config to get all available channels / 使用最大的 out-ch-config 以获得所有可用声道
    cmd.push("dlbaudiodecbin".to_string());
    if matches!(format, AudioFormat::TrueHD) {
        let pres = env::var("MCAT_TRUEHD_PRESENTATION")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(16);
        cmd.push(format!("truehddec::presentation={pres}"));
    }
    // 使用最高的声道配置（20 = 9.1.6）以获得文件中所有可用的声道
    // Use highest channel config (20 = 9.1.6) to get all available channels in file
    cmd.push("out-ch-config=20".to_string());

    cmd.extend(vec![
        "!".to_string(),
        "deinterleave".to_string(),
        "name=d".to_string(),
        format!("d.src_{channel_id}"),
        "!".to_string(),
        "queue".to_string(),
        "!".to_string(),
        "audioconvert".to_string(),
        "!".to_string(),
        "audio/x-raw,format=F32LE".to_string(),
        "!".to_string(),
        "wavenc".to_string(),
        "!".to_string(),
        "filesink".to_string(),
        "sync=false".to_string(),
        format!("location={}", output_file.display()),
    ]);

    cmd
}

fn build_gstreamer_command(
    input_file: &Path,
    output_file: &Path,
    channel_id: usize,
    out_ch_config: u32,
    gst_launch: &Path,
    gst_plugins: &Path,
    format: AudioFormat,
) -> Vec<String> {
    let mut cmd = vec![
        gst_launch.to_string_lossy().to_string(),
        "--gst-plugin-path".to_string(),
        gst_plugins.to_string_lossy().to_string(),
        "filesrc".to_string(),
        format!("location={}", input_file.display()),
        "!".to_string(),
    ];

    match format {
        AudioFormat::Eac3 => {
            cmd.extend(vec!["dlbac3parse".to_string(), "!".to_string()]);
        }
        AudioFormat::TrueHD => {
            cmd.extend(vec![
                "dlbtruehdparse".to_string(),
                "align-major-sync=false".to_string(),
                "!".to_string(),
            ]);
        }
    }

    // dlbaudiodecbin + properties / 杜比音频解码器及其属性
    cmd.push("dlbaudiodecbin".to_string());
    if matches!(format, AudioFormat::TrueHD) {
        let pres = env::var("MCAT_TRUEHD_PRESENTATION")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(16);
        cmd.push(format!("truehddec::presentation={pres}"));
    }

    // 明确指定声道配置 / Explicitly specify channel configuration
    cmd.push(format!("out-ch-config={out_ch_config}"));

    cmd.extend(vec![
        "!".to_string(),
        "deinterleave".to_string(),
        "name=d".to_string(),
        format!("d.src_{channel_id}"),
        "!".to_string(),
        "queue".to_string(),
        "!".to_string(),
        "audioconvert".to_string(),
        "!".to_string(),
        "audio/x-raw,format=F32LE".to_string(),
        "!".to_string(),
        "wavenc".to_string(),
        "!".to_string(),
        "filesink".to_string(),
        "sync=false".to_string(),
        format!("location={}", output_file.display()),
    ]);

    cmd
}

fn execute_command(command: &[String], gst_scanner: Option<&Path>) -> Result<()> {
    let mut cmd = Command::new(&command[0]);
    cmd.args(&command[1..]);

    // 为本地 Dolby 工具设置 DYLD_LIBRARY_PATH/Set DYLD_LIBRARY_PATH for local Dolby tools
    if std::path::Path::new("./dolby-tools").exists() {
        let libs_path = std::path::PathBuf::from("./dolby-tools/gst-plugins-libs")
            .canonicalize()
            .ok();

        if let Some(libs) = libs_path {
            let mut dyld_path = libs.to_string_lossy().to_string();
            if let Ok(existing) = env::var("DYLD_LIBRARY_PATH") {
                dyld_path = format!("{dyld_path}:{existing}");
            }
            cmd.env("DYLD_LIBRARY_PATH", dyld_path);
        }
    }

    if let Some(scanner) = gst_scanner {
        cmd.env("GST_PLUGIN_SCANNER", scanner);
    }

    if std::env::var("MCAT_VERBOSE_GST").is_ok() {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else {
        // 丢弃子进程输出，避免在高并发下因管道缓冲阻塞 / Drop child stdio to avoid pipe blocking under high concurrency
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd.status().map_err(|e| {
        DecodeError::GStreamerFailed(format!(
            "无法执行 gst-launch/Failed to execute gst-launch: {e}"
        ))
    })?;

    if !status.success() {
        let code = status
            .code()
            .map_or_else(|| "signal".to_string(), |c| c.to_string());
        let cmd_line = command.join(" ");

        // 检测是否可能是声道数不匹配的错误 / Detect if this might be channel count mismatch error
        let suggestion = if cmd_line.contains("out-ch-config") {
            "\n\n[提示] 如果看到 'Delayed linking failed' 或 'src_N' 相关错误，说明文件的实际声道数与请求的不匹配。\n\
             请尝试使用较小的声道配置，如 --channels 7.1 或 --channels 5.1\n\
             或设置环境变量 MCAT_VERBOSE_GST=1 查看详细错误信息。\n\
             \n\
             [Tip] If you see 'Delayed linking failed' or 'src_N' related errors, the file's actual channel count doesn't match the requested configuration.\n\
             Try using a smaller channel configuration like --channels 7.1 or --channels 5.1,\n\
             or set MCAT_VERBOSE_GST=1 environment variable to see detailed error information."
                .to_string()
        } else {
            String::new()
        };

        return Err(DecodeError::GStreamerFailed(format!(
            "GStreamer 管道失败 (退出码 {code})/Pipeline failed (exit {code}){suggestion}"
        )));
    }

    Ok(())
}
