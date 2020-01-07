import matplotlib.pyplot as plt
import numpy as np
import statistics

data = [float(i.split()[-1]) for i in open("resources/leaves.txt").readlines()]
mu = statistics.mean(data)
sig = statistics.stdev(data)

n, bins, patches = plt.hist(data)

plt.show()
