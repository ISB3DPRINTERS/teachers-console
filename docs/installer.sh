#!/bin/bash
is_user_root ()
{
    [ "${EUID:-$(id -u)}" -eq 0 ]
}
echo "VERSION 1.0.0 Build script"
echo "This script will install and build a binary of the project"
echo "Do you want to continue? y/n"
read confirmation

if [ "$confirmation" == "n" ]
then
echo "Exiting"
exit
fi
echo "Installing Build deps"
sudo apt install libpq-dev libz2-dev libcland-15-dev zliblg-dev liblz4-dev libzstd-dev libssl-dev openssl pkg-config
wait
echo "Checing pnpm is installed"
npm install pnpm -g
wait
echo "Installing react dependancies"
pnpm install
wait
echo "React building"
pnpm build
wait
echo "Compiling Rust project"
echo "This may take a while"
cargo build --release
wait
echo "Finished. Build is in target/release/teachers-console"
wait
exit
