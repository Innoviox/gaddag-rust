from string import ascii_uppercase as au
from random import shuffle
alph = au + "?"

x, _y = [], []
for i in open("resources/leaves.txt").readlines():
    a, b = i.split()
    x.append([a.count(j) for j in alph])
    _y.append(float(b))
_xvar = [[j[i] for j in x] for i in range(len(alph))]

n = 100000

order = list(range(len(_xvar[0])))
shuffle(order)

xvar = []
for i in _xvar:
    xvar.append([])
    for j in order[:n]:
        xvar[-1].append(i[j])

y = [_y[j] for j in order[:n]]

print("loaded")

from statsmodels.formula.api import ols
import pandas as pd

X = dict(zip(alph, xvar))
X['blank'] = X.pop('?')

X['y'] = y

data = pd.DataFrame(X)


model = ols("y ~ " + " + ".join(alph[:-1]) + " + blank", data).fit()

print(model.summary())
