# OpenFlash Hardware Guide

Complete guide for building your own OpenFlash NAND programmer.

## Bill of Materials

### Minimum Setup (~$5)

| Item | Quantity | Price | Notes |
|------|----------|-------|-------|
| Raspberry Pi Pico | 1 | ~$4 | Or clone |
| Jumper wires | 20 | ~$1 | Female-to-female |
| **Total** | | **~$5** | |

### Recommended Setup (~$15)

| Item | Quantity | Price | Notes |
|------|----------|-------|-------|
| Raspberry Pi Pico | 1 | ~$4 | |
| TSOP-48 adapter | 1 | ~$3 | With ZIF socket |
| Breadboard | 1 | ~$2 | Half-size |
| Jumper wires | 30 | ~$2 | Various |
| 10kΩ resistors | 5 | ~$1 | Pull-ups |
| 100nF capacitors | 5 | ~$1 | Decoupling |
| **Total** | | **~$13** | |

### Pro Setup (~$30)

| Item | Quantity | Price | Notes |
|------|----------|-------|-------|
| Raspberry Pi Pico | 1 | ~$4 | |
| Custom PCB | 1 | ~$10 | See PCB section |
| TSOP-48 ZIF socket | 1 | ~$5 | For easy chip swap |
| Pin headers | 1 set | ~$2 | |
| SMD components | 1 set | ~$5 | Resistors, caps |
| Enclosure | 1 | ~$4 | 3D printed |
| **Total** | | **~$30** | |

## Wiring Diagrams

### Basic Breadboard Setup

```
                    Raspberry Pi Pico
                    ┌─────────────────┐
                    │ USB             │
                    │ ┌─────────────┐ │
              GP0 ──┤ │             │ ├── VBUS
              GP1 ──┤ │             │ ├── VSYS
              GND ──┤ │             │ ├── GND
              GP2 ──┤ │             │ ├── 3V3_EN
              GP3 ──┤ │             │ ├── 3V3 ──────┐
              GP4 ──┤ │             │ ├── ADC_VREF  │
              GP5 ──┤ │             │ ├── GP28      │
              GP6 ──┤ │             │ ├── GND       │
              GP7 ──┤ │             │ ├── GP27      │
              GP8 ──┤ │             │ ├── GP26      │
              GP9 ──┤ │             │ ├── RUN       │
             GP10 ──┤ │             │ ├── GP22      │
             GP11 ──┤ │             │ ├── GND       │
             GP12 ──┤ │             │ ├── GP21      │
             GP13 ──┤ │             │ ├── GP20      │
              GND ──┤ │             │ ├── GP19      │
             GP14 ──┤ │             │ ├── GP18      │
             GP15 ──┤ │             │ ├── GP17      │
                    │ └─────────────┘ │             │
                    └─────────────────┘             │
                                                   │
                         NAND Flash                │
                    ┌─────────────────┐            │
              CLE ──┤                 ├── VCC ─────┘
              ALE ──┤                 ├── GND ──┐
              WE# ──┤                 │         │
              RE# ──┤                 │         │
              CE# ──┤                 │         │
             R/B# ──┤                 │         │
               D0 ──┤                 │         │
               D1 ──┤                 │         │
               D2 ──┤                 │         │
               D3 ──┤                 │         │
               D4 ──┤                 │         │
               D5 ──┤                 │         │
               D6 ──┤                 │         │
               D7 ──┤                 ├── GND ──┘
                    └─────────────────┘
```

### Connection Table

| Pico Pin | GPIO | NAND Signal | Description |
|----------|------|-------------|-------------|
| Pin 1 | GP0 | CLE | Command Latch Enable |
| Pin 2 | GP1 | ALE | Address Latch Enable |
| Pin 4 | GP2 | WE# | Write Enable (active low) |
| Pin 5 | GP3 | RE# | Read Enable (active low) |
| Pin 6 | GP4 | CE# | Chip Enable (active low) |
| Pin 7 | GP5 | R/B# | Ready/Busy (low=busy) |
| Pin 9 | GP6 | D0 | Data bit 0 |
| Pin 10 | GP7 | D1 | Data bit 1 |
| Pin 11 | GP8 | D2 | Data bit 2 |
| Pin 12 | GP9 | D3 | Data bit 3 |
| Pin 14 | GP10 | D4 | Data bit 4 |
| Pin 15 | GP11 | D5 | Data bit 5 |
| Pin 16 | GP12 | D6 | Data bit 6 |
| Pin 17 | GP13 | D7 | Data bit 7 |
| Pin 36 | 3V3 | VCC | Power (3.3V) |
| Pin 3,8,13 | GND | GND | Ground |

