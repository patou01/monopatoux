import os
import subprocess
import sys
from enum import Enum
from pathlib import Path


os.chdir(Path("/home/drooppi/monopatoux"))
here = Path(os.path.dirname(__file__))
RUFF_PATH = (here / "../../../+ruff+ruff/ruff-x86_64-unknown-linux-gnu/ruff").resolve()

args = [RUFF_PATH] + sys.argv[2:]
result = subprocess.run(args)
sys.exit(result.returncode)
