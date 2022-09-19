# LeafiePong

A simple C++ [Pong](https://en.wikipedia.org/wiki/Pong) clone built using [SDL](https://www.libsdl.org/), the [MongoDB C++ Driver](https://www.mongodb.com/docs/drivers/cxx/) and [MongoDB Atlas](https://www.mongodb.com/atlas/database).

This project is part of MongoDB's BuildFest2022, and is based on the [tutorial by Austin Morlan](https://austinmorlan.com/posts/pong_clone/). To comply with the original license and to ensure appropriate attribution this project starts with a clone of the original [Git repository](https://code.austinmorlan.com/austin/pong).

## Installation

Currently only OSX build instructions are provided (poorly), but the code "should" be portable to Linux and Windows without much (if any) modification.

```bash
brew install gcc sdl2_ttf sdl2_mixer sdl2 mongo-c-driver
./build.sh
```

## Usage

Playing the game locally involves using the `up` and `down` arrow keys to control the player on the left and the `w` and `s` keys to control the player on the right.

## MIT License

Copyright (c) 2022 Alex Bevilacqua
Copyright (c) 2020 Austin Morlan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished
to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next
paragraph) shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
