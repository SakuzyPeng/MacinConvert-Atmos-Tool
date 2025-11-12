use crate::channels::ChannelConfig;
use crate::error::{DecodeError, Result};
use crate::format::AudioFormat;
use rayon::{prelude::*, ThreadPoolBuilder};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
    let mut decoded_files = Vec::new();
    let mut handles = Vec::new();

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
            execute_command(&command)?;
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
                    execute_command(command)
                })
                .collect::<Result<()>>()
        })?;
    }

    Ok(decoded_files)
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

    cmd.extend(vec![
        "dlbaudiodecbin".to_string(),
        format!("out-ch-config={out_ch_config}"),
    ]);

    if matches!(format, AudioFormat::TrueHD) {
        cmd.push("truehddec-presentation=16".to_string());
    }

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

fn execute_command(command: &[String]) -> Result<()> {
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

    // 丢弃子进程输出，避免在高并发下因管道缓冲阻塞 / Drop child stdio to avoid pipe blocking under high concurrency
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    let status = cmd.status().map_err(|e| {
        DecodeError::GStreamerFailed(format!(
            "无法执行 gst-launch/Failed to execute gst-launch: {e}"
        ))
    })?;

    if !status.success() {
        return Err(DecodeError::GStreamerFailed(
            "GStreamer 管道失败/GStreamer pipeline failed".to_string(),
        ));
    }

    Ok(())
}
