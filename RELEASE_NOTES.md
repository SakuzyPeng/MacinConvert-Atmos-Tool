# Release Notes / 发布说明

## v0.1.0-pre – Pre‑Release / 预发布

- Highlights / 亮点
  - Rust CLI for converting Dolby Atmos (E‑AC3/TrueHD) to multi‑channel WAV on macOS
    面向 macOS 的 Rust 命令行工具，将 E‑AC3/TrueHD 杜比全景声转换为多声道 WAV。
  - New lazy mode: one‑file‑at‑a‑time auto flow (discover → decode → merge → cleanup)
    新增懒人模式：按文件顺序自动处理（发现 → 解码 → 合并 → 清理）。

- Format & Channels / 格式与声道
  - Formats: E‑AC3, TrueHD
    支持格式：E‑AC3、TrueHD。
  - Channels: 2.0 → 9.1.6 presets (ITU‑R BS.2051 order)
    声道：2.0 至 9.1.6 预设（ITU‑R BS.2051 排列）。

- Performance / 性能
  - Default parallelism: 4 jobs (tunable via `-j/--jobs` or `MCAT_MAX_PAR`)
    默认并发：4（可通过 `-j/--jobs` 或 `MCAT_MAX_PAR` 调整）。
  - Reference timings on our dev Mac: 7.1.4 ≈ 10–11s; 9.1.6 ≈ 24–25s
    开发机参考耗时：7.1.4 ≈ 10–11 秒；9.1.6 ≈ 24–25 秒（实际因环境而异）。

- Tooling & Setup / 工具与配置
  - Locate tools by priority: `--dolby-tools` → `MCAT_GST_LAUNCH`+`MCAT_GST_PLUGINS` → `MCAT_DOLBY_TOOLS` → `./dolby-tools` → Dolby Reference Player
    工具查找优先级：`--dolby-tools` → `MCAT_GST_LAUNCH`+`MCAT_GST_PLUGINS` → `MCAT_DOLBY_TOOLS` → `./dolby-tools` → Dolby Reference Player。
  - Dolby binaries are not bundled; place them under `dolby-tools/` or point via CLI/env
    不内置 Dolby 二进制，请放入 `dolby-tools/` 或通过 CLI/环境变量指定。

- CLI Additions / 新增命令行
  - `--lazy`: auto batch (one file at a time), 9.1.6 + merge + cleanup
    `--lazy`：按文件顺序自动处理，默认 9.1.6 并自动合并与清理。
  - `--dolby-tools <PATH>`: base dir containing `gstreamer/bin` and `gst-plugins`
    `--dolby-tools <PATH>`：包含 `gstreamer/bin` 与 `gst-plugins` 的基目录。
  - `-j/--jobs <N>`: set parallel jobs (overrides default/env)
    `-j/--jobs <N>`：设置并发作业数（覆盖默认/环境）。

- Stability Changes / 稳定性改进
  - Drop child stdout/stderr to avoid pipe blocking; add `queue` and `filesink sync=false`
    丢弃子进程输出避免管道阻塞；引入 `queue` 与 `filesink sync=false` 降低背压。
  - Remove pre‑existing outputs before run (risk: deletes same‑name outputs; verify paths to avoid data loss)
    运行前清理同名旧输出文件（风险：会删除同名输出，操作前请确认路径与文件名，避免误删）。

- Artifacts / 产物
  - macOS builds uploaded as timestamped tar.gz (e.g., `MacinConvert-Atmos-Tool‑YYYY‑MM‑DD‑HH‑MM‑SS‑TZ.tar.gz`)
    macOS 构建以带时间戳的 tar.gz 形式上传（如 `MacinConvert-Atmos-Tool‑YYYY‑MM‑DD‑HH‑MM‑SS‑TZ.tar.gz`）。
  - Unsigned on macOS; Gatekeeper may warn — use Security & Privacy or `xattr -d com.apple.quarantine`
    macOS 产物未签名，可能触发 Gatekeeper 提示；可通过“安全性与隐私”或 `xattr -d com.apple.quarantine` 放行。

- Known Limitations / 已知限制
  - Requires valid Dolby/GStreamer components; TrueHD/E‑AC3 handling depends on upstream plugins
    依赖有效的 Dolby/GStreamer 组件；TrueHD/E‑AC3 行为受上游插件影响。
  - Container/codec matrix not fully validated; please report edge cases
    容器/编解码组合尚未完全验证，欢迎反馈边缘案例。

- Testing Invitation / 测试邀请
  - Please help exercise different containers, channel presets, and platform setups
    欢迎协助验证不同容器、声道预设与平台环境。
  - Feedback (with sample references when possible) → **ruuokk208@gmail.com**
    反馈（尽可能附样本信息）→ **ruuokk208@gmail.com**。

Please treat this tag as a pre‑release; validate critical results in your reference workflow.
此版本为预发布，关键场景请在自有流程中交叉验证结果。
