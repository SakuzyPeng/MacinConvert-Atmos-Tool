mod channels;
mod cli;
mod decoder;
mod error;
mod format;
mod merger;
mod tools;

use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = cli::Args::parse();

    // Lazy mode handling / 懒人模式处理（按文件顺序一个接一个处理，不改变声道内并行策略）
    let single = args.single; // 保持通道内并行 / keep per-file parallel by default
    let mut merge = args.merge;
    let mut cleanup = args.cleanup;

    // Build input list / 构建输入列表
    let mut inputs: Vec<PathBuf> = Vec::new();
    if let Some(inp) = args.input.clone() {
        inputs.push(inp);
    }
    if args.lazy || inputs.is_empty() {
        println!("已启用懒人模式/Lazy mode enabled");
        merge = true;
        cleanup = true;
        // 仅在当前目录收集候选文件（不递归），通过文件头检测 E-AC3/TrueHD /
        // Collect in current directory only (non-recursive) using header detection
        let mut candidates: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
        if let Ok(rd) = std::fs::read_dir(".") {
            for entry in rd.flatten() {
                let p = entry.path();
                if p.is_file() {
                    // quick size check to skip tiny files
                    if let Ok(meta) = entry.metadata() {
                        if meta.len() < 4 {
                            continue;
                        }
                        if format::detect_format(&p, None).is_ok() {
                            if let Ok(mtime) = meta.modified() {
                                candidates.push((p.clone(), mtime));
                            }
                        }
                    }
                }
            }
        }
        candidates.sort_by(|a, b| a.1.cmp(&b.1));
        if candidates.is_empty() {
            eprintln!("[错误] 未找到可用输入文件/No suitable input file found");
            return Err("未找到可用输入文件/No suitable input file found".into());
        }
        inputs = candidates.into_iter().map(|(p, _)| p).collect();
        println!(
            "将按顺序处理 {} 个文件/Processing {} files sequentially",
            inputs.len(),
            inputs.len()
        );
    }

    // Ensure at least one input exists / 至少应有一个输入
    if inputs.is_empty() {
        eprintln!("[错误] 未提供输入/No input provided");
        return Err("未提供输入/No input provided".into());
    }

    // Concurrency override via CLI / 通过 CLI 覆盖并发度
    if let Some(j) = args.jobs {
        std::env::set_var("MCAT_MAX_PAR", j.to_string());
    }

    // Locate Dolby tools / 定位 Dolby 工具
    let (gst_launch, gst_plugins) = tools::locate_tools(args.dolby_tools.as_deref())?;
    println!("找到 GStreamer 工具/Found GStreamer tools");
    // Channel configuration / 声道配置（懒人模式默认 9.1.6）
    let channels_str = if args.lazy {
        "9.1.6".to_string()
    } else {
        args.channels.clone()
    };
    let channel_config = channels::get_config(&channels_str)?;

    // In batch (multiple inputs), treat --output as a directory / 多文件批处理时将 --output 视为目录
    let batch_output_dir: Option<PathBuf> = if inputs.len() > 1 {
        if let Some(ref o) = args.output {
            if o.exists() {
                if o.is_dir() {
                    Some(o.clone())
                } else {
                    return Err(
                        "批处理时 --output 必须为目录/--output must be a directory in batch mode"
                            .into(),
                    );
                }
            } else {
                std::fs::create_dir_all(o)?;
                Some(o.clone())
            }
        } else {
            None
        }
    } else {
        None
    };

    for (idx, input) in inputs.iter().enumerate() {
        if !input.exists() {
            eprintln!(
                "[警告] 跳过不存在的文件/Skip missing file: {}",
                input.display()
            );
            continue;
        }
        println!(
            "[{} / {}] 处理文件/Processing file: {}",
            idx + 1,
            inputs.len(),
            input.display()
        );

        // Detect audio format / 检测音频格式
        let audio_format = format::detect_format(input, args.format.as_deref())?;
        println!("检测到格式/Detected format: {audio_format:?}");

        // Decode audio / 解码音频（按文件顺序）
        // 批处理时使用输出目录 + 输入基名作为 base；单文件保持原有行为 / in batch, use output dir + input stem as base; single-file keeps original semantics
        let per_file_base_buf;
        let output_base = if let Some(dir) = &batch_output_dir {
            let stem = input
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            per_file_base_buf = dir.join(stem);
            Some(&per_file_base_buf)
        } else {
            args.output.as_ref()
        };
        let decoded_files = decoder::decode(
            input,
            output_base,
            &gst_launch,
            &gst_plugins,
            &audio_format,
            &channel_config,
            single,
            args.no_numbers,
        )?;
        println!(
            "已解码 {} 个声道文件/Decoded {} channel files",
            decoded_files.len(),
            decoded_files.len()
        );

        // Merge channels if requested / 如果需要合并声道
        if merge {
            let merged_file = if let Some(dir) = &batch_output_dir {
                let stem = input
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output");
                dir.join(stem).with_extension("wav")
            } else if let Some(output) = &args.output {
                output.with_extension("wav")
            } else {
                input.with_extension("wav")
            };

            merger::merge_channels(&decoded_files, &merged_file)?;
            println!(
                "已将声道合并至 {}/Merged channels to {}",
                merged_file.display(),
                merged_file.display()
            );

            // Cleanup discrete files if requested / 如果需要清理分离的文件
            if cleanup {
                for file in &decoded_files {
                    std::fs::remove_file(file)?;
                    println!("已删除 {}/Removed {}", file.display(), file.display());
                }
            }
        }
    }

    println!("完成!/Done!");
    Ok(())
}
