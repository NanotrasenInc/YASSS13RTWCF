@echo off
:: https://stackoverflow.com/questions/4165387/create-folder-with-batch-but-only-if-it-doesnt-already-exist
if not exist "build" mkdir build
cd build
cmake ../ -G"CodeBlocks - MinGW Makefiles"
echo "Code::Blocks project files have been created in the build directory."
pause