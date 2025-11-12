# MacinConvert-Atmos-Tool

A Rust command-line tool for converting Dolby Atmos audio files to multi-channel WAV files on macOS.

一个 Rust 命令行工具，用于在 macOS 上将杜比全景声（Dolby Atmos）音频文件转换为多声道 WAV 文件。

## Features / 功能

- Auto-detect E-AC3 and TrueHD audio formats / 自动检测 E-AC3 和 TrueHD 音频格式
- Support 13 different channel configurations from 2.0 stereo to 9.1.6 Dolby Atmos / 支持 13 种不同的声道配置，从 2.0 立体声到 9.1.6 杜比全景声
- Parallel decoding for multiple channels with improved speed / 并行解码多声道，提高处理速度
- Sequential decoding mode to save memory / 顺序解码模式，节省内存
- Optional channel merging to combine mono files into multi-channel WAV / 可选的声道合并功能，将分离的单声道文件合并为多声道 WAV
- Automatic cleanup of intermediate files / 自动清理中间文件
- Support for local Dolby tools or system-installed Dolby Reference Player / 支持本地 Dolby 工具或系统安装的 Dolby Reference Player

## System Requirements / 系统要求

- macOS 10.13 or higher / macOS 10.13 或更高版本
- Rust 1.70 or higher for building / Rust 1.70 或更高版本（用于构建）
- GStreamer and Dolby-related plugins and libraries / GStreamer 及 Dolby 相关插件和库

## Installation / 安装

### Building / 构建

```bash
cargo build --release
```

Output binary is located at / 输出二进制文件位于 `target/release/MacinConvert-Atmos-Tool`

### Dependencies and Tool Configuration / 依赖项和工具配置

The tool requires the following GStreamer related components / 该工具需要以下 GStreamer 相关组件：
- gst-launch-1.0 - GStreamer command-line tool / GStreamer 命令行工具
- dlbac3parse - E-AC3 format parser / E-AC3 格式解析器
- dlbtruehdparse - TrueHD format parser / TrueHD 格式解析器
- dlbaudiodecbin - Dolby audio decoder / Dolby 音频解码器
- Related GStreamer plugin libraries / 相关的 GStreamer 插件库

#### Getting GStreamer Tools / 获取 GStreamer 工具

There are two ways to obtain the required GStreamer tools / 有两种方式获取所需的 GStreamer 工具：

**Method 1: System Installation / 方式 1：系统安装**

If Dolby Reference Player is already installed on your system, the tool will automatically search for it at the following location / 如果系统中已安装 Dolby Reference Player，工具会自动在以下位置查找：

```
/Applications/Dolby/Dolby Reference Player.app/Contents
```

Install from / 从以下位置安装：https://professional.dolby.com/product/media-processing-and-delivery/drp---dolby-reference-player/

**Method 2: Local dolby-tools Directory / 方式 2：本地 dolby-tools 目录**

Create a dolby-tools directory in the project root with the following structure / 在项目根目录创建 dolby-tools 目录，包含以下结构：

```
dolby-tools/
├── gstreamer/
│   └── bin/
│       └── gst-launch-1.0                    # GStreamer main program / GStreamer 主程序
├── gst-plugins/                               # GStreamer plugins directory / GStreamer 插件目录
│   ├── libdlbac3parse.so                     # E-AC3 parsing plugin / E-AC3 解析插件
│   ├── libdlbtruehdparse.so                  # TrueHD parsing plugin / TrueHD 解析插件
│   ├── libdlbaudiodecbin.so                  # Dolby audio decoder / Dolby 音频解码器
│   └── [other GStreamer plugins] / [其他 GStreamer 插件]
└── gst-plugins-libs/                         # GStreamer plugin dependencies / GStreamer 插件依赖库
    ├── libdlb*.dylib                         # Dolby library files / Dolby 库文件
    ├── libgst*.dylib                         # GStreamer library files / GStreamer 库文件
    └── [other dependency libraries] / [其他依赖库]
```

