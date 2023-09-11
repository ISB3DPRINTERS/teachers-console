# ISB3DPRINTERS PROJECT: teachers-console

## Deployment info

To build a binary, run `pnpm build` and then `cargo build --release`.
This will compile the client and then embed the client into the final binary.
All you need for deployment is the binary found in `target/release/teachers-console`.
You can then upload that to whatever server you want.
Make sure you have postgres configured and set up on that server.

