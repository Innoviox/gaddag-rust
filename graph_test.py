import numpy as np
import matplotlib.pyplot as plt
# This import registers the 3D projection, but is otherwise unused.
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 unused import
from statistics import mean

# setup the figure and axes
# fig = plt.figure(figsize=(8, 3))
# ax1 = fig.add_subplot(121, projection='3d')

# fake data
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
                # _x[i - 1] = mean(p1)
                # _y[j - 1] = mean(p2)
                d.append(mean(p2))
        else:
            d.append(0)
    data.append(d)
data = np.array(data)

fig = plt.figure()
ax = Axes3D(fig)

lx= len(data[0])            # Work out matrix dimensions
ly= len(data[:,0])
xpos = np.arange(0,lx,1)    # Set up a mesh of positions
ypos = np.arange(0,ly,1)
xpos, ypos = np.meshgrid(xpos+0.25, ypos+0.25)

xpos = xpos.flatten()   # Convert positions to 1D array
ypos = ypos.flatten()
zpos = np.zeros(lx*ly)

dx = 0.5 * np.ones_like(zpos)
dy = dx.copy()
dz = data.flatten()

ax.bar3d(xpos,ypos,zpos, dx, dy, dz, shade=True)

plt.show()
