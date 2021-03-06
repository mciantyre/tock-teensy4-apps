TOCK_ROOT_DIRECTORY = tock/
TEENSY_LOADER = teensy_loader_cli
OBJCOPY = arm-none-eabi-objcopy

TARGET = thumbv7em-none-eabi
PLATFORM = teensy40
TEENSY4 = $(TOCK_ROOT_DIRECTORY)boards/$(PLATFORM)/
KERNEL = $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
LOADER = $(TEENSY_LOADER) --mcu=TEENSY40 -w -v

kernel:
	@$(MAKE) -C $(TEENSY4) kernel

build:
	@mkdir -p build

BLINK_APP = libtock-c/examples/blink/build/cortex-m7/cortex-m7.tbf
blink-app:
	@$(MAKE) -C libtock-c/examples/blink

CONSOLE_APP = libtock-c/examples/tests/console/build/cortex-m7/cortex-m7.tbf
console-app:
	@$(MAKE) -C libtock-c/examples/tests/console

build/blink.elf: kernel blink-app build
	@$(OBJCOPY) --update-section .apps=$(BLINK_APP) $(KERNEL) $@

build/console.elf: kernel console-app build
	@$(OBJCOPY) --update-section .apps=$(CONSOLE_APP) $(KERNEL) $@

build/%.hex: build/%.elf
	@$(OBJCOPY) -O ihex $< $@

blink: build/blink.hex
	@$(LOADER) $<

console: build/console.hex
	@$(LOADER) $<

clean:
	@rm -Rf build
	@$(MAKE) -C $(TEENSY4) clean
	@$(MAKE) -C libtock-c/examples/blink clean
	@$(MAKE) -C libtock-c/examples/tests/console clean

APPS = $(BLINK_APP) $(CONSOLE_APP)
all: kernel blink-app console-app build
	$(shell cat $(APPS) > build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex
