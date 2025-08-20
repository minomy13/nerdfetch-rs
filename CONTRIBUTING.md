# Contributing

🤝 **Thank you for contributing!** 🤝

If you have any further questions, feel free to contact maintainers.

## Git

1. Fork this repository
2. Create a new branch of `dev`
3. Make your changes and when you're ready create a pull request
4. We'll discuss everything and eventually squash and merge your PR

## Formatting

Every submitted PR *must* be formatted with `cargo fmt`.

## Documentation

If your PR introduces new features, modifies the configuration, or makes similar changes, you must update the documentation in `README.md` accordingly.

## Adding a new module

- Add config schema to `config/schema.rs`. Just follow the example of existing module configs - adopt their derives, naming scheme, etc. Also, all
  properties must be optional. Finally, add your module config to the `Modules` struct. It must be optional as well. Sort alphabetically.
- Add your module as a new file to the `modules` directory. Follow the example of existing ones. Create a new struct and implement the `Module` trait.
- Add your module as a `mod` to `modules.rs`. Then, add it to the list in the `get_modules` function. Sort everything alphabetically.

### Accessing config

For accessing config (for example for the `is_active` method of the `Module` trait) you should use the `conf_unwrap_or!(config, or, path / to / config / property)` macro.

### Performance

- Avoid spawning child processes if possible. Prefer using the `sysinfo` crate or a similar alternative. If spawning a process is unavoidable,
  please discuss it with the maintainers in the related issue and benchmark the command’s performance.
