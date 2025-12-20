# Rust

## Dependencies

+ [Target](../../target.md)

## Mirrors

1. <https://mirrors.tuna.tsinghua.edu.cn/rustup/dist>
2. <https://rsproxy.cn/rustup>

## Install Rust

Reference: <https://forge.rust-lang.org/infra/other-installation-methods.html>

### Standalone Installers (Rustup)

1. Check the system environment
2. List Rust versions ( `https://static.rust-lang.org/dist/channel-rust-stable.toml` )
3. Download the installation package for the corresponding version.
   ( `https://static.rust-lang.org/rustup/dist/{TARGET_TRIPLE}/rustup-init{.exe?}` )
4. Install to the designated location
5. Check the installation status

### Standalone Installers (No Rustup)

1. Check the system environment
2. List Rust versions ( `https://static.rust-lang.org/dist/channel-rust-stable.toml` )
3. Download the installation package for the corresponding version.
   ( `https://static.rust-lang.org/rustup/dist/rustc-{VERSION}-{TARGET_TRIPLE}.tar.xz` )
4. Unzip the installation package
5. Install to the designated location
6. Check the installation status

### Source Code (No Rustup)

1. Choose Host Rust versions
2. Check the system environment
3. List Rust Source versions ( `https://static.rust-lang.org/dist/channel-rust-stable.toml` )
4. Download the source code ( `https://static.rust-lang.org/dist/rustc-{VERSION}-src.tar.xz` )
5. Unzip the source code
6. Compile the source code
7. Install to the designated location (or system link)
8. Check the installation status

## Uninstall Rust

### Rustup

1. Run `rustup self uninstall`
2. Check the uninstallation status

### Source Code

1. Remove the installation files (or system link)
2. Clean the source code directory (optional)
3. Check the uninstallation status

## Update Rust

### Rustup

1. Run `rustup update`

### Source Code

1. Choose Host Rust versions
2. Check the system environment
3. List Rust Source versions ( `https://static.rust-lang.org/dist/channel-rust-stable.toml` )
4. Download the source code ( `https://static.rust-lang.org/dist/rustc-{VERSION}-src.tar.xz` )
5. Unzip the source code (Overwrite)
6. Compile the source code
7. Install to the designated location (or system link)
8. Check the installation status
