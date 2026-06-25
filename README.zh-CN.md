# MacinConvert-Atmos-Tool

[English](README.md) | [中文](README.zh-CN.md)

一个 Rust 命令行工具，用于在 macOS 上将杜比全景声（Dolby Atmos）音频文件转换为多声道 WAV 文件。

## 功能

- 自动检测 E-AC3 和 TrueHD 音频格式
- 支持 13 种不同的声道配置，从 2.0 立体声到 9.1.6 杜比全景声
- 并行解码多声道，提高处理速度
- 顺序解码模式，节省内存
- 可选的声道合并功能，将分离的单声道文件合并为多声道 WAV
- 自动清理中间文件
- 支持本地 Dolby 工具或系统安装的 Dolby Reference Player

## 系统要求

- macOS 10.13 或更高版本
- Rust 1.70 或更高版本（用于构建）
- GStreamer 及 Dolby 相关插件和库

## 安装

### 构建

```bash
cargo build --release
```

输出二进制文件位于 `target/release/MacinConvert-Atmos-Tool`。

### 依赖项和工具配置

该工具需要以下 GStreamer 相关组件：

- gst-launch-1.0 - GStreamer 命令行工具
- dlbac3parse - E-AC3 格式解析器
- dlbtruehdparse - TrueHD 格式解析器
- dlbaudiodecbin - Dolby 音频解码器
- 相关的 GStreamer 插件库

#### 获取 GStreamer 工具

有两种方式获取所需的 GStreamer 工具：

**方式 1：系统安装**

如果系统中已安装 Dolby Reference Player，工具会自动在以下位置查找：

```
/Applications/Dolby/Dolby Reference Player.app/Contents
```

从以下位置安装：https://professional.dolby.com/product/media-processing-and-delivery/drp---dolby-reference-player/

**方式 2：本地 dolby-tools 目录（可执行文件同目录）**

将 `dolby-tools/` 放在可执行文件同目录，结构如下：

```
<exe_dir>/dolby-tools/
├── gstreamer/
│   └── bin/
│       └── gst-launch-1.0                    # GStreamer 主程序
├── gst-plugins/                               # GStreamer 插件目录
│   ├── libdlbac3parse.so                     # E-AC3 解析插件
│   ├── libdlbtruehdparse.so                  # TrueHD 解析插件
│   ├── libdlbaudiodecbin.so                  # Dolby 音频解码器
│   └── [其他 GStreamer 插件]
└── gst-plugins-libs/                         # GStreamer 插件依赖库
    ├── libdlb*.dylib                         # Dolby 库文件
    ├── libgst*.dylib                         # GStreamer 库文件
    └── [其他依赖库]
```

在未通过 CLI/环境变量指定时，工具先查找可执行文件同目录的 `dolby-tools/`，其后回退到系统 DRP。

#### 环境变量覆盖

可通过环境变量覆盖工具查找路径：

- `MCAT_GST_LAUNCH`：`gst-launch-1.0` 的绝对路径
- `MCAT_GST_PLUGINS`：GStreamer 插件目录路径
- `MCAT_DOLBY_TOOLS`：包含 `gstreamer/bin` 与 `gst-plugins` 的基目录

查找顺序：

1. `MCAT_GST_LAUNCH` + `MCAT_GST_PLUGINS`
2. `MCAT_DOLBY_TOOLS`
3. `<exe_dir>/dolby-tools`（可执行文件同目录）
4. Dolby Reference Player app bundle

#### 获取 GStreamer 组件

GStreamer 及相关插件可以从以下来源获取：

- 系统包管理器（`brew install gstreamer`）
- 官方 GStreamer 二进制发行版
- 包含 GStreamer 的其他应用程序

将所需的二进制文件和库放在上述目录结构中即可。

## 使用方法

### 基本用法

```bash
./MacinConvert-Atmos-Tool --input file.eac3
```

### 指定声道配置

默认为 9.1.6（16 个声道）。其他可用配置：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1
```

支持的声道配置：

- 2.0 - 立体声
- 3.1 - 左、右、中、低频
- 5.1 - 标准环绕声
- 7.1 - 扩展环绕声
- 9.1 - 九声道
- 5.1.2、5.1.4 - 杜比全景声（5.1 基础）
- 7.1.2、7.1.4、7.1.6 - 杜比全景声（7.1 基础）
- 9.1.2、9.1.4、9.1.6 - 杜比全景声（9.1 基础）

### 指定输出位置

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --output ~/Movies/decoded
```

如果未指定，输出文件将与输入文件在同一目录。

### 指定音频格式

