mod channels;
mod cli;
mod decoder;
mod error;
mod flac_converter;
mod format;
mod merger;
mod tools;

use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug)]
struct InputPlan {
    inputs: Vec<PathBuf>,
    merge: bool,
    cleanup: bool,
    channels_str: String,
}

/// Collect candidate audio files in a directory (non-recursive),
/// using header-based detection for E-AC3/TrueHD. Sorted by mtime.
/// 基于文件头在指定目录（不递归）收集候选音频，按修改时间排序。
fn collect_candidates_in_dir(dir: &Path) -> Vec<(PathBuf, SystemTime)> {
    let mut candidates = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Ok(meta) = entry.metadata() {
                    if meta.len() >= 4 && crate::format::detect_format(&p, None).is_ok() {
                        if let Ok(mtime) = meta.modified() {
                            candidates.push((p, mtime));
                        }
                    }
                }
            }
        }
    }
    candidates.sort_by(|a, b| a.1.cmp(&b.1));
    candidates
}

/// Resolve inputs for normal or lazy mode. In lazy mode, scan current dir first,
/// then fallback to the executable's directory when empty. Also enforce merge/cleanup=true.
/// 解析普通/懒人模式输入；懒人模式先扫描当前目录，若为空回退到可执行文件目录；并强制开启合并与清理。
fn resolve_inputs(args: &crate::cli::Args) -> Result<InputPlan, Box<dyn std::error::Error>> {
    let mut merge = args.flags.merge;
    let mut cleanup = args.flags.cleanup;

    // If not lazy and an input is provided, return directly
    if !args.flags.lazy {
        if let Some(inp) = args.input.clone() {
            return Ok(InputPlan {
                inputs: vec![inp],
                merge,
                cleanup,
                channels_str: args.channels.clone(),
            });
        }
    }

    // Lazy mode path
    println!("已启用懒人模式/Lazy mode enabled");
    merge = true;
    cleanup = true;

    // 仅在当前目录收集候选文件（不递归）/Current directory only, non-recursive
    let mut candidates = collect_candidates_in_dir(Path::new("."));

    // 若当前目录没有结果，回退到可执行文件所在目录/If empty, fallback to exe dir
    if candidates.is_empty() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                candidates = collect_candidates_in_dir(exe_dir);
            }
        }
    }

    if candidates.is_empty() {
        eprintln!("[错误] 未找到可用输入文件/No suitable input file found");
        return Err("未找到可用输入文件/No suitable input file found".into());
    }

    let inputs = candidates.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
    println!(
        "将按顺序处理 {} 个文件/Processing {} files sequentially",
        inputs.len(),
        inputs.len()
    );

    // 懒人模式固定为 9.1.6/Lazy mode forces 9.1.6
    Ok(InputPlan {
        inputs,
        merge,
        cleanup,
        channels_str: "9.1.6".to_string(),
    })
}

/// Prepare a batch output directory when multiple inputs are given.
/// 多文件输入时，准备批量输出目录（--output 必须是目录）。
fn prepare_batch_output_dir(
    inputs: &[PathBuf],
    output_opt: Option<&PathBuf>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    if inputs.len() <= 1 {
        return Ok(None);
    }

    if let Some(o) = output_opt {
        if o.exists() {
            if o.is_dir() {
                return Ok(Some(o.clone()));
            }
            return Err(
                "批处理时 --output 必须为目录/--output must be a directory in batch mode".into(),
            );
        }
        std::fs::create_dir_all(o)?;
        return Ok(Some(o.clone()));
    }
    Ok(None)
}

/// Compute per-file output base path considering batch directory.
/// 结合批量目录计算单文件输出基路径。
fn output_base_for(
    input: &Path,
    batch_output_dir: Option<&Path>,
    output_opt: Option<&PathBuf>,
) -> Option<PathBuf> {
    if let Some(dir) = batch_output_dir {
        let stem = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        return Some(dir.join(stem));
    }
    output_opt.cloned()
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = cli::Args::parse();

    // Parse inputs and flags (handles lazy mode) / 解析输入与开关（含懒人模式）
    let plan = resolve_inputs(&args)?;

    // Per-file parallel by default unless --single / 默认保持每文件内并行，除非 --single
    let single = args.flags.single;

    // Concurrency override via CLI / 通过 CLI 覆盖并发度
    if let Some(j) = args.jobs {
        std::env::set_var("MCAT_MAX_PAR", j.to_string());
    }

    // Locate Dolby tools / 定位 Dolby 工具
    let (gst_launch, gst_plugins) = tools::locate_tools(args.dolby_tools.as_deref())?;
    println!("找到 GStreamer 工具/Found GStreamer tools");
    let channel_config = channels::get_config(&plan.channels_str)?;

    // Prepare batch output directory if needed / 如有需要，准备批处理输出目录
    let batch_output_dir = prepare_batch_output_dir(&plan.inputs, args.output.as_ref())?;

    for (idx, input) in plan.inputs.iter().enumerate() {
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
            plan.inputs.len(),
            input.display()
        );

        // Detect audio format / 检测音频格式
        let audio_format = format::detect_format(input, args.format.as_deref())?;
        println!("检测到格式/Detected format: {audio_format:?}");

        // Decode audio / 解码音频（按文件顺序）
        // 批处理时使用输出目录 + 输入基名作为 base；单文件保持原有行为 / in batch, use output dir + input stem as base; single-file keeps original semantics
        let per_file_base =
            output_base_for(input, batch_output_dir.as_deref(), args.output.as_ref());
        let decoded_files = decoder::decode(
            input,
            per_file_base.as_ref(),
            &gst_launch,
            &gst_plugins,
            audio_format,
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
        if plan.merge {
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

            merger::merge_channels(&decoded_files, &merged_file, Some(&channel_config))?;
            println!(
                "已将声道合并至 {}/Merged channels to {}",
                merged_file.display(),
                merged_file.display()
            );

            // Convert to FLAC if requested / 如果需要转码为 FLAC
            if args.flags.flac {
                // 检查声道数限制 / Check channel limit for FLAC
                let num_channels = channel_config.names.len() as u16;
                if let Err(e) = flac_converter::check_flac_compatibility(num_channels) {
                    eprintln!("[警告] FLAC 转码失败/FLAC conversion warning: {e}");
                } else {
                    // 构建 FLAC 输出路径 / Build FLAC output path
                    let flac_file = merged_file.with_extension("flac");

                    // 执行转码 / Perform conversion
                    match flac_converter::convert_batch(
                        &merged_file,
                        &flac_file,
                        Some(&channel_config),
                    ) {
                        Ok(()) => {
                            println!(
                                "FLAC 转码完成/FLAC conversion completed: {}",
                                flac_file.display()
                            );

                            // 删除原始 WAV 文件（如果不保留）/ Delete original WAV (if not keeping)
                            if !args.flags.keep_wav {
                                std::fs::remove_file(&merged_file)?;
                                println!(
                                    "已删除原始 WAV 文件/Removed original WAV: {}",
                                    merged_file.display()
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[错误] FLAC 转码失败/FLAC conversion failed: {e}. 保留原始 WAV 文件/Keeping original WAV."
                            );
                        }
                    }
                }
            }

            // Cleanup discrete files if requested / 如果需要清理分离的文件
            if plan.cleanup {
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
