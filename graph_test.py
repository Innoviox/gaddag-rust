import numpy as np
from PIL import Image
from statistics import mean
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  
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
mi = min(map(lambda i: min(filter(bool, i)), data))
data = np.array([np.array([(j - mi if j > 0 else 0) for j in i]) for i in data])

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
##

##
##img = Image.new('RGB', (10, 10))
##
##for idx, row in enumerate(data):
##    for jdx, col in enumerate(row):
##        if col != 0:
##            a = int(((col - mi) / (m - mi)) * 255)
##            img.putpixel((idx, jdx), (a, a, a))
##
##img.show()
