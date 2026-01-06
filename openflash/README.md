# OpenFlash 🔥

> **The Ultimate NAND Flash Dumper & Analyzer**  
> *Where premium software meets budget hardware*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-%23000000?logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Framework-Tauri-%23FF0000)](https://tauri.app/)

---

## 🚀 **What is OpenFlash?**

**OpenFlash** is a cutting-edge, open-source hardware-software toolkit designed for **dumping, analyzing, and writing raw NAND flash memory**. Born from the philosophy of *"Cheap hardware, premium software"*, it pushes all complex logic into a powerful desktop application while keeping microcontroller firmware minimal and efficient.

Perfect for **reverse engineers, hardware hackers, data recovery specialists, and embedded developers** who need to extract firmware from devices, analyze NAND flash dumps, or recover data from damaged storage.

---

## ✨ **Key Features**

### 🧠 **AI-Powered Analysis**
- **Smart filesystem detection** using machine learning algorithms
- **Automatic signature recognition** for common filesystems (SquashFS, UBIFS, YAFFS2, etc.)
- **Intelligent data structure analysis** to identify code vs data regions

### 🔍 **Advanced Auto-Detection**
- **ONFI database** with 100+ known NAND flash chips
- **Automatic chip parameter detection** (size, page size, block size, timing)
- **Real-time timing optimization** for different chip generations

### 🛡️ **ECC Handling**
- **Hamming & BCH error correction** algorithms
- **Automatic ECC detection and correction**
- **Raw data preservation** with optional ECC processing

### 🎨 **Visual Analysis Tools**
- **Hex viewer** with virtual scrolling for large dumps
- **Bitmap visualization** to identify data density patterns
- **Timeline view** of flash operations

### 🌐 **Cross-Platform Support**
- **Windows, macOS, Linux** desktop application
- **Hardware-agnostic firmware** (RP2040, STM32F1, more coming)
- **USB 2.0 High-Speed** for maximum throughput

---

## 🏗️ **Architecture**

```
┌─────────────────────────────────────────────────────────┐
│                    Desktop Application                  │
├─────────────────────────────────────────────────────────┤
│  GUI: Tauri + React/TypeScript + TailwindCSS            │
│  Core: Rust with async Tokio runtime                    │
│  USB: rusb/nusb for device communication                │
│  AI: tract for on-device ML inference                   │
└─────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────┐
│                    Core Library                         │
├─────────────────────────────────────────────────────────┤
│  • ONFI: Chip database & auto-detection                 │
│  • Protocol: USB communication protocol                 │
│  • ECC: Error correction algorithms                     │
│  • Analysis: AI-powered data analysis                   │
└─────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────┐
│                    Firmware (Microcontroller)           │
├─────────────────────────────────────────────────────────┤
│  • RP2040: Raspberry Pi Pico support                    │
│  • STM32F1: Blue Pill development board                 │
│  • Embassy: Async runtime for embedded Rust             │
│  • PIO: Hardware-level timing precision (RP2040)        │
└─────────────────────────────────────────────────────────┘
```

---

## 📋 **Supported Hardware**

### 🥁 **NAND Flash Types**
- **SLC, MLC, TLC** NAND Flash memories
- **ONFI 1.0, 2.0, 3.0, 4.0** compliant devices
- **Toggle Mode** NAND support (planned)
- **Parallel & Serial** interfaces

### 🖥️ **Microcontroller Targets**
- **Raspberry Pi Pico** (RP2040) - *Recommended*
- **Blue Pill** (STM32F103C8T6) - *Budget option*
- **ESP32-S3** - *Future support*
- **Custom PCB** designs - *Coming soon*

### 💻 **Host Systems**
- **Windows 10/11** (x64, ARM64)
- **macOS 10.15+** (Intel, Apple Silicon)
- **Linux** (Ubuntu 20.04+, Debian 11+, Arch)

---

## 🛠️ **USB Protocol**

OpenFlash uses a custom binary protocol over USB Bulk transfers:

```
┌─────────────┬─────────────────────────────────────────┐
│  Command ID │  Arguments (63 bytes)                   │
│  (1 byte)   │                                         │
├─────────────┼─────────────────────────────────────────┤
│    0x01     │  PING - Test connection                 │
│    0x02     │  BUS_CONFIG - Set timing parameters     │
│    0x03     │  NAND_CMD - Send NAND command (CLE)     │
│    0x04     │  NAND_ADDR - Send address (ALE)         │
│    0x05     │  NAND_READ_PAGE - Read page data        │
│    0x06     │  NAND_WRITE_PAGE - Write page data      │
│    0x07     │  READ_ID - Read chip ID                 │
│    0x08     │  RESET - Reset NAND flash               │
└─────────────┴─────────────────────────────────────────┘
```

---

## 🚀 **Quick Start**

### Prerequisites
- **Rust** (1.70+) - [Install Rust](https://rustup.rs/)
- **Node.js** (18+) - [Install Node.js](https://nodejs.org/)
- **Tauri prerequisites** - [Tauri Setup Guide](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation
```bash
# Clone the repository
git clone https://github.com/your-username/openflash.git
cd openflash

# Install dependencies
cd gui && npm install && cd ..

# Build the application
cargo tauri build
```

### Development
```bash
# Run in development mode
cargo tauri dev
```

---

## 📊 **Performance**

| Operation | Speed (RP2040) | Speed (STM32F1) | Notes |
|-----------|----------------|-----------------|-------|
| Chip ID Read | < 10ms | < 50ms | Instant recognition |
| Page Read (4KB) | ~100μs | ~500μs | Timing optimized |
| Full Dump (1GB) | ~45 min | ~3.5 hours | ECC processing included |

---

## 🧪 **AI Analysis Capabilities**

OpenFlash uses machine learning to identify:

- **Filesystem types**: SquashFS, UBIFS, YAFFS2, JFFS2, ext4
- **Firmware signatures**: Router firmwares, bootloader patterns
- **Data structures**: Compression formats, encryption headers
- **Anomaly detection**: Corrupted sectors, bad blocks

---

## 🤝 **Contributing**

We welcome contributions! Check out our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Roadmap
- [ ] **SPI NAND** support
- [ ] **eMMC** dumping capabilities  
- [ ] **Advanced ECC** algorithms
- [ ] **Hardware debugger** integration
- [ ] **Multi-device** parallel dumping
- [ ] **Web-based** analysis tools

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 **Acknowledgments**

- **Rust Embedded Team** for embassy ecosystem
- **Tauri Team** for the amazing framework
- **Open Source Hardware** community
- **NAND Flash** reverse engineering pioneers

---

<p align="center">
  <em>Made with ❤️ for the hardware hacking community</em>
</p>

<p align="center">
  <strong>OpenFlash - Because your data deserves to be free</strong>
</p>

