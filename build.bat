@echo off
echo Attempting to build server.
cd server
cargo build

echo Attempting to build client.
cd ..\client
cargo build

pause