The tool will prioritize the local dolby-tools directory, and fall back to system-installed Dolby Reference Player if not found / 工具会优先使用本地 dolby-tools 目录，如果不存在则自动回退到系统安装的 Dolby Reference Player。

#### Environment Overrides / 环境变量覆盖

You can override tool locations via environment variables / 可通过环境变量覆盖工具查找路径：

- `MCAT_GST_LAUNCH`: absolute path to `gst-launch-1.0` / `gst-launch-1.0` 的绝对路径
- `MCAT_GST_PLUGINS`: path to GStreamer plugins dir / GStreamer 插件目录路径
- `MCAT_DOLBY_TOOLS`: base dir containing `gstreamer/bin` and `gst-plugins` / 包含 `gstreamer/bin` 与 `gst-plugins` 的基目录

Lookup order / 查找顺序：
1) `MCAT_GST_LAUNCH` + `MCAT_GST_PLUGINS`
2) `MCAT_DOLBY_TOOLS`
3) `./dolby-tools`
4) Dolby Reference Player app bundle

#### Obtaining GStreamer Components / 获取 GStreamer 组件

GStreamer and related plugins can be obtained from the following sources / GStreamer 及相关插件可以从以下来源获取：
- System package manager (brew install gstreamer) / 系统包管理器（brew install gstreamer）
- Official GStreamer binary releases / 官方 GStreamer 二进制发行版
- Other applications that include GStreamer / 包含 GStreamer 的其他应用程序

Place the required binary files and libraries in the above directory structure / 将所需的二进制文件和库放在上述目录结构中即可。

## Usage / 使用方法

### Basic Usage / 基本用法

```bash
./MacinConvert-Atmos-Tool --input file.eac3
```

### Specifying Channel Configuration / 指定声道配置

Default is 9.1.6 (16 channels). Other available configurations / 默认为 9.1.6（16 个声道）。其他可用配置：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1
```

Supported channel configurations / 支持的声道配置：
- 2.0 - Stereo / 立体声
- 3.1 - Left, Right, Center, LFE / 左、右、中、低频
- 5.1 - Standard surround / 标准环绕声
- 7.1 - Extended surround / 扩展环绕声
- 9.1 - Nine channel / 九声道
- 5.1.2, 5.1.4 - Dolby Atmos (5.1 base) / 杜比全景声（5.1 基础）
- 7.1.2, 7.1.4, 7.1.6 - Dolby Atmos (7.1 base) / 杜比全景声（7.1 基础）
- 9.1.2, 9.1.4, 9.1.6 - Dolby Atmos (9.1 base) / 杜比全景声（9.1 基础）

### Specifying Output Location / 指定输出位置

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --output ~/Movies/decoded
```

If not specified, output files will be in the same directory as the input file / 如果未指定，输出文件将与输入文件在同一目录。

### Specifying Audio Format / 指定音频格式

The program auto-detects the format, but you can also specify it explicitly / 程序会自动检测格式，但也可以显式指定：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --format eac3
./MacinConvert-Atmos-Tool --input file.thd --format truehd
```

### Sequential Decoding (Memory-Efficient) / 顺序解码（节省内存）

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --single
```

Parallel decoding is faster but uses more memory. Sequential decoding processes one channel at a time, saving memory / 并行解码更快但消耗更多内存。顺序解码逐个处理每个声道，更节省内存。

### Merging Channels / 合并声道

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge
```

This option merges all separated mono WAV files into a single multi-channel WAV file / 此选项将所有分离的单声道 WAV 文件合并为单个多声道 WAV 文件。

### Cleanup After Merging / 清理中间文件

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge --cleanup
```

Automatically delete separated mono files after merging / 合并后自动删除分离的单声道文件。

### Output Filename Format / 输出文件名格式

Default format / 默认格式：`input.01_L.wav`, `input.02_R.wav`, ...

Output without numbers / 不带编号的输出格式：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --no-numbers
```

Output format / 输出格式：`input.L.wav`, `input.R.wav`, ...

### Complete Example / 完整示例

```bash
./MacinConvert-Atmos-Tool \
  --input /path/to/source_file.mp4 \
  --output ~/Movies/decoded \
  --channels 9.1.6 \
  --merge \
  --cleanup
