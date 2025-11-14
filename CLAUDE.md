# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

**说中文！**: 此项目使用中文进行交流。请用中文回复所有问题和提供所有反馈。

## 快速命令参考

```bash
# 开发工作流 / Development workflow
cargo build              # 构建调试版本 / Build debug binary
cargo build --release   # 构建发布版本 / Build release binary
cargo check             # 快速验证代码 / Quick syntax check
cargo fmt               # 格式化代码 / Format code
cargo clippy -- -D warnings  # 代码检查 / Lint warnings
cargo run -- --help     # 查看所有 CLI 参数 / Show all CLI args

# 测试 / Testing
cargo run -- --input audio/test.eac3 --channels 5.1  # 测试基本功能 / Test basic functionality
RUST_LOG=debug cargo run -- --input test.eac3        # 启用调试日志 / Enable debug logging

# 项目设置 / Initial setup
bash scripts/setup-hooks.sh  # 配置 git hooks / Setup git hooks
```

## 双语编码规范

### 代码注释规范

所有代码注释使用**中英双语**，格式为：`中文 / English`。**禁止使用 emoji**

```rust
// 验证输入文件是否存在 / Verify that the input file exists
if !input_path.exists() {
    return Err("输入文件不存在 / Input file not found".into());
}

// 并行解码多个声道 / Decode multiple channels in parallel
let results: Vec<_> = channels
    .iter()
    .par_bridge()
    .map(|channel| decode_channel(channel))
    .collect();
```

**禁止使用 emoji 在注释中**（❌ 不要用 ✅ 或 🔍 等）

### 日志和打印语句规范

所有用户可见的日志、打印语句和错误提示使用**中英双语格式**，**禁止使用 emoji**

```rust
// 使用 log crate 时
info!("开始解码音频文件 / Starting to decode audio file: {:?}", input_path);
warn!("检测到无效的声道配置 / Invalid channel configuration detected: {}", channels);
error!("无法定位 GStreamer 工具 / Failed to locate GStreamer tools");

// 打印到 stdout/stderr 时
println!("[完成] 格式检测完成 / [Done] Format detection completed: {:?}", format);
eprintln!("[错误] 无法读取文件 / [Error] Failed to read file");
```

**禁止使用 emoji**（✅、❌、🔍 等）

### 提交信息规范

提交信息使用**中英双语**，遵循以下格式。**禁止使用 emoji**

```
<类型>: <中文描述> / <English description>

<中文详细说明 / English detailed explanation (可选)>

类型可以是：
  • feat: 新功能 / New feature
  • fix: 修复 bug / Bug fix
  • refactor: 重构 / Refactor
  • perf: 性能优化 / Performance improvement
  • docs: 文档更新 / Documentation update
  • test: 测试相关 / Test related
  • chore: 构建工具或依赖更新 / Build tool or dependency update
```

#### 提交信息示例

```
feat: 添加命令行参数 --no-numbers / Add command-line parameter --no-numbers

允许用户输出文件名不带声道编号，输出格式为 input.L.wav 而非 input.01_L.wav
This allows users to output filenames without channel numbers, with format input.L.wav instead of input.01_L.wav

相关 issue: #123 / Related issue: #123
```

```
fix: 修复合并声道时的采样率不匹配问题 / Fix sample rate mismatch when merging channels

在 merger.rs 中验证所有输入文件的采样率是否一致，防止合并时出现音频错位的问题。
Verify that all input files have consistent sample rates in merger.rs to prevent audio misalignment during merging.
```

```
refactor: 优化 tools.rs 中的工具定位逻辑 / Optimize tool location logic in tools.rs

使用 iterator chain 重构工具搜索逻辑，提高可读性。
Refactor tool search logic using iterator chain for better readability.
```

### 变量和函数命名规范

- **函数名、变量名**：使用英文 snake_case（如 `decode_channel`, `merge_channels`）
- **常量名**：使用英文 SCREAMING_SNAKE_CASE（如 `MAX_CHANNELS`, `DEFAULT_SAMPLE_RATE`）
- **类型/结构体名**：使用英文 PascalCase（如 `AudioFormat`, `ChannelConfig`）

示例：

