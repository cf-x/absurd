import platform
from utils import run, size, is_linux

def main():
    print(">>> verifying the toolchains...")
    if is_linux:
        run("rustup target add x86_64-unknown-linux-gnu")
        build_target = "x86_64-unknown-linux-gnu/"
        binary_name = "absurd"
    else:
        run("rustup target add x86_64-pc-windows-gnu")
        build_target = ""
        binary_name = "absurd.exe"

    print(f">>> building binaries for {platform.system()}...")
    if is_linux:
        run(f"cargo build --release --target {build_target}")
    else: 
        run(f"cargo build --release")

    if is_linux:
        run(f"strip target/{build_target}release/{binary_name}")

    size(f"target/{build_target}release/{binary_name}")

if __name__ == "__main__":
    main()