```

This command will / 此命令将：
1. Detect the input file format / 检测输入文件格式
2. Get the 9.1.6 channel configuration (16 channels) / 获取 9.1.6 声道配置（16 个声道）
3. Decode all channels in parallel / 并行解码所有声道
4. Merge into a single multi-channel WAV file / 合并为单个多声道 WAV 文件
5. Delete intermediate mono files / 删除中间的单声道文件

## Command-Line Arguments / 命令行参数

```
Usage: MacinConvert-Atmos-Tool [OPTIONS]

Options:
  -i, --input <INPUT>
      --dolby-tools <PATH>
          指定 dolby-tools 基目录（包含 gstreamer/bin 与 gst-plugins）/Specify dolby-tools base directory (contains gstreamer/bin and gst-plugins)
  -j, --jobs <JOBS>
          并行作业数（覆盖默认与环境变量 MCAT_MAX_PAR）/Parallel jobs (overrides default and env MCAT_MAX_PAR)
      Input audio file (E-AC3/TrueHD format) / 输入音频文件（E-AC3/TrueHD 格式）

  -o, --output <OUTPUT>
      Output file base path (optional, defaults to input directory) / 输出文件基础路径（可选，默认为输入目录）

  -c, --channels <CHANNELS>
      Output channel configuration (default: 9.1.6) / 输出声道配置（默认：9.1.6）

  -f, --format <FORMAT>
      Input audio format (eac3/truehd, optional, auto-detect by default) / 输入音频格式（eac3/truehd，可选，默认自动检测）

  --no-numbers
      Output filenames without channel numbers / 输出文件名不带声道编号

  -s, --single
      Sequential decoding of individual channels (saves memory) / 顺序解码单个声道（节省内存）

  -m, --merge
      Merge decoded channels into a single multi-channel WAV file / 合并解码的声道为单个多声道 WAV 文件

  --cleanup
      Remove separated mono files after merging / 合并后删除分离的单声道文件

  -h, --help
      Show help information / 显示帮助信息

  -V, --version
      Show version information / 显示版本信息
