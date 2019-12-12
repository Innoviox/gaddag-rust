import os

cmd = "cargo run --release text -n {} -s {} -e {} > tests/test_{}.txt"

n = 10

for i in range(10):
    # print(i)
    for j in range(10):
        os.system(cmd.format(n, (i + 1) / 10, (j + 1) / 10, str(i)+str(j)))
        # print(f"\t{j}")
