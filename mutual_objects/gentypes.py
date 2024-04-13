import os

os.system("cargo test")

bind_dir = os.path.join(os.getcwd(), "bindings")

with open("../frontend/src/types.d.ts", "w") as t_file:
    for filename in os.listdir(bind_dir):
        with open(os.path.join(bind_dir, filename), 'r') as f:
            content = f.read().splitlines()[2]
            content = content.replace("type", "interface")
            content = content.replace("= ", "")
            content = content.replace(";", "")
            t_file.write(f"{content}\n")
