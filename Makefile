
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

install: 
	cp -f $(TARGET) $(SBIN)
	cp -f rc.d/bsdfan $(ETC)/rc.d/ && chmod 755 $(ETC)/rc.d/bsdfan
	test -f $(ETC)/bsdfan.conf || cp -f bsdfan.conf $(ETC)/

uninstall: 
	rm -f $(CMD) $(STARTUP)

clean:
	rm -rf $(TARGET_DIR)

