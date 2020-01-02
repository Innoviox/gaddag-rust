import matplotlib.pyplot as plt

plt.hist([float(i.split()[-1]) for i in open("resources/leaves.txt").readlines()])
plt.show()
