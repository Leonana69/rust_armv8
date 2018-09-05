.PHONY: clean
CROSS_COMPILE ?= ./bin/aarch64-none-elf-

# PATH for aarch64-linux-gnu-gdb
PATH := $(PATH):/home/leochan/linaro-64/bin
SHELL := env PATH=$(PATH) /bin/bash


CC := $(CROSS_COMPILE)gcc
AS := $(CROSS_COMPILE)as
LD := $(CROSS_COMPILE)ld

rust_target = aarch64-unknown-none
# rust_target = aarch64-unknown-linux-gnu
rust_os := ./target/$(rust_target)/debug/libhello.a
linker_script = hello.ld
assembly_object_files = startup.o
FILE ?= hello.elf
TARGET = hello.elf

all: $(TARGET)

$(TARGET): xargo $(rust_os) $(assembly_object_files) $(LINKER_SCRIPT)
	$(LD) -n --gc-sections -T $(linker_script) -o $@ $(assembly_object_files) $(rust_os)
	$(CROSS_COMPILE)objcopy -Obinary hello.elf hello.bin
	$(CROSS_COMPILE)objdump -S hello.elf > hello.list

cargo:
	cargo build --target=$(rust_target)

xargo:
	xargo build --target=$(rust_target)

gdb:
	# LD_LIBRARY_PATH=bin::LD_LIBRARY_PATH:/usr/local/lib
	
	# export PATH=bin:${PATH}:/home/leochan/linaro-64/bin \
	LD_LIBRARY_PATH=/usr/local/lib aarch64-linux-gnu-gdb "$(FILE)" -ex "target remote localhost:1234"

%.o: %.S
	$(AS) -c $^ -o $@

qemu: $(TARGET)
	@qemu-system-aarch64 -M ? | grep virt >/dev/null || exit
	@echo "Press Ctrl-A and then X to exit QEMU"
	@echo
	qemu-system-aarch64	-M virt -cpu cortex-a57	\
						-smp 4 -m 4096 \
						-nographic \
						-kernel hello.elf

dqemu: $(TARGET)
	@qemu-system-aarch64 -M ? | grep virt >/dev/null || exit
	@echo "Press Ctrl-A and then X to exit QEMU"
	@echo
	qemu-system-aarch64	-M virt -cpu cortex-a57	\
						-smp 4 -m 4096 \
						-nographic \
						-s -S \
						-kernel hello.elf

clean:
	rm -f *.o *.bin *.elf *.list DAC* -r target