```rust
// ✅ 正确
fn decode_audio_file(input_path: &Path) -> Result<Vec<f32>> { }
const MAX_CHANNELS: usize = 16;
struct ChannelConfiguration { }

// ❌ 不正确
fn 解码音频文件(input_path: &Path) -> Result<Vec<f32>> { } // 不要用中文命名
const 最大声道数: usize = 16; // 不要用中文命名
```

## 项目概述

**MacinConvert-Atmos-Tool** 是一个 Rust 命令行工具，用于在 macOS 上将杜比全景声（Dolby Atmos）音频文件（E-AC3/TrueHD 格式）转换为多声道 WAV 文件。该工具利用 GStreamer 和杜比专有插件来解码受保护的杜比音频格式。

### 主要功能
- 自动检测 E-AC3 和 TrueHD 格式
- 支持 13 种不同的声道配置（2.0 → 9.1.6）
- 并行和顺序解码模式
- 可选的声道合并以创建多声道 WAV 文件
- 自动清理中间文件

## 环境变量参考

| 变量 / Variable | 描述 / Description | 示例 / Example |
|---|---|---|
| `RUST_LOG` | 日志级别（error, warn, info, debug, trace） / Logging level | `RUST_LOG=debug` |
| `MCAT_GST_LAUNCH` | `gst-launch-1.0` 的绝对路径 / Absolute path to gst-launch-1.0 | `MCAT_GST_LAUNCH=/usr/bin/gst-launch-1.0` |
| `MCAT_GST_PLUGINS` | GStreamer 插件目录路径 / Path to GStreamer plugins dir | `MCAT_GST_PLUGINS=/path/to/gst-plugins` |
| `MCAT_DOLBY_TOOLS` | Dolby 工具基目录 / Base dir containing gstreamer/bin and gst-plugins | `MCAT_DOLBY_TOOLS=/path/to/dolby-tools` |
| `MCAT_MAX_PAR` | 最大并行作业数（可被 `-j/--jobs` 覆盖）/ Max parallel jobs (overridden by -j flag) | `MCAT_MAX_PAR=8` |
| `DYLD_LIBRARY_PATH` | 动态库搜索路径（macOS）/ Dynamic library search path (macOS) | `DYLD_LIBRARY_PATH=/path/to/libs:$DYLD_LIBRARY_PATH` |
| `GST_PLUGIN_PATH` | GStreamer 插件搜索路径 / GStreamer plugin search path | `GST_PLUGIN_PATH=/path/to/plugins:$GST_PLUGIN_PATH` |
| `RUST_BACKTRACE` | 启用 Rust 崩溃堆栈跟踪 / Enable Rust crash backtrace | `RUST_BACKTRACE=1` 或 `full` |

## 项目设置

### 初次克隆后的设置

```bash
# 配置预提交钩子（推荐）/ Setup pre-commit hooks (recommended)
bash scripts/setup-hooks.sh

# 或者手动配置 / Or manually configure
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

## 构建与开发命令

```bash
# 构建调试版本
cargo build

# 构建优化的发布版本
cargo build --release

# 运行发布版本二进制文件
./target/release/MacinConvert-Atmos-Tool --input file.eac3 --channels 9.1.6

# 直接使用 cargo 运行工具
cargo run -- --input file.eac3 --channels 9.1.6

# 使用日志输出运行
RUST_LOG=debug cargo run -- --input file.eac3 --channels 9.1.6

# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 验证代码（构建但不链接）
cargo check
```

## CLI 参数

主要入口点：`src/main.rs` 使用 `src/cli.rs` 进行参数解析（Clap derive 风格）。

常见用法：
```bash
./MacinConvert-Atmos-Tool \
  --input movie.eac3 \
  --output ~/Movies/decoded \
  --channels 9.1.6 \
  --merge --cleanup
