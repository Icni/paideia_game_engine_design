# Testing

All of our tests are in the `tests` directory. Here are some tips to creating a test:

1. Create a terminal and navigate to the `tests` directory.
2. Run `cargo new test_name`, but replace `test_name` with a more meaningful name.
3. In `Cargo.toml`, add a dependency by typing in `engine_name = { path = "../.." }`, where `engine_name` is replaced by the actual name of your engine.
4. Create a folder called `assets` for all of the non-code resources of your test.
5. Create a folder called `textures` *inside* of `assets` for the textures (images) you want to use in your test.

## Globe Viewer test

Copy and paste the code from `starter code/globe_viewer.rs` into the `main.rs` file of your test.
