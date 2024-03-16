install:
	# Build release binary.
	cargo build --release
	# Copy config file to /etc.
	sudo cp config.toml /etc/x11_edid_auto.toml
	# Allow everyone to read this, because it's for my work and personal user.
	sudo chmod a+r /etc/x11_edid_auto.toml
	# Copy binary to /usr/local/bin.
	sudo cp target/release/x11_edid_auto /usr/local/bin/x11_edid_auto
	# Allow everyone to execute this, because it's for my work and personal user.
	sudo chmod a+x /usr/local/bin/x11_edid_auto
