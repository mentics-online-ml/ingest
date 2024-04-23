Anytime a dependent git repo is changed, you'll need to update the dependency for it by running:
cargo update -p rust-tradier

https://stackoverflow.com/questions/59766239/how-to-build-a-rust-app-free-of-shared-libraries

To minimize executable size and build static executable, build with:
RUSTFLAGS='-C link-arg=-s -Zlocation-detail=none' cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release


Got error for static building against musl, but the above was small enough, so that's ok.
To minimize executable size and build static executable, build with:
RUSTFLAGS='-C link-arg=-s' cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-musl --release