```

关键标志：
- `--input` (必需)：E-AC3 或 TrueHD 文件
- `--channels`：声道配置（默认：9.1.6）
- `--single`：顺序解码（节省内存）
- `--merge`：将声道合并为单个多声道 WAV
- `--cleanup`：合并后删除中间文件
- `--no-numbers`：输出文件名不带声道编号

## 代码架构

### 模块组织

1. **`cli.rs`**: CLI 参数解析（Clap derive）
2. **`format.rs`**: 音频格式检测（E-AC3 同步字：`0x0B77`，TrueHD：`0xF8726FBA`）
3. **`channels.rs`**: 声道配置映射（13 个预设：2.0、5.1、9.1.6 等）
4. **`tools.rs`**: 定位 GStreamer 和杜比插件
   - 优先级：本地 `dolby-tools/` → `/Applications/Dolby/Dolby Reference Player.app`
5. **`decoder.rs`**: GStreamer 管道构建和音频解码
   - 构建 `gst-launch-1.0` 的 shell 命令
   - 支持并行和顺序两种模式
6. **`merger.rs`**: 将单声道 WAV 文件组合为多声道格式
   - 验证格式一致性（采样率、帧数）
   - 使用 Float32 格式输出
7. **`error.rs`**: 自定义错误类型（thiserror）
8. **`main.rs`**: 协调整个工作流

### 数据流

```
输入（E-AC3/TrueHD）
  → 格式检测
  → 工具定位
  → 声道配置查询
  → GStreamer 解码管道
  → N 个单声道 WAV 文件
  → 可选：合并为多声道
  → 可选：清理中间文件
```

### 关键类型

- **`AudioFormat`** (`format.rs`)：`EAC3` 或 `TrueHD`
- **`ChannelConfig`** (`channels.rs`)：包含声道名称、ID 和布局
- **`DecodeMode`**：并行（更快）vs 顺序（节省内存）

## 重要依赖

- **clap 4.5**: 使用 derive 宏的 CLI 解析
- **hound 3.5**: WAV 文件 I/O
- **ndarray 0.15**: 数组操作（用于声道合并）
- **thiserror 1.0**: 错误类型派生
- **log + env_logger**: 结构化日志

## 外部工具依赖

该工具需要杜比播放参考播放器或兼容的 GStreamer 插件：
- `gst-launch-1.0`: GStreamer CLI
- `dlbac3parse`, `dlbtruehdparse`: 格式解析器
- `dlbaudiodecbin`: 杜比音频解码器
- 杜比 GStreamer 插件库（通过 `DYLD_LIBRARY_PATH` 设置）

### 设置 GStreamer 环境

如果使用本地 `dolby-tools/` 目录，可能需要配置环境变量：

```bash
# 设置插件库路径
export DYLD_LIBRARY_PATH="/path/to/dolby-tools/gst-plugins-libs:$DYLD_LIBRARY_PATH"

# 设置 GStreamer 插件路径
export GST_PLUGIN_PATH="/path/to/dolby-tools/gst-plugins:$GST_PLUGIN_PATH"

# 运行工具
./target/release/MacinConvert-Atmos-Tool --input file.eac3
```

## 常见开发任务

### 添加新的声道配置
1. 在 `channels.rs::CHANNEL_CONFIGS` 中添加预设
2. 定义声道名称和 ID（遵循现有模式：L、R、C、LFE、Ls、Rs、Lh、Rh 等）

### 修改解码管道
编辑 `decoder.rs::build_gst_command()` 以调整 GStreamer 参数或插件链。

### 改变输出格式
- 单声道：`decoder.rs` 中的 `hound::WavSpec`
- 多声道：`merger.rs::merge_channels()` 控制 Float32 格式

### 调试 GStreamer 问题
```bash
# 使用调试日志运行
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --input file.eac3

# 直接测试 GStreamer 命令
gst-launch-1.0 --help

# 测试特定格式检测
cargo run -- --input file.eac3 --format eac3 --channels 5.1

# 测试顺序解码（验证内存管理）
RUST_LOG=debug cargo run -- --input file.eac3 --single
```

### 验证多声道 WAV 输出

```bash
# 使用 hound 或其他 WAV 检查工具验证输出
# 例如：查看输出文件的元数据
file output.wav

# 检查声道数和采样率（需要 sox）
soxi output.wav
```

### 性能调优

选择解码模式：
- **并行模式**（默认）：更快，适合有充足内存的系统。N 个声道同时解码。
- **顺序模式**（`--single`）：一次一个声道，内存使用更少。用于内存受限环境。

建议：对于 9.1.6（16 声道）配置，如果系统内存 < 8GB，使用 `--single` 选项。

## 测试说明

目前没有单元测试框架。对于手动测试：

```bash
# 快速验证构建
cargo check

# 使用小音频文件进行集成测试
cargo run --release -- --input audio/test.eac3 --channels 5.1

# 测试所有声道配置
for channels in "2.0" "5.1" "7.1" "9.1.6"; do
  echo "Testing $channels..."
  cargo run --release -- --input audio/test.eac3 --channels $channels --output /tmp/test_$channels
