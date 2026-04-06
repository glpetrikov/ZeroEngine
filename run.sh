set -e

cd Fykor/vendor/ImGui
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
cd GLFW
git checkout master
git pull
cd ../../..


#cd Fykor/source/Fykor
#find . -name "*.cpp" -o -name "*.h" | xargs wc -l | sort -nr
#cd ..

mkdir -p build

premake5 gmake
cd build
make -jnproc
cd Debug
echo "Running..."
./Sandbox
