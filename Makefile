all:
	echo "Building server..."
	cd server
	cargo build
	echo "Building client..."
	cd ../client
	cargo build