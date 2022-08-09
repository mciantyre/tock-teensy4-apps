# Tock 2.0 testing on the Teensy 4

This branch is for evaluating Tock 2.1 on the Teensy 4. It combines

- Tock at the `release-2.1-rc1` tag
- libtock-c at commit `1518ff49a` from `master`

See the subtree merge history for specifics. For more information on the Tock
2.1 testing effort, see [here](https://github.com/tock/tock/issues/3116#issuecomment-1209792251).

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

To re-run these tests, see the still-hacky Tock 2.1 section in the Makefile.

- [x] `examples/c_hello` and `examples/tests/printf_long`
  - uart_tx_small and uart_tx_large: applications that write to console with small and large buffers; run both in parallel to properly test virtualization
- [x] `examples/tests/console_recv_short` and `examples/tests/console_recv_long`
- [x] `examples/blink`
- [x] `examples/rot13_client` and `examples/rot13_service`
- [x] `examples/blink` and `examples/c_hello` ~~and `examples/buttons`~~
  - *No buttons available.*
- [x] `examples/lua-hello`
- [x] `examples/tests/console_timeout`
- [x] `examples/tests/malloc_test01`
- [x] `examples/tests/malloc_test02`
- [x] `examples/tests/stack_size_test01`
- [x] `examples/tests/stack_size_test02`
- [x] `examples/tests/mpu_stack_growth`
- [x] `examples/tests/mpu_walk_region`
  - *No buttons available.*
- [x] `examples/tests/multi_alarm_test`
  - *Only one configured LED on board; combined with `whileone` to make it interesting.*
- [x] `examples/tests/whileone`
