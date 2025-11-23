cd vendor/GLFW
mkdir build 
cd build
cmake .. -DBUILD_SHARED_LIBS=ON
make

cd ../../..


premake5 gmake2
cd build
make clean
make -j6
cd Debug
echo "Running..."
./Sandbox