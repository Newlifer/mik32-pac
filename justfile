set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Command to install required tools
get-tools:
    echo "Installing required tools..."
    cargo install svd2rust
    cargo install form
    rustup component add rustfmt

# Command to build the project
build:
    echo "Building the project..."
    svd2rust -i ./misc/mik32v2.svd --target riscv
    form -i ./lib.rs -o ./src/
    cargo fmt
    rm lib.rs
    rm build.rs