程序会自动检测格式，但也可以显式指定：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --format eac3
./MacinConvert-Atmos-Tool --input file.thd --format truehd
```

### 顺序解码（节省内存）

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --single
```

并行解码更快但消耗更多内存。顺序解码逐个处理每个声道，更节省内存。

### 合并声道

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge
```

此选项将所有分离的单声道 WAV 文件合并为单个多声道 WAV 文件。

### 清理中间文件

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge --cleanup
```

合并后自动删除分离的单声道文件。

### 输出文件名格式

默认格式：`input.01_L.wav`、`input.02_R.wav`、……

不带编号的输出格式：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --no-numbers
```

输出格式：`input.L.wav`、`input.R.wav`、……

### 完整示例

```bash
./MacinConvert-Atmos-Tool \
  --input /path/to/source_file.mp4 \
  --output ~/Movies/decoded \
  --channels 9.1.6 \
  --merge \
  --cleanup
```

此命令将：

1. 检测输入文件格式
2. 获取 9.1.6 声道配置（16 个声道）
3. 并行解码所有声道
4. 合并为单个多声道 WAV 文件
5. 删除中间的单声道文件

## 命令行参数

```
用法: MacinConvert-Atmos-Tool [选项]

选项:
  -i, --input <INPUT>
          输入音频文件（E-AC3/TrueHD 格式；懒人模式可省略）
  -o, --output <OUTPUT>
          输出文件基础路径（可选，默认为输入目录）
  -c, --channels <CHANNELS>
          输出声道配置（默认：9.1.6）
  -f, --format <FORMAT>
          输入音频格式（eac3/truehd，可选，默认自动检测）
      --dolby-tools <PATH>
          指定 dolby-tools 基目录（包含 gstreamer/bin 与 gst-plugins）
  -j, --jobs <JOBS>
          并行作业数（覆盖默认与环境变量 MCAT_MAX_PAR）
      --no-numbers
          输出文件名不带声道编号
  -s, --single
          顺序解码单个声道（节省内存）
  -m, --merge
          合并解码的声道为单个多声道 WAV 文件
      --cleanup
          合并后删除分离的单声道文件
      --lazy
          懒人模式：自动按文件顺序处理并合并清理
      --flac
          将合并的 WAV 转码为 FLAC 格式（最大压缩）
      --keep-wav
          FLAC 转码后保留原始合并的 WAV 文件
  -h, --help
          显示帮助信息
  -V, --version
          显示版本信息
```

### FLAC 转码

将合并的多声道 WAV 转码为 FLAC 格式，支持最大压缩和杜比声道元数据：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1 --merge --flac
```

FLAC 特性：

- 最大压缩级别 (-8)
- 在 Vorbis 注释中保留原始杜比声道名称
- 声道布局标注为“源自杜比”
- 支持最多 8 个声道（FLAC 限制）

