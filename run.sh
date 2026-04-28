set -e

cd ZeroEngine/vendor/ImGui
git checkout docking
cd ..
cd GLFW
git checkout master
git pull
cd ..
cd FrameLog
git checkout main
git pull
cd ..
cd glm
git checkout master
git pull
cd ../../..


#cd ZeroEngine/source/ZeroEngine
#find . -name "*.cpp" -o -name "*.h" | xargs wc -l | sort -nr
#cd ..

cargo +nightly fmt


mkdir -p build

premake5 gmake
cd build
make -j$(nproc)
make config=release -j$(nproc)
cd Debug
echo "Running..."
./Sandbox
