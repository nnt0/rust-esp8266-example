# rust-esp8266-example
Some example code for using rust on the esp8266.

Unfortunately no one really supports the esp8266 or removed support for it. So everything is quite difficult to work with or just doesn't work on the esp8266.

# Installation

1. `cargo install espup`
2. `espup install -v 1.80.0.0` Version 1.80 is needed, it didn't work with other compiler versions
3. `git clone https://github.com/esp-rs/espflash`
4. `cd espflash`
5. `git checkout 8f43cc42261718e67f1c37dafe56d158c6290766` This is the commit before they removed esp8266 support
6. In espflash/src/targets/esp8266.rs set lines 53-56 and 61-64 to "Ok(0)" (without the ")
7. In espflash/src/connection/mod.rs comment out the lines 177-194
8. In cargo-espflash/Cargo.toml set the Versions of cargo in line 31 and 34 to 0.75.0
9. `cargo install --path path/to/the/cloned/espflash/repo/folder`
10. Download toolchain from https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/windows-setup.html#download-the-toolchain-for-the-esp8266 and unzip it
11. Add the bin folder in the downloaded folder to the path (alternatively execute `$Env:Path = $Env:Path + ";path/to/the/bin/folder"` in PowerShell)

Compile and flash with `cargo espflash flash --release --no-stub`

Also see <https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html>
