import matplotlib.pyplot as plt
p1s, p2s, ts = [], [], []
g1s, g2s = [], []
t = []
for i, game in enumerate(open("testcogo_3000.txt").read().split("#character-encoding UTF-8")[1:], start=1):
    lines = game.split("\n")[3:]
    times = [int(i.split()[-1]) for i in lines if 'Time' in i]
    p1 = [int(i.split("+")[1].split()[0]) for i in lines if 'p1' in i]
    p2 = [int(i.split("+")[1].split()[0]) for i in lines if 'p2' in i]
    g1s.append(sum(p1))
    g2s.append(sum(p2))
    p1s.extend(p1)
    p2s.extend(p2)
    ts.extend(times)
    for a, b in zip([times[i] for i in range(0, len(times), 2)], [i for i in lines if 'p1' in i]):
        if False or '?' not in b: t.append(a)
    for a, b in zip([times[i] for i in range(1, len(times), 2)], [i for i in lines if 'p2' in i]):
        if False or '?' not in b: t.append(a)
##    plt.subplot(10, 10, i)
##    plt.plot(range(len(times)), times)
##    plt.axis('off')
##
##    for j, bl in enumerate([i for i in lines if 'p1' in i or 'p2' in i]):
##        n = bl.count('?')
##        if n > 0:
##            try:
##                plt.plot([j], [times[j]], ' ry'[n]+'o')
##            except: pass

# plt.show()
import seaborn as sns
sns.set(style="whitegrid")
# ax2 = sns.violinplot(x=g2s)
# ax = sns.violinplot(x=g1s)
ax = sns.violinplot(x=list(map(sum, zip(g1s, g2s))))
plt.show()
