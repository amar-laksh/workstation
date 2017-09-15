build:
	cargo build --release

install:
	sudo cp ./target/release/workstation /usr/bin && sudo cp ./haarcascade_frontalface_alt.xml /usr/bin && sudo chmod +x /usr/bin/workstation	
