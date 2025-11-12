mod channels;
mod cli;
mod decoder;
mod error;
mod format;
mod merger;
mod tools;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = cli::Args::parse();

    // Validate input file exists / 验证输入文件存在
    if !args.input.exists() {
        return Err("输入文件不存在/Input file not found".into());
    }

    // Locate Dolby tools / 定位 Dolby 工具
    let (gst_launch, gst_plugins) = tools::locate_tools()?;
    println!("找到 GStreamer 工具/Found GStreamer tools");

    // Detect audio format / 检测音频格式
    let audio_format = format::detect_format(&args.input, args.format.as_deref())?;
    println!("检测到格式/Detected format: {audio_format:?}");

    // Get channel configuration / 获取声道配置
    let channel_config = channels::get_config(&args.channels)?;
    println!(
        "使用 {} 声道配置，共 {} 个声道/Using {} channel configuration with {} channels",
        args.channels,
        channel_config.names.len(),
        args.channels,
        channel_config.names.len()
    );

    // Decode audio / 解码音频
    let decoded_files = decoder::decode(
        &args.input,
        args.output.as_ref(),
        &gst_launch,
        &gst_plugins,
        &audio_format,
        &channel_config,
        args.single,
        args.no_numbers,
    )?;
    println!(
        "已解码 {} 个声道文件/Decoded {} channel files",
        decoded_files.len(),
        decoded_files.len()
    );

    // Merge channels if requested / 如果需要合并声道
    if args.merge {
        let merged_file = if let Some(output) = &args.output {
            output.with_extension("wav")
        } else {
            args.input.with_extension("wav")
        };

        merger::merge_channels(&decoded_files, &merged_file)?;
        println!(
            "已将声道合并至 {}/Merged channels to {}",
            merged_file.display(),
            merged_file.display()
        );

        // Cleanup discrete files if requested / 如果需要清理分离的文件
        if args.cleanup {
            for file in &decoded_files {
                std::fs::remove_file(file)?;
                println!("已删除 {}/Removed {}", file.display(), file.display());
            }
        }
    }

    println!("完成!/Done!");
    Ok(())
}
