# Contribution Guidelines

Contributions to this project are welcome!

Though the functionality of this plugin is now mostly redundant with the bevy [gizmos API](https://docs.rs/bevy/latest/bevy/prelude/struct.Gizmos.html), it still may be useful for some projects.

## Styling Changes

Styling changes are not discouraged, but pull requests containing styling changes are required to separate those changes into their own commits when applicable.

Example: upon running `rustfmt`, commit it separately, distinct from the commmits containing code changes.

Styling change examples:

- Reordering imports
- Changing how arguments, tuple elements, or struct fields are laid out across lines
- Converting single-line expressions to multi-line (or vice versa)
- Re-indenting nested function parameters or changing their nesting style
- ... or any that don't impact the functionality or change the compiled output

## Updates to New Bevy Versions

Support for the newest bevy version may lag behind, in which case PRs updating this plugin are very welcome.

When contributing an upate, remember to do the following:

- Bump the plugin version in `Cargo.toml`, and the *Installation* section of the `README.md`
- Add a table entry in the *Compatibility* section of the `README.md` for the new version
- Check that examples run as expected
  - The compiler cannot catch everything when working with a game engine, some updates may change logic / expected behavior
  - In the case where examples don't behave as expected, engine debug logs are often helpful
- Check that the project still compiles for WASM (Web Assembly)
  - Install the `wasm32-unknown-unknown` target by running `rustup target add wasm32-unknown-unknown`
  - Compile the project with `cargo build --target wasm32-unknown-unknown`

**Thank You!**
