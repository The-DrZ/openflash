# OpenFlash Roadmap

Детальный план развития проекта OpenFlash.

## Текущий статус: v2.3.5

---

## ✅ Завершённые версии

### v1.0 — Initial Release
- Parallel NAND read/write
- 30+ чипов в базе данных
- Hamming + BCH ECC
- Детекция SquashFS/UBIFS/JFFS2

### v1.1 — SPI NAND Support
- 20+ SPI NAND чипов
- Quad SPI (QSPI) поддержка
- Internal ECC статус
- Всего 4 провода!

### v1.2 — eMMC Support
- eMMC/MMC через SPI mode
- CID/CSD/EXT_CSD регистры
- Block read/write операции
- Boot partition доступ

### v1.25 — STM32F1 Expansion
- SPI NAND для Blue Pill
- eMMC для Blue Pill
- Полный паритет с RP2040

### v1.3 — AI-Powered Analysis
- Pattern recognition
- Anomaly detection
- Recovery suggestions
- Chip-specific recommendations

### v1.4 — AI Analysis v1.4
- Filesystem detection (YAFFS2, UBIFS, ext4, FAT...)
- OOB/spare area analysis
- Encryption key search (AES-128/192/256)
- Wear leveling analysis
- Memory map visualization
- Dump comparison
- Report export


### v1.5 — ESP32 & STM32F4 Support
- ESP32 firmware с WiFi/BLE
- STM32F4 firmware (USB OTG, FSMC)
- Web interface для ESP32
- 4 платформы: RP2040, STM32F1, STM32F4, ESP32

### v1.6 — NOR Flash & UFS Support
- SPI NOR flash (W25Q, MX25L, IS25LP) — 30+ чипов
- UFS (Universal Flash Storage) — версии 2.0-4.0
- ONFI 5.0 support с NV-DDR3
- 16-bit parallel NAND bus
- 10 property-based тестов

### v1.7 — Advanced Write Operations
- Full chip programming с верификацией
- Bad block management
- Wear leveling write
- Incremental backup/restore
- Clone chip-to-chip
- 12 новых протокольных команд (0xA0-0xAB)

### v1.8 — Scripting & Automation
- Python API (pyopenflash) через PyO3
- CLI tool (openflash) с clap
- Batch processing
- Custom analysis plugins
- CI/CD integration
- 12 новых протокольных команд (0xB0-0xBB)

### v1.9 — Advanced AI Features
- ML-based chip identification
- Firmware unpacking (binwalk)
- Automatic rootfs extraction
- Vulnerability scanning
- Custom signature database
- 10 новых протокольных команд (0xC0-0xC9)

### v2.0 — Multi-device & Enterprise
- Multi-device parallel dumping
- Device farm management
- Remote operation (server mode)
- Production line integration
- REST API
- 16 новых протокольных команд (0xD0-0xDF)

### v2.1 — Hardware Expansion
**Статус:** ✅ Released

| Фича | Статус |
|------|--------|
| Official OpenFlash PCB | ✅ Done |
| TSOP-48 ZIF adapter board | ✅ Done |
| BGA rework station integration | ✅ Done |
| Logic analyzer mode | ✅ Done |
| JTAG/SWD passthrough | ✅ Done |

**OpenFlash PCB v1:**
- RP2040 + ESP32 combo
- TSOP-48 ZIF socket
- SPI NAND/NOR socket (SOP-8)
- eMMC socket
- USB-C + WiFi
- OLED display (128x64)
- ~$25 BOM

**Реализация:**
- Новый модуль `hardware` в core library
- 16 новых протокольных команд (0xE0-0xEF)
- 14 unit тестов для hardware модуля
- TSOP-48 pinout для Samsung, Hynix, Micron, Toshiba
- Logic analyzer до 24 MHz с VCD/Sigrok экспортом
- JTAG chain scanning и SWD debug interface

### v2.3 — Platform Expansion
**Статус:** ✅ Released

| Фича | Статус |
|------|--------|
| Raspberry Pi Pico 2 (RP2350) | ✅ Done |
| Raspberry Pi SBC (3B+, 4, 5, Zero 2W) | ✅ Done |
| Arduino GIGA R1 WiFi (STM32H747) | ✅ Done |
| Orange Pi (Zero 3, 2W, 5) | ✅ Done |

**Новые платформы:**

**1. Raspberry Pi Pico 2 (RP2350)** — Высокий приоритет
- Архитектура: Dual Cortex-M33 @ 150MHz (или RISC-V Hazard3)
- 520KB SRAM (vs 264KB на RP2040)
- Улучшенные PIO блоки для NV-DDR timing
- ARM TrustZone, secure boot
- Прямая миграция с RP2040

**2. Raspberry Pi SBC (Linux GPIO)**
- Поддержка: Pi 3B+, Pi 4, Pi 5, Zero 2W
- Работа через /dev/mem и rppal
- Unix socket для локального управления
- Headless server mode
- Высокая скорость обработки (CPU 1.5+ GHz)

**3. Arduino GIGA R1 WiFi (STM32H747)**
- Dual-core: Cortex-M7 @ 480MHz + Cortex-M4 @ 240MHz
- 1MB RAM, 2MB Flash
- USB OTG HS (512-byte packets)
- FMC для parallel NAND с hardware ECC
- SDMMC для eMMC/SD с HS200
- WiFi/BLE через Murata модуль

