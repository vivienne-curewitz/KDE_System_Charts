pushd build
cmake ..
make
popd
mv build/app ./app
