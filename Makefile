BINARY = target/xtensa-esp32-espidf/release/esp32-thermometer
USB ?= /dev/ttyUSB0

help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "📦  build  - Build the firmware"
	@echo "📥  upload - Build and upload the firmware to the ESP32"
	@echo "🧹  clean  - Clean build artifacts"

build: $(BINARY)

upload: $(BINARY)
	cargo +esp espflash flash --release --port $(USB) --monitor

monitor:
	cargo +esp espflash monitor --port $(USB)

$(BINARY): src/context.rs src/errors.rs src/main.rs src/temperature.rs src/uart.rs src/wifi.rs
	cargo +esp build --release

clean:
	rm -rf .embuild
	cargo clean

.PHONY: clean upload help build