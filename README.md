# Thermometer

ESP32 Thermometer program in RUST.

## Development environment

### On your local machine

```bash
cargo install espup
espup install --extended-llvm
cargo install ldproxy
cargo install cargo-espflash
source ${HOME}/export-esp.sh   # to add in your .bashrc
```

### In a devcontainer

```bash
devcontainer open .
```

## Commands

### Build and upload

```bash
make build
make upload
```

### Monitor Serial

```bash
make monitor
```

### Clean all

```bash
make clean
```

## Device

When uploaded on the device, send to the serial the first command: `help`. Then follow what is displayed.