可选：转码后保留原始 WAV 文件：

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1 --merge --flac --keep-wav
```

不带 `--keep-wav` 时，转码成功后原始 WAV 会被删除以节省磁盘空间。

## 输出格式

### 单声道文件

格式：`input.01_L.wav`、`input.02_R.wav`、……

规格：

- 采样格式：Float32
- 采样率：48000 Hz（与源文件相同）
- 声道数：1

### 合并后的多声道文件

格式：`input.wav`

规格：

- 采样格式：Float32
- 采样率：48000 Hz（与源文件相同）
- 声道数：根据配置（2-16 个声道）
- 声道顺序：按 ITU-R BS.2051 标准排列

### FLAC 文件

格式：`input.flac`

规格：

- 编码格式：FLAC（免费无损音频编码）
- 压缩：最大级别 (-8)
- 采样格式：24-bit PCM 整数
- 采样率：48000 Hz（与源文件相同）
- 声道数：最多 8 个（FLAC 规范限制）
- 元数据：Vorbis 注释，包含声道布局信息
- 文件大小：大约为原始 WAV 大小的 15-20%（最大压缩）

性能示例：

- 5.1 声道 16 分钟音频：~244 MB WAV → ~42 MB FLAC（压缩 82.8%）

## 日志

使用 `RUST_LOG` 环境变量控制日志级别：

```bash
RUST_LOG=debug ./MacinConvert-Atmos-Tool --input file.eac3
```

支持的级别：error、warn、info、debug、trace。

## 懒人模式

无需参数即可双击或运行二进制，程序会：

- 仅扫描当前目录（不递归），通过文件头检测 E-AC3/TrueHD，并按时间顺序逐个处理。
- 每个文件内部使用默认并发解码（默认 4，可用 `-j/--jobs` 或 `MCAT_MAX_PAR` 调整），按 9.1.6 配置自动 `--merge --cleanup`。
- 在批处理模式下将 `--output` 视为输出目录（若不存在自动创建），每个输出以输入基名命名。

命令等价示例：

```bash
./MacinConvert-Atmos-Tool --lazy
```

## 常见问题

### 找不到 Dolby 工具

可以通过以下方式指定工具位置：

- `--dolby-tools <PATH>`：基目录需包含 `gstreamer/bin/gst-launch-1.0` 与 `gst-plugins`。
- 环境变量：`MCAT_GST_LAUNCH` 与 `MCAT_GST_PLUGINS`，或 `MCAT_DOLBY_TOOLS` 基目录。
- 若未指定，将依次查找 `<exe_dir>/dolby-tools` 与系统 Dolby Reference Player 应用包。

### 解码速度慢

- 使用 `--single` 选项进行顺序解码可能会更慢。
- 并行解码更快但消耗更多内存。
- 解码速度主要取决于 GStreamer 插件的性能。

### 内存不足

使用 `--single` 选项进行顺序解码，一次只解码一个声道。

### 输出文件损坏

确保输出目录有足够的磁盘空间。一个典型的 9.1.6 解码输出约为源文件的 15-20 倍。

## 开发

### 构建调试版本

```bash
cargo build
```

### 运行检查

```bash
cargo fmt
cargo clippy -- -D warnings
```

### 测试

```bash
cargo run -- --input audio/sample_input.ec3 --channels 5.1
```

## 已知限制

### macOS TrueHD 8 通道限制

**问题**

在 macOS 上，Dolby Reference Player 的 GStreamer 插件仅支持解码 TrueHD Atmos 文件的前 8 个声道。虽然 TrueHD 文件可以包含多个 presentation（包括 16 通道版本），但 macOS 插件无法访问这些高通道版本。

**根本原因**

macOS 插件构建在实现级别禁用或移除了 `truehddec-presentation` 参数。虽然该属性在 GObject 元数据中存在，但无法通过 `gst-launch-1.0` 命令行或编程 API（PyGObject、Rust 绑定）访问。

**影响**

- E-AC3 文件：完全支持，无限制
- TrueHD 文件：仅前 8 个声道可解码

**解决方案**

使用 `--channels auto` 选项自动检测文件中实际可解码的声道数：

```bash
# 自动检测可用声道
./MacinConvert-Atmos-Tool --input file.mlp --channels auto
```

对于 TrueHD Atmos 文件，这将检测并提取恰好 8 个声道。如果需要访问 TrueHD 文件中的所有 presentation，可以：

1. 使用 Windows 版本的工具（Windows 支持 `truehddec-presentation`）
2. 使用 Dolby Reference Player CLI 导出特定 presentation
3. 等待 Dolby 更新 macOS 插件（不太可能）

## 许可证

MIT License

## 作者

Sakuzy

## 致谢

- Claude 4.5 Haiku (Anthropic)：负责本项目的代码编写。
- OpenAI GPT-5 与 Codex 系列：提供审查与质量保障。

## 免责声明

- 本项目仅供研究与教学使用——在解码专有格式之前，请先确认当地法律与许可条款。
- 项目不内置任何 Dolby 二进制文件；请自备工具并确保拥有合法使用权。
- 本项目与 Dolby 无任何从属或合作关系；相关商标归其各自权利人所有。

## 更新日志

### 0.1.2

新功能：

- 添加 FLAC 音频转码功能
  - 使用 `--flac` 标志将合并的多声道 WAV 转码为 FLAC
  - 支持 `--keep-wav` 标志保留原始 WAV 文件
  - 最大压缩级别 (-8) 用于最优文件大小
  - 在 FLAC Vorbis 注释中保留杜比声道名称
  - 声道布局标注为“源自杜比”
  - 支持最多 8 个声道（FLAC 规范限制）
  - 编码时将 32-bit Float WAV 转换为 24-bit PCM
  - 实际示例：244 MB WAV → 42 MB FLAC（压缩 82.8%）

### 0.1.1

改进：

- 为合并的声道添加 WAV 文件注释支持
  - 声道配置现在存储在 WAV INFO/ICOM chunk 中
  - mediainfo 和其他工具现在可以读取原始声道配置
  - 注释示例：`5.1.2 [1: L, 2: R, 3: C, 4: LFE, 5: Ls, 6: Rs, 7: Ltm, 8: Rtm]`
- 移除 JSON 元数据输出（已由 WAV 注释取代）
- 代码清理和小优化

### 0.1.0

初始版本，支持：

- E-AC3 和 TrueHD 格式检测
- 13 种声道配置
- 并行和顺序解码
- 声道合并和自动清理
