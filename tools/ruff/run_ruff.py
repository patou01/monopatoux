import os
import subprocess
import sys

# Use Bazel runfiles to find the ruff binary
try:
    from bazel_tools.tools.python.runfiles import runfiles
except ImportError:
    runfiles = None

RUFF_REL_PATH = "ruff-x86_64-unknown-linux-gnu/ruff"

if runfiles:
    r = runfiles.Create()
    ruff_path = r.Rlocation(os.path.join("ruff_bin", RUFF_REL_PATH))
else:
    # Fallback for non-Bazel execution
    ruff_path = os.path.join(os.path.dirname(__file__), RUFF_REL_PATH)

print(f"[run_ruff.py] Resolved Ruff binary path: {ruff_path}", file=sys.stderr)
print(f"[run_ruff.py] Ruff binary exists: {os.path.exists(ruff_path)}", file=sys.stderr)

if not os.path.exists(ruff_path):
    sys.stderr.write(f"Ruff binary not found at {ruff_path}\n")
    sys.exit(1)

args = [ruff_path] + sys.argv[1:]
result = subprocess.run(args)
sys.exit(result.returncode)
