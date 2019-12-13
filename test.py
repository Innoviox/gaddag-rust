import os

cmd = "cargo run --release text -n {} -s {} -e {} > tests/test_{}.txt"

n = 100

for i in range(10):
    # print(i)
    for j in range(10):
        if j <= i:
            os.system(cmd.format(n, (i + 1) / 10, (j + 1) / 10, str(i+1)+str(j+1)))
        # print(f"\t{j}")
