
BUILD = cargo build
BUILD_FLAGS = --release

TARGET_DIR = ./target
TARGET = $(TARGET_DIR)/release/bsdfan

PREFIX = /usr/local
SBIN = $(PREFIX)/sbin
ETC = $(PREFIX)/etc

STARTUP = $(ETC)/rc.d/bsdfan
CMD = $(SBIN)/bsdfan

all: 
	$(BUILD) $(BUILD_FLAGS)

install: all
	cp -f $(TARGET) $(SBIN)

uninstall: 
	rm -f $(CMD) $(STARTUP)

clean:
	rm -rf $(TARGET_DIR)

