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

##################
# TOCK 2.0 TESTING
##################

c_hello_printf_long: build kernel
	@$(MAKE) -C libtock-c/examples/c_hello
	@$(MAKE) -C libtock-c/examples/tests/printf_long
	@$(shell cat \
		libtock-c/examples/c_hello/build/cortex-m7/cortex-m7.tbf \
		libtock-c/examples/tests/printf_long/build/cortex-m7/cortex-m7.tbf \
		> build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

console_recv_short: build kernel
	@$(MAKE) -C libtock-c/examples/tests/console_recv_short
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/console_recv_short/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

console_recv_long: build kernel
	@$(MAKE) -C libtock-c/examples/tests/console_recv_long
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/console_recv_long/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

rot13_client_service: build kernel
	@$(MAKE) -C libtock-c/examples/rot13_client
	@$(MAKE) -C libtock-c/examples/rot13_service
	@$(shell cat \
		libtock-c/examples/rot13_client/build/cortex-m7/cortex-m7.tbf \
		libtock-c/examples/rot13_service/build/cortex-m7/cortex-m7.tbf \
		> build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

c_hello_blink: build kernel
	@$(MAKE) -C libtock-c/examples/c_hello
	@$(MAKE) -C libtock-c/examples/blink
	@$(shell cat \
		libtock-c/examples/c_hello/build/cortex-m7/cortex-m7.tbf \
		libtock-c/examples/blink/build/cortex-m7/cortex-m7.tbf \
		> build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

lua-hello: build kernel
	@$(MAKE) -C libtock-c/examples/lua-hello
	$(OBJCOPY) --update-section .apps=libtock-c/examples/lua-hello/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

console_timeout: build kernel
	@$(MAKE) -C libtock-c/examples/tests/console_timeout
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/console_timeout/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

malloc_test01: build kernel
	@$(MAKE) -C libtock-c/examples/tests/malloc_test01
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/malloc_test01/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

malloc_test02: build kernel
	@$(MAKE) -C libtock-c/examples/tests/malloc_test02
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/malloc_test02/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

stack_size_test01: build kernel
	@$(MAKE) -C libtock-c/examples/tests/stack_size_test01
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/stack_size_test01/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

stack_size_test02: build kernel
	@$(MAKE) -C libtock-c/examples/tests/stack_size_test02
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/stack_size_test02/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

mpu_stack_growth: build kernel
	@$(MAKE) -C libtock-c/examples/tests/mpu_stack_growth
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/mpu_stack_growth/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

mpu_walk_region: build kernel
	@$(MAKE) -C libtock-c/examples/tests/mpu_walk_region
	$(OBJCOPY) --update-section .apps=libtock-c/examples/tests/mpu_walk_region/build/cortex-m7/cortex-m7.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

multi_alarm_test_whileone: build kernel
	@$(MAKE) -C libtock-c/examples/tests/multi_alarm_test
	@$(MAKE) -C libtock-c/examples/tests/whileone
	@$(shell cat \
		libtock-c/examples/tests/multi_alarm_test/build/cortex-m7/cortex-m7.tbf \
		libtock-c/examples/tests/whileone/build/cortex-m7/cortex-m7.tbf \
		> build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex

tutorials_05_ipc: build kernel
	@$(MAKE) -C libtock-c/examples/tutorials/05_ipc/led
	@$(MAKE) -C libtock-c/examples/tutorials/05_ipc/logic
	@$(shell cat \
		libtock-c/examples/tutorials/05_ipc/led/build/cortex-m7/cortex-m7.tbf \
		libtock-c/examples/tutorials/05_ipc/logic/build/cortex-m7/cortex-m7.tbf \
		> build/apps.tbf)
	$(OBJCOPY) --update-section .apps=build/apps.tbf $(KERNEL) build/apps.elf
	$(OBJCOPY) -O ihex build/apps.elf build/apps.hex
	$(LOADER) build/apps.hex
