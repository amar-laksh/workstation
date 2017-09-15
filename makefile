build:
	cargo build --release

install:
	sudo mv ./target/release/workstation /usr/bin/ && sudo cp ./haarcascade_frontalface_alt.xml /usr/bin && chmod +x /usr/bin/workstation	
