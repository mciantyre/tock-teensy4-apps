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

- [ ] `examples/c_hello` and `examples/tests/printf_long`
  - uart_tx_small and uart_tx_large: applications that write to console with small and large buffers; run both in parallel to properly test virtualization
- [ ] `examples/tests/console_recv_short` and `examples/tests/console_recv_long`
- [ ] `examples/blink`
- [ ] `examples/rot13_client` and `examples/rot13_service`
- [ ] `examples/blink` and `examples/c_hello` ~~and `examples/buttons`~~
  - *No buttons available.*
- [ ] `examples/lua-hello`
- [ ] `examples/tests/console_timeout`
- [ ] `examples/tests/malloc_test01`
- [ ] `examples/tests/stack_size_test01`
- [ ] `examples/tests/stack_size_test02`
- [ ] `examples/tests/mpu_stack_growth`
- [ ] `examples/tests/mpu_walk_region`
  - *No buttons available.*
- [ ] `examples/tests/multi_alarm_test`
  - *Only one configured LED on board; combined with `whileone` to make it interesting.*
- [ ] `examples/tests/whileone`
