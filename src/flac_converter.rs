use crate::channels::ChannelConfig;
use crate::error::{DecodeError, Result};
use std::path::Path;
use std::process::Command;

/// 验证是否可以转码为 FLAC / Verify if conversion to FLAC is possible
/// FLAC 限制：最多 8 个声道 / FLAC limitation: max 8 channels
pub fn check_flac_compatibility(channels: u16) -> Result<()> {
    if channels > 8 {
        return Err(DecodeError::FlacConversionFailed(format!(
            "FLAC 不支持 {channels} 个声道，最多 8 个 / FLAC does not support {channels} channels, max 8"
        )));
    }
    Ok(())
}

/// 检查系统是否安装了 flac 命令行工具 / Check if flac CLI tool is available
fn check_flac_command() -> Result<()> {
    match Command::new("flac").arg("--version").output() {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(DecodeError::FlacConversionFailed(
            "未找到 flac 命令行工具，请安装 FLAC / flac command not found, please install FLAC"
                .to_string(),
        )),
    }
}

/// 将 32-bit Float WAV 转换为 24-bit Integer WAV
/// Convert 32-bit Float WAV to 24-bit Integer WAV
fn convert_to_24bit_wav(input_path: &Path, output_path: &Path) -> Result<()> {
    let reader = hound::WavReader::open(input_path).map_err(|e| {
        DecodeError::FlacConversionFailed(format!("无法读取 WAV 文件 / Cannot read WAV file: {e}"))
    })?;

    let spec = reader.spec();

    // 创建输出规格（24-bit PCM）/ Create output spec (24-bit PCM)
    let output_spec = hound::WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, output_spec).map_err(|e| {
        DecodeError::FlacConversionFailed(format!(
            "无法创建输出 WAV 文件 / Cannot create output WAV file: {e}"
        ))
    })?;

    // 读取 32-bit Float 样本并转换为 24-bit Integer
    // Read 32-bit Float samples and convert to 24-bit Integer
    let samples: Vec<f32> = reader
        .into_samples::<f32>()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| {
            DecodeError::FlacConversionFailed(format!(
                "读取 WAV 样本失败 / Failed to read WAV samples: {e}"
            ))
        })?;

    for sample in samples {
        // 限制在 [-1.0, 1.0] 范围内 / Clamp to [-1.0, 1.0]
        let clamped = sample.clamp(-1.0, 1.0);
        // 转换为 24-bit（用 32-bit 整数表示）/ Convert to 24-bit (represented as 32-bit int)
        // 范围：-8388608 到 8388607 / Range: -8388608 to 8388607
        let sample_i32 = (clamped * 8388607.0) as i32;

        writer.write_sample(sample_i32).map_err(|e| {
            DecodeError::FlacConversionFailed(format!(
                "写入 WAV 样本失败 / Failed to write WAV sample: {e}"
            ))
        })?;
    }

    writer.finalize().map_err(|e| {
        DecodeError::FlacConversionFailed(format!(
            "完成 WAV 写入失败 / Failed to finalize WAV writing: {e}"
        ))
    })?;

    Ok(())
}

/// 批量转码 WAV 文件为 FLAC（带声道配置） / Batch convert WAV files to FLAC with channel config
pub fn convert_batch(
    wav_path: &Path,
    flac_path: &Path,
    channel_config: Option<&ChannelConfig>,
) -> Result<()> {
    log::info!(
        "开始转码为 FLAC / Starting FLAC conversion: {} → {}",
        wav_path.display(),
        flac_path.display()
    );

    wav_to_flac_with_config(wav_path, flac_path, channel_config)?;

    log::info!(
        "FLAC 转码完成 / FLAC conversion completed: {}",
        flac_path.display()
    );
    Ok(())
}

