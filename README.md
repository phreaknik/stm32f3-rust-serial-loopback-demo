# stm32f3-rust-serial-loopback-demo
A serial loop-back project to use as a template for the Rust [RTFM framework](http://blog.japaric.io/rtfm-v2/) on the STM32F3 mcu. This project was developed using the STM32F3 Discovery Board, using the tutorials at [blog.japaric.io](http://blog.japaric.io) for guidance.


This project is and embedded project, meaning `rust-std` is not applicable on our system. To build this project without `rust-std`, use [Xargo](https://github.com/japaric/xargo) instead of Rust's native Cargo.

# Usage
Before building and running this program, a bit of hardware setup is required. Since this demo uses the UART interface, we need to connect a UART adapter from our PC to the Discovery Board. I use the [Adafruit CP2104 Friend](http://www.adafruit.com/product/3309), but any USB to serial adapter should work fine. Use jumper wires to connect the TX/RX pins of the adapter to the PA9/PA10 of the Discovery Board.

Now, open a serial port terminal (gtkterm/minicom/putty/etc) and connect to the UART adapter. Remember to configure the connection as follows:
* Baud rate: `115,200`
* Bits: `8`
* Parity: `none`
* Stop bits: `1`
* Flow control: `none`

Now you can build and run the project.

Build:

```$ xargo build```

Connect to the STM32 with openocd over ST-Link:

```$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg```

Attach a GDB debug session::

```
$ arm-none-eabi-gdb
>>> target remote :3333
>>> load
>>> tb main
>>> step
```

Run the project. If everything was setup correctly, you should be able to type into the serial terminal and see your text echoed back on the screen.

For convenience to anyone using VS Code, the build and debug steps have been added as custom tasks / launch config for VS Code.

# License
Licensed under the [MIT](/LICENSE) open source license.
