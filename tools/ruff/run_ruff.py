import os
import subprocess
import sys
from pathlib import Path

cwd = os.getcwd()
os.chdir(Path("~/monopatoux").resolve())
print(cwd)
here = Path(os.path.dirname(__file__))
print(f"here: {here}", file=sys.stderr)
RUFF_PATH = (here / "../../../+ruff+ruff/ruff-x86_64-unknown-linux-gnu/ruff").resolve()

print(f"[run_ruff.py] Ruff binary path: {RUFF_PATH}", file=sys.stderr)
print(f"[run_ruff.py] Ruff binary exists: {os.path.exists(RUFF_PATH)}", file=sys.stderr)

if not os.path.exists(RUFF_PATH):
    sys.stderr.write(f"Ruff binary not found at {RUFF_PATH}\n")
    sys.exit(1)

args = [RUFF_PATH] + sys.argv[2:]
result = subprocess.run(args)
sys.exit(result.returncode)
