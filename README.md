# MacinConvert-Atmos-Tool

[English](README.md) | [中文](README.zh-CN.md)

A Rust command-line tool for converting Dolby Atmos audio files to multi-channel WAV files on macOS.

## Features

- Auto-detect E-AC3 and TrueHD audio formats
- Support 13 different channel configurations from 2.0 stereo to 9.1.6 Dolby Atmos
- Parallel decoding for multiple channels with improved speed
- Sequential decoding mode to save memory
- Optional channel merging to combine mono files into multi-channel WAV
- Automatic cleanup of intermediate files
- Support for local Dolby tools or system-installed Dolby Reference Player

## System Requirements

- macOS 10.13 or higher
- Rust 1.70 or higher for building
- GStreamer and Dolby-related plugins and libraries

## Installation

### Building

```bash
cargo build --release
```

Output binary is located at `target/release/MacinConvert-Atmos-Tool`.

### Dependencies and Tool Configuration

The tool requires the following GStreamer related components:

- gst-launch-1.0 - GStreamer command-line tool
- dlbac3parse - E-AC3 format parser
- dlbtruehdparse - TrueHD format parser
- dlbaudiodecbin - Dolby audio decoder
- Related GStreamer plugin libraries

#### Getting GStreamer Tools

There are two ways to obtain the required GStreamer tools:

**Method 1: System Installation**

If Dolby Reference Player is already installed on your system, the tool will automatically search for it at the following location:

```
/Applications/Dolby/Dolby Reference Player.app/Contents
```

Install from: https://professional.dolby.com/product/media-processing-and-delivery/drp---dolby-reference-player/

**Method 2: Local dolby-tools Directory**

Place a `dolby-tools/` folder next to the executable with the following structure:

```
<exe_dir>/dolby-tools/
├── gstreamer/
│   └── bin/
│       └── gst-launch-1.0                    # GStreamer main program
├── gst-plugins/                               # GStreamer plugins directory
│   ├── libdlbac3parse.so                     # E-AC3 parsing plugin
│   ├── libdlbtruehdparse.so                  # TrueHD parsing plugin
│   ├── libdlbaudiodecbin.so                  # Dolby audio decoder
│   └── [other GStreamer plugins]
└── gst-plugins-libs/                         # GStreamer plugin dependencies
    ├── libdlb*.dylib                         # Dolby library files
    ├── libgst*.dylib                         # GStreamer library files
    └── [other dependency libraries]
```

When not specified via CLI/env, the tool looks next to the executable first, then the system Dolby Reference Player.

#### Environment Overrides

You can override tool locations via environment variables:

- `MCAT_GST_LAUNCH`: absolute path to `gst-launch-1.0`
- `MCAT_GST_PLUGINS`: path to GStreamer plugins dir
- `MCAT_DOLBY_TOOLS`: base dir containing `gstreamer/bin` and `gst-plugins`

Lookup order:

1. `MCAT_GST_LAUNCH` + `MCAT_GST_PLUGINS`
2. `MCAT_DOLBY_TOOLS`
3. `<exe_dir>/dolby-tools`
4. Dolby Reference Player app bundle

#### Obtaining GStreamer Components

GStreamer and related plugins can be obtained from the following sources:

- System package manager (`brew install gstreamer`)
- Official GStreamer binary releases
- Other applications that include GStreamer

Place the required binary files and libraries in the above directory structure.

## Usage

### Basic Usage

```bash
./MacinConvert-Atmos-Tool --input file.eac3
```

### Specifying Channel Configuration

Default is 9.1.6 (16 channels). Other available configurations:

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1
```

Supported channel configurations:

- 2.0 - Stereo
- 3.1 - Left, Right, Center, LFE
- 5.1 - Standard surround
- 7.1 - Extended surround
- 9.1 - Nine channel
- 5.1.2, 5.1.4 - Dolby Atmos (5.1 base)
- 7.1.2, 7.1.4, 7.1.6 - Dolby Atmos (7.1 base)
- 9.1.2, 9.1.4, 9.1.6 - Dolby Atmos (9.1 base)

### Specifying Output Location

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --output ~/Movies/decoded
```

If not specified, output files will be in the same directory as the input file.

### Specifying Audio Format

The program auto-detects the format, but you can also specify it explicitly:

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --format eac3
./MacinConvert-Atmos-Tool --input file.thd --format truehd
```

### Sequential Decoding (Memory-Efficient)

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --single
```

