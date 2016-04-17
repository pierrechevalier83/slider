# slider
A slider puzzle written in Rust using the Piston library

[![Build Status](https://travis-ci.org/pierrechevalier83/slider.svg?branch=master)](https://travis-ci.org/pierrechevalier83/slider) [![Clippy Linting Result](https://clippy.bashy.io/github/pierrechevalier83/slider/master/badge.svg)](https://clippy.bashy.io/github/pierrechevalier83/slider/master/log)

## How to configure
- Check out src/settings.rs
- You can customize the image path
  - The image needs to be present in the assets folder
- If no image is found, it will default to a gradient of colors
- You can also customize various sizes

## Build steps
With high resolution pigtures, debug is very slow
`cargo run --release`

## How to play
- Arrows to move tile
- Space to shuffle

![alt tag](https://raw.githubusercontent.com/pierrechevalier83/slider/master/slider.png)

## TODO
- unit tests
- get rid of global state (mostly from settings.rs)
- settings: make configurable (size, color, image or not, ...)
- scoring mechanisms

## Notes
I am new to rust. Feel free to leave feedback on the code. (I now I must be missing lots of opportunities for using idiomatic constructions)
