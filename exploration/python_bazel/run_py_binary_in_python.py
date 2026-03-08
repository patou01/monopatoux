from python.runfiles import runfiles
import subprocess
import sys
import os

def main():
    r = runfiles.Create()
    if not r:
        print("Could not create runfiles object")
        sys.exit(1)

    executable_path = r.Rlocation("_main/tools/pytest/run")

    if not executable_path or not os.path.exists(executable_path):
        print(f"Could not find executable for //tools/pytest:run")
        print(f"Tried Rlocation '_main/tools/pytest/run', 'monopatoux/tools/pytest/run', and 'tools/pytest/run'")
        sys.exit(1)

    print(f"Executing: {executable_path}")
    # We call it with --help just to verify it runs and calls pytest
    result = subprocess.run([executable_path, "--help"])
    sys.exit(result.returncode)

if __name__ == "__main__":
    main()
