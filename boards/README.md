Platforms Supported by Tock
===========================

The `/boards` directory contains the physical hardware platforms
that Tock supports.

| Board                                                             | Architecture     | MCU            | Interface  | App deployment              | QEMU Support? |
|-------------------------------------------------------------------|------------------|----------------|------------|-----------------------------|---------------|
| [Hail](hail/README.md)                                            | ARM Cortex-M4    | SAM4LC8BA      | Bootloader | tockloader                  | No            |
| [Imix](imix/README.md)                                            | ARM Cortex-M4    | SAM4LC8CA      | Bootloader | tockloader                  | No            |
| [Nordic nRF52-DK](nordic/nrf52dk/README.md)                       | ARM Cortex-M4    | nRF52832       | jLink      | tockloader                  | No            |
| [Nordic nRF52840-DK](nordic/nrf52840dk/README.md)                 | ARM Cortex-M4    | nRF52840       | jLink      | tockloader                  | No            |
| [Nordic nRF52840-Dongle](nordic/nrf52840_dongle/README.md)        | ARM Cortex-M4    | nRF52840       | jLink      | tockloader                  | No            |
| [ACD52832](acd52832/README.md)                                    | ARM Cortex-M4    | nRF52832       | jLink      | tockloader                  | No            |
| [Nano 33 BLE](nano33ble/README.md)                                | ARM Cortex-M4    | nRF52840       | Bootloader | tockloader                  | No            |
| [Nano RP2040 Connect](nano_rp2040_connect/README.md)              | ARM Cortex-M0+   | RP2040         | custom     | custom                      | No            |
| [Clue nRF52840](clue_nrf52840/README.md)                          | ARM Cortex-M4    | nRF52840       | Bootloader | tockloader                  | No            |
| [BBC Micro:bit v2](microbit_v2/README.md)                         | ARM Cortex-M4    | nRF52833       | openocd    | tockloader                  | No            |
| [ST Nucleo F446RE](nucleo_f446re/README.md)                       | ARM Cortex-M4    | STM32F446      | openocd    | custom                      | #1827         |
| [ST Nucleo F429ZI](nucleo_f429zi/README.md)                       | ARM Cortex-M4    | STM32F429      | openocd    | custom                      | #1827         |
| [STM32F3Discovery kit](stm32f3discovery/README.md)                | ARM Cortex-M4    | STM32F303VCT6  | openocd    | custom                      | #1827         |
| [STM32F412G Discovery kit](stm32f412gdiscovery/README.md)         | ARM Cortex-M4    | STM32F412G     | openocd    | custom                      | #1827         |
| [WeAct F401CCU6 Core Board](weact_f401ccu6/README.md)             | ARM Cortex-M4    | STM32F401CCU6  | openocd    | custom                      | No            |
| [SparkFun RedBoard Artemis Nano](redboard_artemis_nano/README.md) | ARM Cortex-M4    | Apollo3        | custom     | custom                      | No            |
| [SparkFun RedBoard Red-V](redboard_redv/README.md)                | RISC-V           | FE310-G002     | openocd    | tockloader                  | Yes (5.1)     |
| [i.MX RT 1052 Evaluation Kit](imxrt1050-evkb/README.md)           | ARM Cortex-M7    | i.MX RT 1052   | custom     | custom                      | No            |
| [Teensy 4.0](teensy40/README.md)                                  | ARM Cortex-M7    | i.MX RT 1062   | custom     | custom                      | No            |
| [Pico Explorer Base](pico_explorer_base/README.md)                | ARM Cortex-M0+   | RP2040         | openocd    | openocd                     | No            |
| [Raspberry Pi Pico](raspberry_pi_pico/README.md)                  | ARM Cortex-M0+   | RP2040         | openocd    | openocd                     | No            |
| [SiFive HiFive1 Rev B](hifive1/README.md)                         | RISC-V           | FE310-G002     | openocd    | tockloader                  | Yes (5.1)     |
| [Digilent Arty A-7 100T](arty_e21/README.md)                      | RISC-V RV32IMAC  | SiFive E21     | openocd    | tockloader                  | No            |
| [OpenTitan Earlgrey on CW310](opentitan/earlgrey-cw310/README.md) | RISC-V RV32IMC   | EarlGrey       | custom     | custom                      | Yes (5.1)     |
| [LiteX on Digilent Arty A-7](litex/arty/README.md)                | RISC-V RV32IMC   | LiteX+VexRiscV | custom     | tockloader (flash-file)[^1] | No            |
| [Verilated LiteX Simulation](litex/sim/README.md)                 | RISC-V RV32IMC   | LiteX+VexRiscv | custom     | tockloader (flash-file)[^1] | No            |
| [ESP32-C3-DevKitM-1](esp32-c3-devkitM-1/README.md)                | RISC-V-ish RV32I | ESP32-C3       | custom     | custom                      | No            |
| [QEMU RISC-V 32 bit `virt` platform](qemu_rv32_virt/README.md)    | RISC-V RV32IMAC  | QEMU           | custom     | custom                      | Yes (duh)     |

[^1]: Tockloader is not able to interact with this board directly, but
      can be used to work on a flash-image of the board, which can in
      turn be flashed onto / read from the board. For more specific
      information, visit the board's README.

# Out of Tree Boards

Some projects that use Tock maintain their own board definitions outside the
Tock repository.

| Project                                                  | Boards                                     | Architecture   | MCU      | Build System  |
|----------------------------------------------------------|--------------------------------------------|----------------|----------|---------------|
| [OpenSK](https://github.com/google/opensk)               | nRF52840-DK, nRF52840-Dongle, nRF52840-MDK | ARM Cortex-M4  | nRF52840 | Python script |
| [OpenTitan](https://github.com/lowrisc/opentitan)        | OpenTitan                                  | RISC-V RV32IMC | EarlGrey | Meson         |
| [Tock-on-Titan](https://github.com/google/tock-on-titan) | golf2, papa                                | ARM Cortex-M3  | H1       | Makefiles     |
