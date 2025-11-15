use crate::channels::ChannelConfig;
use crate::error::{DecodeError, Result};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn merge_channels(
    channel_files: &[std::path::PathBuf],
    output_file: &Path,
    config: Option<&ChannelConfig>,
) -> Result<()> {
    // 目前使用 hound 库的简单方案/For now, we'll use a simple approach via hound library
    // 将所有单声道 WAV 合成为多声道 WAV/This will combine all mono WAV files into a multi-channel WAV

    if channel_files.is_empty() {
        return Err(DecodeError::MergeFailed(
            "没有声道文件可合并/No channel files to merge".to_string(),
        ));
    }

    // Verify all files exist / 验证所有文件存在
    for file in channel_files {
        if !file.exists() {
            return Err(DecodeError::MergeFailed(format!(
                "未找到声道文件/Channel file not found: {}",
                file.display()
            )));
        }
    }

    // Read first file to get parameters / 读取第一个文件获取参数
    let first_reader = hound::WavReader::open(&channel_files[0]).map_err(|e| {
        DecodeError::MergeFailed(format!(
            "无法读取第一个 WAV 文件/Cannot read first WAV file: {e}"
        ))
    })?;

    let spec = first_reader.spec();
    let num_frames = first_reader.len() as usize;

    // Verify all files have the same format / 验证所有文件具有相同的格式
    for file in &channel_files[1..] {
        let reader = hound::WavReader::open(file).map_err(|e| {
            DecodeError::MergeFailed(format!("无法读取 WAV 文件/Cannot read WAV file: {e}"))
        })?;

        let file_spec = reader.spec();
        if file_spec.channels != 1 {
            return Err(DecodeError::MergeFailed(format!(
                "期望单声道，但得到 {} 个声道/Expected mono channel, got {} channels",
                file_spec.channels, file_spec.channels
            )));
        }
        if file_spec.sample_rate != spec.sample_rate {
            return Err(DecodeError::MergeFailed(
                "采样率不匹配/Sample rate mismatch".to_string(),
            ));
        }
        if reader.len() as usize != num_frames {
            return Err(DecodeError::MergeFailed(
                "帧数不匹配/Frame count mismatch".to_string(),
            ));
        }
    }

    // Read all channel data / 读取所有声道数据
    let mut all_channels = Vec::new();
    for file in channel_files {
        let reader = hound::WavReader::open(file).map_err(|e| {
            DecodeError::MergeFailed(format!("无法读取 WAV 文件/Cannot read WAV file: {e}"))
        })?;

        #[allow(clippy::cast_precision_loss)]
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader
                .into_samples::<f32>()
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| {
                    DecodeError::MergeFailed(format!("无法读取样本/Cannot read samples: {e}"))
                })?,
            hound::SampleFormat::Int => {
                reader
                    .into_samples::<i32>()
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|e| {
                        DecodeError::MergeFailed(format!("无法读取样本/Cannot read samples: {e}"))
                    })?
                    .into_iter()
                    .map(|s| s as f32 / 2_147_483_648.0) // Convert i32 to f32 / 将 i32 转换为 f32
                    .collect()
            }
        };

        all_channels.push(samples);
    }

    // Create output writer / 创建输出写入器
    let out_spec = hound::WavSpec {
        channels: u16::try_from(channel_files.len()).expect("channels <= u16"),
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(output_file, out_spec).map_err(|e| {
        DecodeError::MergeFailed(format!("无法创建输出 WAV/Cannot create output WAV: {e}"))
    })?;

    // Interleave and write samples / 交错并写入样本
    for frame_idx in 0..num_frames {
        for channel_data in &all_channels {
            if frame_idx < channel_data.len() {
                writer.write_sample(channel_data[frame_idx]).map_err(|e| {
                    DecodeError::MergeFailed(format!("无法写入样本/Cannot write sample: {e}"))
                })?;
            }
        }
    }

    writer.finalize().map_err(|e| {
        DecodeError::MergeFailed(format!("无法最终化 WAV 文件/Cannot finalize WAV file: {e}"))
    })?;

    // 在 WAV 文件备注中写入声道配置信息 / Add channel configuration to WAV file comments
    if let Some(ch_config) = config {
        let channel_list = ch_config
            .names
            .iter()
            .enumerate()
            .map(|(idx, name)| format!("{}: {}", idx + 1, name))
            .collect::<Vec<_>>()
            .join(", ");
        let comment = format!("{} [{}]", ch_config.name, channel_list);
        add_wav_comment(output_file, &comment).map_err(|e| {
            DecodeError::MergeFailed(format!("无法添加 WAV 备注/Failed to add WAV comment: {e}"))
        })?;
    }

    Ok(())
}

/// 在 WAV 文件中添加备注信息 / Add comment to WAV file
/// 将声道配置信息写入 WAV 文件的 LIST chunk 中的 ICOM (comment) 字段
fn add_wav_comment(file_path: &Path, comment: &str) -> std::io::Result<()> {
    let comment_bytes = comment.as_bytes();
    // ICOM 块需要偶数长度的数据（如果奇数则补一个 null byte）
    let padded_len = (comment_bytes.len() + 1) & !1;

    // LIST chunk 的结构：
    // "LIST" (4 bytes) + size (4 bytes) + "INFO" (4 bytes) + "ICOM" (4 bytes) + size (4 bytes) + data
    let list_size = 4 + 4 + 4 + 4 + padded_len; // INFO + ICOM + ICOM_size + comment data

    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Seek 到 RIFF 大小字段（偏移 4）/ Seek to RIFF size field (offset 4)
    file.seek(SeekFrom::Start(4))?;

    // 读取 RIFF 大小 / Read RIFF size
    let mut riff_size_bytes = [0u8; 4];
    file.read_exact(&mut riff_size_bytes)?;
    let mut riff_size = u32::from_le_bytes(riff_size_bytes) as u64;

    // 添加新的 LIST chunk 到文件末尾（在任何现有数据之前）
    file.seek(SeekFrom::End(0))?;

    // 写入 LIST chunk header
    file.write_all(b"LIST")?;
    file.write_all(&(list_size as u32).to_le_bytes())?;
    file.write_all(b"INFO")?;

    // 写入 ICOM subchunk
    file.write_all(b"ICOM")?;
    file.write_all(&(comment_bytes.len() as u32).to_le_bytes())?;
    file.write_all(comment_bytes)?;

    // 如果数据长度为奇数，添加 padding byte
    if !comment_bytes.len().is_multiple_of(2) {
        file.write_all(&[0u8])?;
    }

    // 更新 RIFF 大小
    riff_size += 4 + 4 + 4 + 4 + padded_len as u64;
    file.seek(SeekFrom::Start(4))?;
    file.write_all(&(riff_size as u32).to_le_bytes())?;

    Ok(())
}
