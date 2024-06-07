# Yo

**Y**ammy **O**utputs for ADB - An ADB helper

> [!WARNING]
> This project coded because I was bored. This project doesn't get fast updates and etc. 

## Installation
The installation of Yo can be done with **Cargo** and **Rust**.

```bash
git clone https://github.com/kadircelkx/yo.git
cd yo
cargo build --release 
mv target/release/yo /usr/local/bin
```

If you want all of this in one line use this:

```bash
git clone https://github.com/kadircelkx/yo.git && cd yo && cargo build --release && mv target/release/yo /usr/local/bin
```

## Usage

- Use `yo devices` to list all devices connected and can be listed in ADB.

- Use `yo connect <host>` to connect a device with host.

- Use `yo disconnect <host>` to disconnect a device with host.

- Use `yo push <from> <to>` to push a file from computer to device.

- Use `yo pull <from> <to>` to pull a file from device to computer.

- Use `yo install <app_path/package_name>` to install a app. [Refer to here](https://github.com/kadircelkx/yo/blob/master/README.md#yo-install)

- Use `yo shell` to open a interactive shell. Better to use native ADB shell.

## yo install
I think this is the most attractive feature of Yo. You can install a **.apk** file locally or from **Google Store**.

