# The backend server, written in Rust.

## Dependencies

This requires, for compilation only (via apt):
- `libpq-dev`
- `libbz2-dev`
- `libclang-15-dev`
- `zlib1g-dev`
- `libz3-dev`
- `liblz4-dev`
- `libzstd-dev`
- `libssl-dev`
- `openssl`
- `pkg-config`
- `libsqlite3-dev`

For Ubuntu/Debian run:  
``sudo apt install libpq-dev libz2-dev libcland-15-dev zliblg-dev liblz4-dev libzstd-dev libssl-dev openssl pkg-config libsqlite3-dev``
## Compilation

To compile, just run one of these commands:

### Compilation - x86 (64-bit) Linux

- Dynamically linked, GNU: `cargo build --release --target x86_64-unknown-liunx-gnu`
- Statically linked, GNU: `cargo build --release --target x86_64-unknown-liunx-gnu -- -C target-feature=+crt-static`
- Statically linked, MUSL: `cargo build --release --target x86_64-unknown-liunx-musl`

### Compilation - x86 (32-bit) Linux

- Dynamically linked, GNU: `cargo build --release --target i686-unknown-liunx-gnu`
- Statically linked, GNU: `cargo build --release --target i686-unknown-liunx-gnu -- -C target-feature=+crt-static`
- Statically linked, MUSL: `cargo build --release --target i686-unknown-liunx-musl`

### Compilation - Arm (64-bit) Linux (supports RPi 4 and above)

- Dynamically linked, GNU: `cargo build --release --target aarch64-unknown-liunx-gnu`
- Statically linked, GNU: `cargo build --release --target aarch64-unknown-liunx-gnu -- -C target-feature=+crt-static`
- Statically linked, MUSL: `cargo build --release --target aarch64-unknown-liunx-musl`

### Compilation - Arm (32-bit) Linux (supports RPi 3 and below)

- Dynamically linked, GNU: `cargo build --release --target arm-unknown-liunx-gnueabihf`
- Statically linked, GNU: `cargo build --release --target arm-unknown-liunx-gnueabihf -- -C target-feature=+crt-static`
- Statically linked, MUSL: `cargo build --release --target arm-unknown-liunx-musleabihf`

## Probably Asked Questions

Q: Why am I writing this in Rust instead of, say, TypeScript?
A: Because password hashing, disk operations, and database usage are much more optimized in Rust. Also, since this is a 3D printing control panel, USB and other connection types are much easier to use in Rust.

Q: What stack am I using?
A: The async runtime is [Tokio](https://tokio.rs) (battle-tested and amazing), the web server is [Axum](https://lib.rs/crates/axum) (fast, efficient, and made by Tokio), the database is [PostgreSQL](https://postgresql.org) or [SQLite](https://sqlite.org) with [Diesel](https://diesel.rs), and the frontend is [React](https://react.dev/) and [React Router](https://reactrouter.com/en/main).

Q: How did I make this so quick?
A: I didn't. I took a lot of code from [Mayhem](https://github.com/RedstoneWizard08/mayhem).