## Important Considerations

### Power

⚠️ **CRITICAL**: NAND flash operates at 3.3V only!

- Never connect 5V to NAND pins
- Pico's 3V3 pin can supply ~300mA (enough for most chips)
- For high-current chips, use external 3.3V regulator

### Pull-up Resistors

The R/B# (Ready/Busy) signal needs a pull-up resistor:

```
3.3V ──┬── 10kΩ ──┬── R/B# pin
       │          │
       └──────────┴── GP5 (Pico)
```

### Decoupling Capacitors

Add 100nF capacitor between VCC and GND near the NAND chip:

```
VCC ──┬──────── NAND VCC
      │
     ═══ 100nF
      │
GND ──┴──────── NAND GND
```

### Signal Integrity Tips

1. **Keep wires short** - Under 10cm if possible
2. **Use ground plane** - Connect multiple GND points
3. **Twisted pairs** - Twist data lines with ground
4. **Avoid parallel runs** - Separate signal wires

## TSOP-48 Adapter

Most NAND chips use TSOP-48 package. An adapter makes connection easier:

### Recommended Adapters

- **TSOP48 to DIP48** - For breadboard use
- **TSOP48 ZIF socket** - For easy chip swapping
- **Universal programmer adapter** - Often includes socket

### Pinout Mapping

