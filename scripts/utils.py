import os
import subprocess
import platform

def run(command):
    result = subprocess.run(command, shell=True, check=True, text=True)
    return result

def size(path):
    if os.path.exists(path):
        size = os.path.getsize(path)
        print(f">>> {path} size: {size / (1024 * 1024):.2f} MB")
    else:
        print(f">>> {path} does not exist.")

is_linux = platform.system == "Linux"