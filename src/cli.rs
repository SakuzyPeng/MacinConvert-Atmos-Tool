use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "MacinConvert-Atmos-Tool")]
#[command(about = "将杜比全景声音频转换为多声道 WAV 文件/Convert Dolby Atmos audio to multi-channel WAV files", long_about = None)]
#[command(author = "Sakuzy")]
#[command(version)]
pub struct Args {
    /// 输入文件（E-AC3/TrueHD 格式）/Input file (E-AC3/TrueHD format)
    #[arg(short, long)]
    pub input: PathBuf,

    /// 输出基础路径（可选，默认为输入目录）/Output base path (optional, defaults to input directory)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 输出声道配置（默认：9.1.6）/Output channel configuration (default: 9.1.6)
    #[arg(short, long, default_value = "9.1.6")]
    pub channels: String,

    /// 输入音频格式（如果未指定则自动检测）/Input audio format (auto-detect if not specified)
    #[arg(short, long, value_parser = ["eac3", "truehd"])]
    pub format: Option<String>,

    /// 输出文件名不带声道编号/Don't use numbers in output channel filenames
    #[arg(long)]
    pub no_numbers: bool,

    /// 一次解码一个声道（顺序，节省内存）/Decode one channel at a time (sequential, saves memory)
    #[arg(short, long)]
    pub single: bool,

    /// 将解码的声道合并为单个多声道 WAV 文件/Merge decoded channels into a single multi-channel WAV file
    #[arg(short, long)]
    pub merge: bool,

    /// 合并后删除分离的声道文件/Remove discrete channel files after merging
    #[arg(long)]
    pub cleanup: bool,
}
