# Reproduction of possible wasm-bindgen bug on Webkit

This is a minimal reproduction of a possible bug in wasm-bindgen. Seems that the wasm-bindgen generated code for Webkit browsers crashes in the app http://wasm.simpleicons.org/ The full code is available at https://github.com/mondeja/simple-icons-website-rs

The bug only happens sometimes, so seems to be a memory error. I can't remove more code from this minimal reproducible example because the bug disappears. Even with the actual code the bug doesn't happens always. It raises `unreachable error` in the console, but sometimes raises an error from a struct not being built (`unwrap` error from a `None`) or a message with "(the wasm file) resource was preloaded using link preload but not used within a few seconds" and no error.

It doesn't happens in any other browser, so I think it's a bug in wasm-bindgen for Webkit. Only happens with `--release` mode enabled.

## Steps to reproduce

1. Add wasm32 target with `rustup target add wasm32-unknown-unknown`
2. Install Playwright with `npm install -DE @playwright/test anywhere`
3. Install Playwright browsers with `npx playwright install --with-deps`
4. Install Trunk `cargo install trunk`
5. Run `sh test.sh --wasm-bindgen-version "0.2.84" --max-attempts-to-reproduce 20 --opt-level z`

I've needed to create a script to reproduce it because it doesn't happens always. The script will try to build the app and run the tests until it's reproduced. After the crash it will open the trace generated by Playwright and you can inspect the console to see the error.

- You can change the wasm-bindgen version used by Trunk with the `--wasm-bindgen-version` argument, which is configured in *app/Trunk.toml* and *Cargo.toml* files.
- Set the optimization level with `--opt-level` argument, which is configured in *app/index.html* file.
