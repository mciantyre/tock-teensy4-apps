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

To re-run these tests, see the hacky Tock 2.0 section in the Makefile.

- [x] `examples/c_hello` and `examples/tests/printf_long`
  - uart_tx_small and uart_tx_large: applications that write to console with small and large buffers; run both in parallel to properly test virtualization
- [x] `examples/tests/console_recv_short` and `examples/tests/console_recv_long`
  - We expect this fail for 1.6 (console capsule not fully virtualized), and might work for 2.0. **mciantyre: testing together failed; known issue. Tests pass individually**.
- [x] `examples/blink`
  - blink: blinks LEDs
- [ ] ❌ `examples/rot13_client` and `examples/rot13_service`
  - rot_ipc: tests IPC with a simple service **mciantyre: see rot13-test-panic.log.**
- [x] `examples/blink` and `examples/c_hello` ~~and `examples/buttons`~~
  - No buttons available at the moment.
- [x] `examples/lua-hello`
- [x] `examples/tests/console_timeout`
- [x] `examples/tests/malloc_test01`
- [x] `examples/tests/stack_size_test01`
- [x] `examples/tests/stack_size_test02`
- [x] `examples/tests/mpu_stack_growth`
- [x] `examples/tests/mpu_walk_region`
  - **mciantyre: didn't press a button to force overrun**
- [x] `examples/tests/multi_alarm_test`
  - **mciantyre: only one configured LED on board; combined with `whileone` to make it intersting**
- [x] `examples/tests/whileone`
