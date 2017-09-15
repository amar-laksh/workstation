build:
	cargo build --release

install:
	mv ./target/release/workstation /usr/bin/ && cp ./haarcascade_frontalface_alt.xml /usr/bin && chmod +x /usr/bin/workstation	
