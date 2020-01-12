import matplotlib.pyplot as plt
from matplotlib import colors
import matplotlib as mpl
import numpy as np
from string import ascii_uppercase

data = np.zeros((26, 26))

with open("resources/leaves.txt") as f:
    for line in f:
        a, b = line.split()
        if len(a) == 2 and '?' not in a:
            c, d = map(ascii_uppercase.index, a)
            data[d][c] = float(b)
            data[c][d] = float(b)

# create discrete colormap
bounds = list(range(int(min(map(min, data))) - 1,
                    int(max(map(max, data))) + 1))
# divs = np.array([1 / len(bounds), 1 / len(bounds), 1 / len(bounds)])
# rgbs = [np.ones(3) * i * divs for i in range(len(bounds))]
L = len(bounds)
div = 1 / L
rgbs = [
        [[i * div, 0.1, 0.1] for i in range(L // 3)] +
        [[0.1, 0.1, i * div] for i in range(L // 3)] +
        [[0.1, i * div, 0.1] for i in range(L // 3)]][0]

cmap = mpl.cm.viridis # colors.ListedColormap('viridis')
norm = colors.BoundaryNorm(bounds, cmap.N)

fig, ax = plt.subplots()
ax.imshow(data, cmap=cmap, norm=norm)

# draw gridlines
ax.grid(which='major', axis='both', linestyle='-', color='k', linewidth=2)
plt.xticks(list(range(26)), list(ascii_uppercase))
plt.yticks(list(range(26)), list(ascii_uppercase))
plt.title("Tile Synergies")
plt.show()
