import pathlib
import re

USE_SUPER = re.compile("\w*use super::(.*);\w*")

def fix_imports(base, path):
  with open(path, "r") as infile:
    for line in infile:
      m = USE_SUPER.match(line)
      if m:
        # need a .parent here...
        new_use = "use crate::" + str(path.relative_to(base))[:-3].replace("/", "::") + "::" + m.group(1) + ";"
        old_use = line[:-1]
        print(f"sed 's/{old_use}/{new_use}/g' -i {str(path)} ")
        print("This does not work. It adds an extra path element...")

def main():
  root = pathlib.Path("/work/ProjectsForFun/rust/slaywasm/src")
  for path in root.glob("**/*.rs"):
    fix_imports(root, path)

if __name__ == "__main__":
  main()
