from string import ascii_uppercase as au
alph = au + "?"

x, y = [], []
for i in open("resources/leaves.txt").readlines():
    a, b = i.split()
    x.append([a.count(j) for j in alph])
    y.append(b)

xvar = [[j[i] for j in x] for i in range(len(alph))]

from sklearn import linear_model
import pandas as pd

df = pd.DataFrame(
