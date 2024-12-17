all:
	make build && make run

build:
	cargo build --release && \
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/async-os -O binary target/riscv64gc-unknown-none-elf/release/async-os.bin

run:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ./bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/async-os.bin,addr=0x80200000 \
		-D qemu.log -d in_asm,int,mmu,pcall,cpu_reset,guest_errors