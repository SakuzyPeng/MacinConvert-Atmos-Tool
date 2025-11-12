use crate::error::{DecodeError, Result};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Eac3,
    TrueHD,
}

pub fn detect_format(file_path: &Path, explicit_format: Option<&str>) -> Result<AudioFormat> {
    // If format is explicitly specified, use it / 如果明确指定了格式，使用它
    if let Some(format_str) = explicit_format {
        return match format_str.to_lowercase().as_str() {
            "eac3" => Ok(AudioFormat::Eac3),
            "truehd" => Ok(AudioFormat::TrueHD),
            _ => Err(DecodeError::FormatDetectionFailed(format!(
                "未知格式/Unknown format: {format_str}"
            ))),
        };
    }

    // Auto-detect from file header / 从文件头自动检测
    let mut file = std::fs::File::open(file_path).map_err(|e| {
        DecodeError::FormatDetectionFailed(format!("无法打开文件/Cannot open file: {e}"))
    })?;

    let mut header = [0u8; 10];
    use std::io::Read;
    file.read_exact(&mut header).map_err(|e| {
        DecodeError::FormatDetectionFailed(format!("无法读取文件头/Cannot read file header: {e}"))
    })?;

    // Check for E-AC3 sync word (0x0B77) / 检查 E-AC3 同步字
    if header[0] == 0x0B && header[1] == 0x77 {
        return Ok(AudioFormat::Eac3);
    }

    // Check for TrueHD sync word (0xF8726FBA) / 检查 TrueHD 同步字
    if header.len() >= 4 && &header[0..4] == b"\xf8\x72\x6f\xba" {
        return Ok(AudioFormat::TrueHD);
    }

    // Check for TrueHD anywhere in first 10 bytes (sometimes offset) / 在前 10 字节中任何地方检查 TrueHD
    for i in 0..=6 {
        if header[i] == 0xF8
            && header[i + 1] == 0x72
            && header[i + 2] == 0x6F
            && header[i + 3] == 0xBA
        {
            return Ok(AudioFormat::TrueHD);
        }
    }

    Err(DecodeError::FormatDetectionFailed(
        "无法检测音频格式，请用 --format 指定/Could not detect audio format. Specify with --format"
            .to_string(),
    ))
}
