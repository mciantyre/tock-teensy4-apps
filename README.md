# Tock 2.0 testing on the Teensy 4

This branch is for evaluating Tock 2.0 on the Teensy 4. It combines

- Tock at the `release-2.0-rc2` tag
- libtock-c at commit `0a4eadbd3` from `master`

See the subtree merge history for specifics. For more information on the Tock
2.0 testing effort, see [here](https://github.com/tock/tock/issues/2429).

## Dependencies

- Dependencies for Tock.
- Dependencoes for libtock-c.
- A Teensy program loader, ideally a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli).
  The [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html) should also work.

## Getting Started

Run

```bash
$ make all
```

to build the `blink` and `console` apps, then flash them to your
Teensy 4. You should observe

- the Teensy 4 LED blinking at 2Hz
- a serial console on pins 14 and 15 that echoes back characters

## Additional testing

- [ ] `examples/c_hello` and `examples/tests/printf_long`
  - uart_tx_small and uart_tx_large: applications that write to console with small and large buffers; run both in parallel to properly test virtualization
- [ ] `examples/tests/console_recv_short` and `examples/tests/console_recv_long`
  - We expect this fail for 1.6 (console capsule not fully virtualized), and might work for 2.0.
- [ ] `examples/blink`
  - blink: blinks LEDs
- [ ] `examples/rot_client` and `examples/rot_service`
  - rot_ipc: tests IPC with a simple service
- [ ] `examples/blink` and `examples/c_hello` and `examples/buttons`
- [ ] `examples/lua-hello`
- [ ] `examples/tests/console_timeout`
- [ ] `examples/tests/malloc_test01`
- [ ] `examples/tests/stack_size_test01`
- [ ] `examples/tests/stack_size_test02`
- [ ] `examples/tests/mpu_stack_growth`
- [ ] `examples/tests/mpu_walk_region`
- [ ] `examples/tests/multi_alarm_test`
- [ ] `examples/tests/adc`
- [ ] `examples/tests/adc_continuous`
- [ ] `examples/tutorials/05_ipc/led` and `examples/tutorials/05_ipc/rng` and `examples/tutorials/05_ipc/logic`
- [ ] `examples/tests/gpio` with mode set to 0
- [ ] `examples/tests/whileone`