**4. Orange Pi (Allwinner/Rockchip)**
- Orange Pi Zero 3 (H618)
- Orange Pi Zero 2W (H616)
- Orange Pi 5 (RK3588)
- Memory-mapped GPIO
- Сверхдешёвые ($15-20) программаторы

**Новые возможности:**
- NV-DDR support на RP2350 (до 400MT/s)
- Hardware ECC на STM32H747 FMC
- HS200 mode для eMMC на Arduino GIGA
- Unix socket API для SBC платформ

**Итого поддерживаемых платформ: 9**
- MCU: RP2040, RP2350, STM32F1, STM32F4, STM32H747, ESP32
- SBC: Raspberry Pi, Orange Pi

---

### v2.3.5 — Teensy & Banana Pi ← ТЕКУЩАЯ
**Статус:** ✅ Released

| Фича | Статус |
|------|--------|
| Teensy 4.0 (NXP i.MX RT1062) | ✅ Done |
| Teensy 4.1 (+ SD card slot) | ✅ Done |
| Banana Pi M2 Zero (Allwinner H3) | ✅ Done |
| Banana Pi M4 Berry (Allwinner H618) | ✅ Done |
| Banana Pi BPI-F3 (SpacemiT K1 RISC-V) | ✅ Done |

**Teensy 4.0/4.1 — Game Changer! ⚡**
- **USB High Speed (480 Mbit/s)** — 10-20x быстрее USB Full Speed!
- **1GB дамп за 3-5 минут** вместо 45 минут на Pico
- NXP i.MX RT1062 @ 600 MHz (ARM Cortex-M7)
- FlexIO для precise NV-DDR timing
- Soft ECC on-the-fly (BCH-16/24) без потери скорости
- Logic analyzer mode (до 24 MHz sample rate)
- Teensy 4.1: SD card slot для автономной работы
- Platform IDs: 0x30 (4.0), 0x31 (4.1)

**Banana Pi — Budget SBC Alternative 🍌**
- M2 Zero: RPi Zero form factor ($15)
- M4 Berry: RPi 4 alternative ($25)
- BPI-F3: **RISC-V** (SpacemiT K1) — первая RISC-V плата!
- Hardware SPI для быстрых SPI NAND/NOR операций
- Memory-mapped GPIO для Allwinner SoCs
- Platform ID: 0x12

**Новые capabilities:**
- `sd_card` — SD card slot (Teensy 4.1)
- `logic_analyzer` — Logic analyzer mode
- `soft_ecc` — Software ECC on-the-fly

**Итого поддерживаемых платформ: 11**
- MCU: RP2040, RP2350, STM32F1, STM32F4, STM32H747, ESP32, Teensy 4.0, Teensy 4.1
- SBC: Raspberry Pi, Orange Pi, Banana Pi

---

### v2.2 — Expanded Memory Support
**Статус:** ✅ Released

| Фича | Статус |
|------|--------|
| Parallel NAND expansion (50+ chips) | ✅ Done |
| SPI NAND expansion (35+ chips) | ✅ Done |
| SPI NOR expansion (45+ chips) | ✅ Done |
| eMMC expansion (25+ chips) | ✅ Done |
| New manufacturers support | ✅ Done |

**Новые производители:**
- SPI NAND: Foresee, Dosilicon, Zetta, Puya, Boya
- SPI NOR: EON, XMC, Puya, Boya

**Расширенная поддержка:**
- GigaDevice GD5F1GM9 high-speed SPI NAND (166MHz)
- Micron MT25QL01G 1Gbit SPI NOR
- Samsung/Micron/Hynix eMMC 5.1 до 128GB
- TLC NAND до 32GB (Micron, Kioxia, SK Hynix)
- 1.8V и 1.2V варианты чипов

**Итого поддерживаемых чипов:**
- Parallel NAND: 60+
- SPI NAND: 55+
- SPI NOR: 75+
- eMMC: 40+

---

## 🚀 Будущие релизы

### v3.0 — OpenFlash Pro
**Цель:** Коммерческая версия

| Фича | Приоритет |
|------|-----------|
| Cloud sync & backup | 🟡 Medium |
| Team collaboration | 🟡 Medium |
| Chip database crowdsourcing | 🔴 High |
| AI model updates OTA | 🟡 Medium |
| Enterprise support | 🟢 Low |

---

## 🗓️ Таймлайн

| Версия | Дата | Статус |
|--------|------|--------|
| v1.5 | Q1 2026 | ✅ Released |
| v1.6 | Q1 2026 | ✅ Released |
| v1.7 | Q2 2026 | ✅ Released |
| v1.8 | Q2 2026 | ✅ Released |
| v1.9 | Q3 2026 | ✅ Released |
| v2.0 | Q4 2026 | ✅ Released |
| v2.1 | Q1 2027 | ✅ Released |
| v2.2 | Q1 2027 | ✅ Released |
| v2.3 | Q1 2027 | ✅ Released |
| v2.3.5 | Q1 2027 | ✅ Released |
| v3.0 | 2028 | 🔮 Future |

---

*Последнее обновление: Январь 2027*
