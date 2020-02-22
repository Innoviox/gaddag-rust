import matplotlib.pyplot as plt
import numpy as np
import statistics

data = [(i.split()[0], float(i.split()[-1])) for i in open("resources/leaves.txt").readlines()]

ls = [i[0] for i in data]
data = ns = [i[1] for i in data]

print(ls[ns.index(min(ns))], ls[ns.index(max(ns))])

mu = statistics.mean(data)
sig = statistics.stdev(data)

n, bins, patches = plt.hist(data)

plt.title("Distribution of Leaves")
plt.xlabel("Evaluation")
plt.ylabel("Count")

plt.show()
