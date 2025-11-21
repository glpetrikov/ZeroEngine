cd vendor/GLFW
mkdir build 
cd build
cmake .. -DBUILD_SHARED_LIBS=ON
make

cd ../../..


premake5 gmake2
cd build
bear -- make
cd Debug
echo "Running..."
./Sandbox