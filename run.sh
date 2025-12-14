set -e

#cd Fykor
#find . -name "*.cpp" -o -name "*.h" | xargs wc -l | sort -nr
#cd ..


mkdir -p build

premake5 gmake2
cd build
make -j6
cd Debug
echo "Running..."
./Sandbox