Parallel decoding is faster but uses more memory. Sequential decoding processes one channel at a time, saving memory.

### Merging Channels

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge
```

This option merges all separated mono WAV files into a single multi-channel WAV file.

### Cleanup After Merging

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --merge --cleanup
```

Automatically delete separated mono files after merging.

### Output Filename Format

Default format: `input.01_L.wav`, `input.02_R.wav`, ...

Output without numbers:

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --no-numbers
```

Output format: `input.L.wav`, `input.R.wav`, ...

### Complete Example

```bash
./MacinConvert-Atmos-Tool \
  --input /path/to/source_file.mp4 \
  --output ~/Movies/decoded \
  --channels 9.1.6 \
  --merge \
  --cleanup
```

This command will:

1. Detect the input file format
2. Get the 9.1.6 channel configuration (16 channels)
3. Decode all channels in parallel
4. Merge into a single multi-channel WAV file
5. Delete intermediate mono files

## Command-Line Arguments

```
Usage: MacinConvert-Atmos-Tool [OPTIONS]

Options:
  -i, --input <INPUT>
          Input audio file (E-AC3/TrueHD format; optional in lazy mode)
  -o, --output <OUTPUT>
          Output file base path (optional, defaults to input directory)
  -c, --channels <CHANNELS>
          Output channel configuration (default: 9.1.6)
  -f, --format <FORMAT>
          Input audio format (eac3/truehd, optional, auto-detect by default)
      --dolby-tools <PATH>
          Specify dolby-tools base directory (contains gstreamer/bin and gst-plugins)
  -j, --jobs <JOBS>
          Parallel jobs (overrides default and env MCAT_MAX_PAR)
      --no-numbers
          Output filenames without channel numbers
  -s, --single
          Sequential decoding of individual channels (saves memory)
  -m, --merge
          Merge decoded channels into a single multi-channel WAV file
      --cleanup
          Remove separated mono files after merging
      --lazy
          Lazy mode: auto batch one file at a time with merge + cleanup
      --flac
          Convert merged WAV to FLAC format with maximum compression
      --keep-wav
          Keep the original merged WAV file after FLAC conversion
  -h, --help
          Show help information
  -V, --version
          Show version information
```

### FLAC Conversion

Convert merged multi-channel WAV to FLAC format with maximum compression and Dolby channel metadata:

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1 --merge --flac
```

FLAC features:

- Maximum compression level (-8)
- Preserves original Dolby channel naming in Vorbis comments
- Channel layout marked as "Sourced from Dolby"
- Supports up to 8 channels (FLAC limitation)

Optional: keep the original WAV file after FLAC conversion:

```bash
./MacinConvert-Atmos-Tool --input file.eac3 --channels 5.1 --merge --flac --keep-wav
```

Without `--keep-wav`, the original WAV is deleted after successful FLAC conversion to save disk space.

## Output Format

### Mono Files

Format: `input.01_L.wav`, `input.02_R.wav`, ...

Specifications:

- Sample format: Float32
- Sample rate: 48000 Hz (same as source)
- Number of channels: 1

### Merged Multi-Channel File

Format: `input.wav`

Specifications:

- Sample format: Float32
- Sample rate: 48000 Hz (same as source)
- Number of channels: based on configuration (2-16 channels)
- Channel order: following ITU-R BS.2051 standard

### FLAC File

Format: `input.flac`

Specifications:

- Codec: FLAC (Free Lossless Audio Codec)
- Compression: maximum level (-8)
- Sample format: 24-bit PCM Integer
- Sample rate: 48000 Hz (same as source)
- Number of channels: max 8 (FLAC specification limit)
- Metadata: Vorbis comments including channel layout information
- File size: approximately 15-20% of original WAV size with max compression

Performance example:

- 5.1 channel 16-minute audio: ~244 MB WAV → ~42 MB FLAC (82.8% reduction)

## Logging

