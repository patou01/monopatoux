import os
import subprocess
import sys

def main():
    # Use the BUILD_WORKSPACE_DIRECTORY environment variable to find the project root.
    # This is set by 'bazel run'.
    workspace_dir = os.environ.get("BUILD_WORKSPACE_DIRECTORY")
    if workspace_dir:
        os.chdir(workspace_dir)

    # Pass all command-line arguments directly to pytest.
    # We use 'sys.executable -m pytest' to ensure we use the Python interpreter 
    # provided by the Bazel toolchain.
    cmd = [sys.executable, "-m", "pytest"] + sys.argv[1:]
    
    # Execute pytest and exit with its return code.
    result = subprocess.run(cmd)
    sys.exit(result.returncode)

if __name__ == "__main__":
    main()
