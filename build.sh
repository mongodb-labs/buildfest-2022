#!/usr/bin/env sh
g++-12 $(pkg-config --cflags --libs sdl2) -lSDL2_mixer -lSDL2_ttf -o leafie_pong Main.cpp
