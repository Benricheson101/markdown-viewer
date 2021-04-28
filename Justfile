default: release-linux

release-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

release-win:
	cargo build --release --target x86_64-pc-windows-gnu

# idk if this works
release-darwin:
	cargo build --release --target x86_64-apple-darwin

# vim:ft=make
