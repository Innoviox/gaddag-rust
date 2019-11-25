from string import ascii_uppercase as au
alph = au + "?"

x, y = [], []
for i in open("resources/leaves.txt").readlines():
    a, b = i.split()
    x.append([a.count(j) for j in alph])
    y.append(b)

xvar = [[j[i] for j in x] for i in range(len(alph))]
print("loaded")

from statsmodels.formula.api import ols
import pandas as pd

X = dict(zip(alph, xvar))
X['blank'] = X.pop('?')
X['y'] = y

data = pd.DataFrame(X)


model = ols("y ~ " + " + ".join(alph[:-1]) + " + blank", data).fit()

print(model.summary())