done

# 验证合并功能
cargo run --release -- --input audio/test.eac3 --channels 5.1 --output /tmp/test_merge --merge --cleanup
```

测试检查清单：
1. 使用小音频文件验证格式检测（EAC3 和 TrueHD）
2. 使用 `--channels` 标志分别测试每个声道配置
3. 验证输出文件的声道数和采样率
4. 验证合并操作产生正确的样本交错
5. 确认 `--cleanup` 正确删除了中间文件

## 错误处理

所有操作在 `main()` 中返回 `Result<T, Box<dyn std::error::Error>>`。自定义错误使用 `thiserror` 获取上下文。常见失败点：
- 未找到输入文件（在 `main.rs` 中验证）
- GStreamer/杜比工具未安装
- 无效的声道配置
- 格式检测失败
- 合并期间 WAV 处理错误

## 提交前检查清单

在提交代码前，请运行以下检查：

```bash
# 1. 检查代码格式 / Check code formatting
cargo fmt

# 2. 运行 clippy 检查（必须通过所有 warnings） / Run clippy checks (must pass all warnings)
cargo clippy -- -D warnings

# 3. 验证构建成功 / Verify build succeeds
cargo build --release

# 4. 验证代码符合规范 / Verify code follows standards
# - 所有注释使用中英双语 / All comments use Chinese/English bilingual
# - 函数名、变量名使用英文 / Function and variable names use English
# - 禁止使用 emoji / No emojis in code comments
# - 日志使用中英双语 / Logs use Chinese/English bilingual

# 5. （可选）运行完整测试 / (Optional) Run full tests
cargo run -- --input audio/test.eac3 --channels 5.1
```

提交信息规范：
- 使用 Conventional Commits 格式：`<type>: <Chinese desc> / <English desc>`
- 保持简洁（第一行 < 50 字符）/ Keep concise (first line < 50 chars)
- 添加详细说明时，在第二行留空 / Leave blank line before detailed description

## Git 工作流程

- 主分支：`master` / Main branch: `master`
- 最近重写：Rust 实现（提交 590769c）/ Latest rewrite: Rust implementation (commit 590769c)
- 跟踪 `.DS_Store` 中的更改（当前未跟踪）/ .DS_Store tracking (currently untracked)

## macOS 平台限制说明 / macOS Platform Limitations

### 杜比全景声（Atmos）多声道解码限制

**问题概述 / Problem Overview**
- TrueHD Atmos 文件通常包含多个音频 presentation，包括 8 通道、16 通道等不同版本
- 在 macOS 上，Dolby Reference Player 的 GStreamer 插件 **仅支持解码前 8 个声道**
- Windows 版本支持 `truehddec-presentation` 参数来选择不同 presentation，但 **macOS 版本不支持**

**技术原因 / Technical Reason**
- macOS GStreamer 插件虽然声明了 `truehddec-presentation` 属性，但在命令行解析器中不可用
- 通过 gst-launch-1.0 设置该属性失败：`no property "truehddec-presentation" in element "dlbtruehddec"`
- 尝试通过 Python/Rust GStreamer 绑定直接设置属性也因库依赖问题失败
- **结论**：macOS 构建的插件在实现级别禁用或移除了该功能

**实现的解决方案 / Implemented Solution**
- 添加自动检测功能：`--channels auto`
- 自动尝试解码文件中的所有可用声道，直到遇到空声道
- 对于测试文件（Ver2-THD-from-DolbyMediaEncoder.mlp），自动检测返回 8 个有效声道

**使用建议 / Recommendations**
```bash
# 自动检测声道配置（推荐）/ Auto-detect channel configuration (recommended)
./MacinConvert-Atmos-Tool --input file.mlp --channels auto

# 使用特定声道配置（仅支持前 8 个声道）/ Use specific config (only first 8 channels supported)
./MacinConvert-Atmos-Tool --input file.mlp --channels 5.1
```

**后续工作 / Future Work**
如果用户需要访问 16 通道内容，可以考虑：
1. 在 Windows 环境中使用 Windows 版本的工具
2. 使用 Dolby Reference Player GUI 播放器（虽然许可证会过期）
3. 等待 Dolby 官方更新 macOS GStreamer 插件以启用该功能（不太可能）
