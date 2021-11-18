run:
	cargo build --release
	sudo setcap 'CAP_SYS_PTRACE=ep' target/release/simple-counter-hacker
	./target/release/simple-counter-hacker --connector qemu_procfs --args win10 --collectors interfaces,recvprops,convars -vv