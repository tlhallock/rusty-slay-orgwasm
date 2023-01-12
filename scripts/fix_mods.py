import pathlib
from collections import defaultdict



def find_mods(path, mod_files):
  mod_file = path.parent / "mod.rs"
  mod = path.name[:-3]
  if mod == "mod":
    return
  if path.parent.parent.name != "src":
    parent_parent = path.parent.parent / "mod.rs"
    mod_files[parent_parent].add(str(path.parent.name))
  mod_files[mod_file].add(mod)

def main():
  root = pathlib.Path("/work/ProjectsForFun/rust/slaywasm/src")
  mod_files = defaultdict(set)
  for path in root.glob("**/*.rs"):
    find_mods(path, mod_files)
  for mod_file, contained_modules in mod_files.items():
    # print(mod_file, contained_modules)
    with mod_file.open("w") as outfile:
      for contained_module in sorted(contained_modules):
        outfile.write(f"pub mod {contained_module};\n")

if __name__ == "__main__":
  main()




