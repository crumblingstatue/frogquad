# frogquad

`frogquad` is an opinionated fork of [macroquad](<https://github.com/not-fl3/macroquad>). Work in progress. 🐸

## Features

* Same code for all supported platforms, no platform dependent defines required.
* Efficient 2D rendering with automatic geometry batching.
* Minimal amount of dependencies: build after `cargo clean` takes only 16s on x230(~6 years old laptop).
* Immediate mode UI library included.
* Single command deploy for both WASM and Android.

## Supported Platforms

* PC: Windows/Linux/macOS;
* HTML5;
* Android;
* IOS.

## Build Instructions

### Setting Up a frogquad Project

frogquad is a normal rust dependency, therefore an empty frogquad project may be created with:

```sh
# Create empty cargo project
cargo init --bin
```

Add frogquad as a dependency to Cargo.toml:
```toml

[dependencies]
frogquad = "0.4"
```

Put some frogquad code in `src/main.rs`:
```rust
use frogquad::{
    color,
    conf::Conf,
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    window::{clear_background, screen_height, screen_width},
};

struct BasicShapesApp {}

impl frogquad::App for BasicShapesApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, color::LIGHTGRAY);

        draw_line(fc, 40.0, 40.0, 100.0, 200.0, 15.0, color::BLUE);
        draw_rectangle(
            fc,
            screen_width(fc) / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            color::GREEN,
        );
        draw_circle(
            fc,
            screen_width(fc) - 30.0,
            screen_height(fc) - 30.0,
            15.0,
            color::YELLOW,
        );

        draw_text(fc, "HELLO", 20.0, 20.0, 30.0, color::DARKGRAY);
    }
}

fn main() {
    frogquad::start(Conf::from_title("BasicShapes"), |_fc| {
        Box::new(BasicShapesApp {})
    });
}
```

And to run it natively:
```sh
cargo run
```

### Linux

```sh
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Windows

On windows both MSVC and GNU target are supported, no additional dependencies required.

Also cross-compilation to windows from linux is supported:

```sh
rustup target add x86_64-pc-windows-gnu

cargo run --target x86_64-pc-windows-gnu
```

One of the ways to server static .wasm and .html:

```sh
cargo install basic-http-server
basic-http-server .
```

### IOS

To run on the simulator:

```sh
mkdir MyGame.app
cargo build --target x86_64-apple-ios --release
cp target/x86_64-apple-ios/release/mygame MyGame.app
# only if the game have any assets
cp -r assets MyGame.app
cat > MyGame.app/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key>
<string>mygame</string>
<key>CFBundleIdentifier</key>
<string>com.mygame</string>
<key>CFBundleName</key>
<string>mygame</string>
<key>CFBundleVersion</key>
<string>1</string>
<key>CFBundleShortVersionString</key>
<string>1.0</string>
</dict>
</plist>
EOF

xcrun simctl install booted MyGame.app/
xcrun simctl launch booted com.mygame
```

<details>
<summary>Tips</summary>
Adding the following snippet to your Cargo.toml ensures that all dependencies compile in release even in debug mode. In frogquad, this has the effect of making images load several times faster and your applications much more performant, while keeping compile times miraculously low.

```toml
[profile.dev.package.'*']
opt-level = 3
```
</details>
