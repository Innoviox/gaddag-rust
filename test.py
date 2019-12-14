import os

cmd = "cargo run --release text -n {} -s {} -e {} >> tests/test_{}.txt"

n = 200
r = 1
for i in range(10):
    # print(i)
    for j in range(10):
        if j <= i:
            for _ in range(r):
                os.system(cmd.format(n, (i + 1) / 10, (j + 1) / 10, str(i+1)+str(j+1)))
        # print(f"\t{j}")
