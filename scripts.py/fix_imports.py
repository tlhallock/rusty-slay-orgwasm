import pathlib
import re


def fix_imports(path):
  with open(path, "r") as infile:
    for line in infile:
      m = re.search("use super", line)
      if m:
        print(line)

def main():
  root = pathlib.Path("/work/ProjectsForFun/rust/slaywasm")
  for path in root.glob("**/*.rs"):
    fix_imports(path)

if __name__ == "__main__":
  main()