```

## Output Format / 输出格式

### Mono Files / 单声道文件

Format / 格式：`input.01_L.wav`, `input.02_R.wav`, ...

Specifications / 规格：
- Sample format: Float32 / 采样格式：Float32
- Sample rate: 48000 Hz (same as source) / 采样率：48000 Hz（与源文件相同）
- Number of channels: 1 / 声道数：1

### Merged Multi-Channel File / 合并后的多声道文件

Format / 格式：`input.wav`

Specifications / 规格：
- Sample format: Float32 / 采样格式：Float32
- Sample rate: 48000 Hz (same as source) / 采样率：48000 Hz（与源文件相同）
- Number of channels: Based on configuration (2-16 channels) / 声道数：根据配置（2-16 个声道）
- Channel order: Following ITU-R BS.2051 standard / 声道顺序：按 ITU-R BS.2051 标准排列

## Logging / 日志

Control logging level with RUST_LOG environment variable / 使用 RUST_LOG 环境变量控制日志级别：

```bash
RUST_LOG=debug ./MacinConvert-Atmos-Tool --input file.eac3
```

Supported levels / 支持的级别：error, warn, info, debug, trace

## Lazy Mode / 懒人模式

无需参数即可双击或运行二进制 / Double‑click or run with no args，程序会 / it will:
- 仅在当前目录收集原始杜比音频并按时间顺序逐个处理（扩展名：`ec3/eac3/thd/truehd`）/ Collect raw Dolby sources in the current directory only and process one file at a time (extensions: `ec3/eac3/thd/truehd`).
- 每个文件内部使用默认并发解码（默认 4，可用 `-j/--jobs` 或 `MCAT_MAX_PAR` 调整），按 9.1.6 配置自动 `--merge --cleanup` / For each file, decode with default parallelism (4 by default; tune via `-j/--jobs` or `MCAT_MAX_PAR`) and auto `--merge --cleanup` with 9.1.6.
- 在批处理模式下将 `--output` 视为输出目录（若不存在自动创建），每个输出以输入基名命名 / In batch mode, `--output` is treated as an output directory (auto‑created), each output named after the input stem.

命令等价示例 / Equivalent command:
```bash
./MacinConvert-Atmos-Tool --lazy
```

## FAQ / 常见问题

### Dolby Tools Not Found / 找不到 Dolby 工具

可以通过以下方式指定工具位置 / You can point the tool location via:
- `--dolby-tools <PATH>`：基目录需包含 `gstreamer/bin/gst-launch-1.0` 与 `gst-plugins` / Base dir must contain `gstreamer/bin/gst-launch-1.0` and `gst-plugins`.
- 环境变量：`MCAT_GST_LAUNCH` 与 `MCAT_GST_PLUGINS`，或 `MCAT_DOLBY_TOOLS` 基目录 / Env vars: `MCAT_GST_LAUNCH` + `MCAT_GST_PLUGINS`, or base dir `MCAT_DOLBY_TOOLS`.
- 若未指定，将依次查找 `./dolby-tools` 与系统 Dolby Reference Player 应用包 / If not set, it tries `./dolby-tools` then the system Dolby Reference Player app bundle.

### Decoding is Slow / 解码速度慢

- Using `--single` option for sequential decoding may be slower / 使用 `--single` 选项进行顺序解码可能会更慢
- Parallel decoding is faster but uses more memory / 并行解码更快但消耗更多内存
- Decoding speed mainly depends on GStreamer plugin performance / 解码速度主要取决于 GStreamer 插件的性能

### Out of Memory / 内存不足

Use the `--single` option for sequential decoding to process one channel at a time / 使用 `--single` 选项进行顺序解码，一次只解码一个声道。

### Corrupted Output Files / 输出文件损坏

Make sure the output directory has enough disk space. A typical 9.1.6 decode output is 15-20 times the size of the source file / 确保输出目录有足够的磁盘空间。一个典型的 9.1.6 解码输出约为源文件的 15-20 倍。

## Development / 开发

### Building Debug Version / 构建调试版本

```bash
cargo build
```

### Running Checks / 运行检查

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Testing / 测试

```bash
cargo run -- --input audio/sample_input.ec3 --channels 5.1
```

## Project Structure / 项目结构

```
src/
  main.rs         - Program entry point and main workflow / 程序入口和主工作流
  cli.rs          - Command-line argument parsing / 命令行参数解析
  decoder.rs      - GStreamer decoding logic / GStreamer 解码逻辑
  merger.rs       - Channel merging logic / 声道合并逻辑
  channels.rs     - Channel configuration definitions / 声道配置定义
  tools.rs        - Dolby tool locating / Dolby 工具定位
  format.rs       - Audio format detection / 音频格式检测
  error.rs        - Error type definitions / 错误类型定义
```



## License / 许可证

MIT License

## Author / 作者

Sakuzy

## Acknowledgments / 致谢

- Claude 4.5 Haiku (Anthropic)：负责本项目的代码编写 / Led the code implementation for this project.
- OpenAI GPT-5 与 Codex 系列：提供审查与质量保障 / Provided code review and quality assurance.

## Disclaimer / 免责声明

- For research and educational use only — verify local laws and licenses before decoding proprietary formats.
  本项目仅供研究与教学使用——在解码专有格式之前，请先确认当地法律与许可条款。
- No Dolby binaries are bundled; you must supply your own tools and ensure you have rights to use them.
  项目不内置任何 Dolby 二进制文件；请自备工具并确保拥有合法使用权。
- We are not affiliated with Dolby; all trademarks belong to their respective owners.
  本项目与 Dolby 无任何从属或合作关系；相关商标归其各自权利人所有。

## Changelog / 更新日志

### 0.1.0

Initial release with support for / 初始版本，支持：
- E-AC3 and TrueHD format detection / E-AC3 和 TrueHD 格式检测
- 13 channel configurations / 13 种声道配置
- Parallel and sequential decoding / 并行和顺序解码
- Channel merging and automatic cleanup / 声道合并和自动清理
