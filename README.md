# lilmemcap

Code to run on the M0 on the RK3399.

Current code powers down a bunch of power domains, and measures current draw using an INA214 connected via SPI and reports the results over serial.

There is a `clocks` utility under [rk3399-tools](https://github.com/ahixon/rk3399-tools) which can help you figure out how they're connected and what functions to call to disable/enable those specific power domains for your usecase.
