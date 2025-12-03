mkdir -p build

premake5 gmake2
cd build
make -j6
cd Debug
echo "Running..."
./Sandbox.exe