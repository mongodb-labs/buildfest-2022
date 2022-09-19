#!/usr/bin/env sh
MONGODB_CXX_VERSION=3.6.7
if [ ! -d "lib" ]
then
  curl -OL https://github.com/mongodb/mongo-cxx-driver/releases/download/r$MONGODB_CXX_VERSION/mongo-cxx-driver-r$MONGODB_CXX_VERSION.tar.gz
  mkdir -p lib
  tar -xzf mongo-cxx-driver-r$MONGODB_CXX_VERSION.tar.gz -C lib
  rm mongo-cxx-driver-r$MONGODB_CXX_VERSION.tar.gz
  cd lib/mongo-cxx-driver-r$MONGODB_CXX_VERSION/build
  cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local
  cmake --build . --target EP_mnmlstc_core
  cmake --build .
  make install
fi

g++-12 $(pkg-config --cflags --libs sdl2) -lSDL2_mixer -lSDL2_ttf -o leafie_pong Main.cpp
