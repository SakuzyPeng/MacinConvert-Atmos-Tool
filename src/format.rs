use crate::error::{DecodeError, Result};
use std::io::Read;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // 写一个临时文件并返回其句柄 / Write a temp file with given bytes and return the handle
    fn temp_with_bytes(bytes: &[u8]) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(bytes).unwrap();
        f.flush().unwrap();
        f
    }

    // 显式格式（大小写不敏感）映射到对应枚举 / Explicit format (case-insensitive) maps to enum
    #[test]
    fn explicit_format_is_case_insensitive() {
        let dummy = Path::new("/nonexistent");
        for s in ["eac3", "EAC3", "Eac3"] {
            assert_eq!(detect_format(dummy, Some(s)).unwrap(), AudioFormat::Eac3);
        }
        for s in ["truehd", "TrueHD", "TRUEHD"] {
            assert_eq!(detect_format(dummy, Some(s)).unwrap(), AudioFormat::TrueHD);
        }
    }

    // 显式未知格式返回错误 / Unknown explicit format returns error
    #[test]
    fn explicit_unknown_format_errors() {
        let dummy = Path::new("/nonexistent");
        let err = detect_format(dummy, Some("ac4")).unwrap_err();
        assert!(matches!(err, DecodeError::FormatDetectionFailed(_)));
    }

    // E-AC3 同步字 0x0B77 被识别 / E-AC3 sync word detected
    #[test]
    fn detects_eac3_sync_word() {
        let f = temp_with_bytes(&[0x0B, 0x77, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(detect_format(f.path(), None).unwrap(), AudioFormat::Eac3);
    }

    // TrueHD 同步字位于开头 / TrueHD sync word at start
    #[test]
    fn detects_truehd_sync_word_at_start() {
        let f = temp_with_bytes(&[0xF8, 0x72, 0x6F, 0xBA, 0, 0, 0, 0, 0, 0]);
        assert_eq!(detect_format(f.path(), None).unwrap(), AudioFormat::TrueHD);
    }

    // TrueHD 同步字带前缀偏移仍被识别 / TrueHD sync word at an offset still detected
    #[test]
    fn detects_truehd_sync_word_at_offset() {
        let f = temp_with_bytes(&[0x00, 0x00, 0xF8, 0x72, 0x6F, 0xBA, 0, 0, 0, 0]);
        assert_eq!(detect_format(f.path(), None).unwrap(), AudioFormat::TrueHD);
    }

    // 无任何同步字返回错误 / No sync word returns error
    #[test]
    fn unrecognized_header_errors() {
        let f = temp_with_bytes(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]);
        assert!(detect_format(f.path(), None).is_err());
    }

    // 文件头不足 10 字节返回错误 / Header shorter than 10 bytes returns error
    #[test]
    fn short_header_errors() {
        let f = temp_with_bytes(&[0x0B, 0x77]);
        assert!(detect_format(f.path(), None).is_err());
    }
}