/// 将 WAV 文件转码为 FLAC，带声道配置信息 / Convert WAV to FLAC with channel configuration
fn wav_to_flac_with_config(
    wav_path: &Path,
    flac_path: &Path,
    channel_config: Option<&ChannelConfig>,
) -> Result<()> {
    // 打开 WAV 文件验证格式 / Open WAV file to verify format
    let wav_reader = hound::WavReader::open(wav_path).map_err(|e| {
        DecodeError::FlacConversionFailed(format!(
            "无法打开 WAV 文件 / Cannot open WAV file: {}: {e}",
            wav_path.display()
        ))
    })?;

    let wav_spec = wav_reader.spec();

    // 验证 WAV 格式 / Verify WAV format
    if wav_spec.sample_rate != 48000 {
        return Err(DecodeError::FlacConversionFailed(format!(
            "只支持 48kHz 采样率，但 WAV 是 {} Hz / Only 48kHz supported, but WAV is {} Hz",
            wav_spec.sample_rate, wav_spec.sample_rate
        )));
    }

    if wav_spec.bits_per_sample != 32 {
        return Err(DecodeError::FlacConversionFailed(format!(
            "只支持 32-bit 采样位深，但 WAV 是 {} bit / Only 32-bit supported, but WAV is {} bit",
            wav_spec.bits_per_sample, wav_spec.bits_per_sample
        )));
    }

    // 检查声道数限制 / Check channel limit
    check_flac_compatibility(wav_spec.channels)?;

    // 检查 flac 命令是否可用 / Check if flac command is available
    check_flac_command()?;

    // 先将 32-bit Float WAV 转换为 24-bit Integer WAV
    // Convert 32-bit Float WAV to 24-bit Integer WAV first
    let temp_wav_path = wav_path.with_extension("temp.wav");

    convert_to_24bit_wav(wav_path, &temp_wav_path)?;

    // 使用 flac 命令行工具进行转码，最大压缩率，并添加声道布局元数据
    // Use flac CLI with maximum compression and add channel layout metadata
    let mut cmd = Command::new("flac");
    cmd.arg("-8") // 最大压缩率 / Maximum compression
        .arg("--silent"); // 静默输出 / Silent output

    // 添加实际的声道布局名称（如果可用）/ Add actual channel layout names if available
    if let Some(config) = channel_config {
        if config.name != "auto" && !config.names.is_empty() {
            // 记录实际的声道布局，例如 "L R C LFE Ls Rs"（源自杜比）
            // Record actual channel layout, e.g. "L R C LFE Ls Rs" (sourced from Dolby)
            let layout_str = config.names.join(" ");
            cmd.arg("--tag")
                .arg(format!("CHANNEL_LAYOUT={layout_str} (Sourced from Dolby)"));
        }
    }

    cmd.arg("--tag")
        .arg("COMMENT=Converted by MacinConvert-Atmos-Tool")
        .arg("-o")
        .arg(flac_path)
        .arg(&temp_wav_path);

    let output = cmd.output().map_err(|e| {
        DecodeError::FlacConversionFailed(format!(
            "执行 flac 命令失败 / Failed to execute flac command: {e}"
        ))
    })?;

    // 清理临时 WAV 文件 / Clean up temporary WAV file
    let _ = std::fs::remove_file(&temp_wav_path);

    if !output.status.success() {
        return Err(DecodeError::FlacConversionFailed(format!(
            "FLAC 编码失败/FLAC encoding failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 写一个 F32 WAV / Write an F32 WAV file
    fn write_f32_wav(path: &Path, samples: &[f32], sample_rate: u32, channels: u16) {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut w = hound::WavWriter::create(path, spec).unwrap();
        for s in samples {
            w.write_sample(*s).unwrap();
        }
        w.finalize().unwrap();
    }

    // 声道数限制：<=8 通过，>8 报错 / Channel limit: <=8 ok, >8 errors
    #[test]
    fn flac_compatibility_channel_limit() {
        assert!(check_flac_compatibility(1).is_ok());
        assert!(check_flac_compatibility(8).is_ok());
        assert!(check_flac_compatibility(9).is_err());
        assert!(check_flac_compatibility(16).is_err());
    }

    // 32-bit Float → 24-bit Int 转换：clamp、位深、样本数 / Conversion: clamp, bit depth, count
    #[test]
    fn convert_to_24bit_clamps_and_sets_format() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src.wav");
        let dst = dir.path().join("dst.wav");
        // 含越界样本 / includes out-of-range samples
        write_f32_wav(&src, &[0.0, 0.5, 1.5, -2.0], 48000, 1);

        convert_to_24bit_wav(&src, &dst).unwrap();

        let reader = hound::WavReader::open(&dst).unwrap();
        assert_eq!(reader.spec().bits_per_sample, 24);
        assert_eq!(reader.spec().sample_format, hound::SampleFormat::Int);
        let samples: Vec<i32> = reader.into_samples::<i32>().map(|s| s.unwrap()).collect();
        assert_eq!(samples.len(), 4);
        assert_eq!(samples[0], 0);
        assert_eq!(samples[1], (0.5_f32 * 8_388_607.0) as i32);
        assert_eq!(samples[2], 8_388_607); // 1.5 clamp 到 1.0 / clamped to 1.0
        assert_eq!(samples[3], -8_388_607); // -2.0 clamp 到 -1.0 / clamped to -1.0
    }

    // 前置校验：非 48kHz 采样率报错（不触达外部 flac）/ Pre-check: non-48k rate errors
    #[test]
    fn wav_to_flac_rejects_non_48k() {
        let dir = tempfile::tempdir().unwrap();
        let wav = dir.path().join("a.wav");
        let flac = dir.path().join("a.flac");
        write_f32_wav(&wav, &[0.0, 0.1], 44100, 1);
        assert!(wav_to_flac_with_config(&wav, &flac, None).is_err());
    }

    // 前置校验：非 32-bit 位深报错 / Pre-check: non-32-bit depth errors
    #[test]
    fn wav_to_flac_rejects_non_32bit() {
        let dir = tempfile::tempdir().unwrap();
        let wav = dir.path().join("a.wav");
        let flac = dir.path().join("a.flac");
        // 写 16-bit Int WAV，采样率合法以确保命中位深校验 / 16-bit Int, valid rate to reach depth check
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 48000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut w = hound::WavWriter::create(&wav, spec).unwrap();
        w.write_sample(0_i16).unwrap();
        w.write_sample(1_i16).unwrap();
        w.finalize().unwrap();
        assert!(wav_to_flac_with_config(&wav, &flac, None).is_err());
    }

    // 前置校验：声道数 >8 报错（在调用 flac 之前）/ Pre-check: >8 channels errors before flac
    #[test]
    fn wav_to_flac_rejects_too_many_channels() {
        let dir = tempfile::tempdir().unwrap();
        let wav = dir.path().join("a.wav");
        let flac = dir.path().join("a.flac");
        // 9 声道、2 帧、48kHz、32-bit / 9ch, 2 frames, 48kHz, 32-bit
        write_f32_wav(&wav, &[0.0; 18], 48000, 9);
        assert!(wav_to_flac_with_config(&wav, &flac, None).is_err());
    }
}
