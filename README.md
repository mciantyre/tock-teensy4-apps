# Example Tock applications for the Teensy 4

This repository includes Tock applications that run on the Teensy 4.
It's intended to support Teensy 4 Tock development and testing.

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