Standard TSOP-48 NAND pinout (check your chip's datasheet!):

| Pin | Signal | Pin | Signal |
|-----|--------|-----|--------|
| 1-2 | NC | 47-48 | NC |
| 3 | GND | 46 | NC |
| 4 | VCC | 45 | NC |
| ... | ... | ... | ... |
| 17 | CE# | 32 | NC |
| 18 | GND | 31 | GND |
| 19 | NC | 30 | NC |
| 20 | CLE | 29 | I/O7 |
| 21 | ALE | 28 | I/O6 |
| 22 | WE# | 27 | I/O5 |
| 23 | RE# | 26 | I/O4 |
| 24 | R/B# | 25 | VCC |

## Custom PCB Design

For a cleaner setup, consider a custom PCB:

### Features to Include

- TSOP-48 footprint or ZIF socket
- Pico header footprint
- Decoupling capacitors
- Pull-up resistors
- Status LEDs
- Power indicator
- ESD protection (optional)

### KiCad Files

Coming soon! Check the `hardware/` directory for:
- Schematic
- PCB layout
- Gerber files
- BOM

## Troubleshooting Hardware

### No Communication

1. Check all connections with multimeter
2. Verify 3.3V at NAND VCC pin
3. Check for shorts between adjacent pins
4. Try different USB cable

### Intermittent Errors

1. Add/check decoupling capacitors
2. Shorten wires
3. Check for cold solder joints
4. Add pull-up on R/B#

### Wrong Chip ID

1. Verify CLE and ALE connections
2. Check CE# is going low
3. Verify data bus connections
4. Check chip orientation

## Safety Notes

1. **ESD Protection** - Ground yourself before handling chips
2. **Power Sequence** - Connect GND before VCC
3. **Hot Swap** - Don't connect/disconnect while powered
4. **Verify Voltage** - Always check 3.3V before connecting chip

---

## SPI NAND Wiring (v1.1+)

SPI NAND uses the SPI0 peripheral on the Pico:

| Pico Pin | GPIO | SPI NAND Signal | Description |
|----------|------|-----------------|-------------|
| Pin 21 | GP16 | DO (MISO) | Data Out |
| Pin 22 | GP17 | CS# | Chip Select |
| Pin 24 | GP18 | CLK | Clock |
| Pin 25 | GP19 | DI (MOSI) | Data In |
| Pin 36 | 3V3 | VCC | Power (3.3V) |
| Pin 3,8,13 | GND | GND | Ground |

```
PICO          SPI NAND
────          ────────
GP16  ───►    DO (MISO)
GP17  ───►    CS#
GP18  ───►    CLK
GP19  ───►    DI (MOSI)
3V3   ───►    VCC   ⚠️  3.3V ONLY
GND   ───►    GND
```

---

## eMMC Wiring (v1.2+)

eMMC uses SPI mode via the SPI1 peripheral on the Pico:

| Pico Pin | GPIO | eMMC Signal | Description |
|----------|------|-------------|-------------|
| Pin 16 | GP12 | DAT0 (MISO) | Data Out |
| Pin 17 | GP13 | CS# | Chip Select |
| Pin 19 | GP14 | CLK | Clock |
| Pin 20 | GP15 | CMD (MOSI) | Command/Data In |
| Pin 36 | 3V3 | VCC | Power (check voltage!) |
| Pin 3,8,13 | GND | GND | Ground |

```
PICO          eMMC
────          ────
GP12  ───►    DAT0 (MISO)
GP13  ───►    CS#
GP14  ───►    CLK
GP15  ───►    CMD (MOSI)
3V3   ───►    VCC   ⚠️  Check voltage!
GND   ───►    GND
```

### eMMC Voltage Warning

⚠️ **IMPORTANT**: eMMC chips can operate at different voltages:
- **3.3V** - Most common for older/larger eMMC
- **1.8V** - Common for newer/mobile eMMC
- **Dual voltage** - Some chips support both

**Always check your eMMC datasheet before connecting!**

Many eMMC modules (like those from phone/tablet boards) have onboard voltage regulators. If using a bare eMMC chip, you may need a level shifter for 1.8V operation.

### eMMC Module Adapters

For easier connection, consider using:
- **eMMC to SD adapter** - Allows reading eMMC as SD card
- **eMMC socket board** - ZIF socket for BGA eMMC
- **eMMC breakout board** - Exposes all pins with headers

---

## STM32F1 (Blue Pill) Wiring (v1.25+)

The STM32F103C8T6 "Blue Pill" is an ultra-budget alternative to the Pico.

### STM32F1 SPI NAND Wiring

Uses SPI1 peripheral:

| Blue Pill Pin | GPIO | SPI NAND Signal | Description |
|---------------|------|-----------------|-------------|
| PA5 | SPI1_SCK | CLK | Clock |
| PA6 | SPI1_MISO | DO | Data Out |
| PA7 | SPI1_MOSI | DI | Data In |
| PA4 | GPIO | CS# | Chip Select |
| 3.3V | - | VCC | Power |
| GND | - | GND | Ground |

```
STM32F1       SPI NAND
───────       ────────
PA5   ───►    CLK
PA6   ◄───    DO (MISO)
PA7   ───►    DI (MOSI)
PA4   ───►    CS#
3.3V  ───►    VCC   ⚠️  3.3V ONLY
GND   ───►    GND
```

### STM32F1 eMMC Wiring

Uses SPI2 peripheral:

| Blue Pill Pin | GPIO | eMMC Signal | Description |
|---------------|------|-------------|-------------|
| PB13 | SPI2_SCK | CLK | Clock |
| PB14 | SPI2_MISO | DAT0 | Data Out |
| PB15 | SPI2_MOSI | CMD | Command/Data In |
| PB12 | GPIO | CS# | Chip Select |
| 3.3V | - | VCC | Power (check voltage!) |
| GND | - | GND | Ground |

```
STM32F1       eMMC
───────       ────
PB13  ───►    CLK
PB14  ◄───    DAT0 (MISO)
PB15  ───►    CMD (MOSI)
PB12  ───►    CS#
3.3V  ───►    VCC   ⚠️  Check voltage!
GND   ───►    GND
```

### STM32F1 Notes

- **USB**: Uses PA11 (D-) and PA12 (D+) for USB CDC
- **Clock**: Internal 8MHz HSI or external 8MHz crystal
- **Flash**: Requires ST-Link or serial bootloader (BOOT0 pin)
- **Power**: 3.3V operation, 5V tolerant on most GPIO pins

### Flashing STM32F1 Firmware

```bash
# Using ST-Link
st-flash write target/thumbv7m-none-eabi/release/openflash-firmware-stm32f1.bin 0x8000000

# Using serial bootloader (connect BOOT0 to 3.3V, reset, then flash)
stm32flash -w firmware.bin /dev/ttyUSB0
```
