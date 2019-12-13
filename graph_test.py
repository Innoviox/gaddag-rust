import numpy as np
from PIL import Image
from statistics import mean
data = []
for i in range(1, 11):
    d = []
    for j in range(1, 11):
        if j <= i:
            with open(f"tests/test_{i}{j}.txt") as f:
                p1, p2 = [], []
                for game in f.read().split("#character-encoding UTF-8")[1:]:
                    p1.append(int([i for i in game.split("\n") if 'p1' in i][-1].split()[-1]))
                    p2.append(int([i for i in game.split("\n") if 'p2' in i][-1].split()[-1]))
                d.append(mean(p1))
        else:
            d.append(0)
    data.append(d)
data = np.array(data)

m = max(map(max, data))

img = Image.new('RGB', (10, 10))

for idx, row in enumerate(data):
    for jdx, col in enumerate(row):
        if col != 0:
            a = int((col / m) * 255)
            img.putpixel((idx, jdx), (a, a, a))

img.show()