Control logging level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug ./MacinConvert-Atmos-Tool --input file.eac3
```

Supported levels: error, warn, info, debug, trace.

## Lazy Mode

Double-click or run the binary with no args, and it will:

- Scan the current directory only (non-recursive), detect E-AC3/TrueHD via headers, and process one file at a time in chronological order.
- For each file, decode with default parallelism (4 by default; tune via `-j/--jobs` or `MCAT_MAX_PAR`) and auto `--merge --cleanup` with 9.1.6.
- In batch mode, `--output` is treated as an output directory (auto-created), each output named after the input stem.

Equivalent command:

```bash
./MacinConvert-Atmos-Tool --lazy
```

## FAQ

### Dolby Tools Not Found

You can point the tool location via:

- `--dolby-tools <PATH>`: base dir must contain `gstreamer/bin/gst-launch-1.0` and `gst-plugins`.
- Environment variables: `MCAT_GST_LAUNCH` + `MCAT_GST_PLUGINS`, or base dir `MCAT_DOLBY_TOOLS`.
- If not set, it tries `<exe_dir>/dolby-tools` then the system Dolby Reference Player app bundle.

### Decoding is Slow

- Using `--single` for sequential decoding may be slower.
- Parallel decoding is faster but uses more memory.
- Decoding speed mainly depends on GStreamer plugin performance.

### Out of Memory

Use the `--single` option for sequential decoding to process one channel at a time.

### Corrupted Output Files

Make sure the output directory has enough disk space. A typical 9.1.6 decode output is 15-20 times the size of the source file.

## Development

### Building Debug Version

```bash
cargo build
```

### Running Checks

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Testing

```bash
cargo run -- --input audio/sample_input.ec3 --channels 5.1
```

## Known Limitations

### macOS TrueHD 8-Channel Limitation

**Issue**

On macOS, the Dolby Reference Player's GStreamer plugin only supports decoding the first 8 channels of TrueHD Atmos files. While TrueHD files can contain multiple presentations including 16-channel versions, the macOS plugin cannot access these higher-channel presentations.

**Root Cause**

The macOS plugin build disables or removes the `truehddec-presentation` parameter at the implementation level. Although the property exists in GObject metadata, it is not accessible via `gst-launch-1.0` command-line or programmatic APIs (PyGObject, Rust bindings).

**Impact**

- E-AC3 files: fully supported, no limitations
- TrueHD files: only first 8 channels decodable

**Workaround**

Use the `--channels auto` option to automatically detect the actual decodable channels in a file:

```bash
# Auto-detect available channels
./MacinConvert-Atmos-Tool --input file.mlp --channels auto
```

For TrueHD Atmos files, this will detect and extract exactly 8 channels. If you need to access all presentations in a TrueHD file, you may need to:

1. Use the Windows version of the tools (Windows supports `truehddec-presentation`)
2. Use the Dolby Reference Player CLI to export specific presentations
3. Wait for Dolby to update the macOS plugin (unlikely)

## License

MIT License

## Author

Sakuzy

## Acknowledgments

- Claude 4.5 Haiku (Anthropic): led the code implementation for this project.
- OpenAI GPT-5 and Codex series: provided code review and quality assurance.

## Disclaimer

- For research and educational use only — verify local laws and licenses before decoding proprietary formats.
- No Dolby binaries are bundled; you must supply your own tools and ensure you have rights to use them.
- We are not affiliated with Dolby; all trademarks belong to their respective owners.

## Changelog

### 0.1.2

New features:

- Add FLAC audio conversion support
  - Convert merged multi-channel WAV to FLAC format with the `--flac` flag
  - Support the `--keep-wav` flag to preserve the original WAV file
  - Maximum compression level (-8) for optimal file size
  - Preserve Dolby channel naming in FLAC Vorbis comments
  - Channel layout marked as "Sourced from Dolby"
  - Support up to 8 channels (FLAC specification limit)
  - 32-bit Float WAV to 24-bit PCM conversion during encoding
  - Real-world example: 244 MB WAV → 42 MB FLAC (82.8% reduction)

### 0.1.1

Improvements:

- Add WAV file comment support for merged channels
  - Channel configuration is now stored in the WAV INFO/ICOM chunk
  - mediainfo and other tools can now read the original channel configuration
  - Example comment: `5.1.2 [1: L, 2: R, 3: C, 4: LFE, 5: Ls, 6: Rs, 7: Ltm, 8: Rtm]`
- Remove JSON metadata output (replaced by WAV comments)
- Code cleanup and minor optimizations

### 0.1.0

Initial release with support for:

- E-AC3 and TrueHD format detection
- 13 channel configurations
- Parallel and sequential decoding
- Channel merging and automatic cleanup
