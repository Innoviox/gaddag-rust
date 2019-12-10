import matplotlib.pyplot as plt
p1s, p2s, ts = [], [], []
for i, game in enumerate(open("test.txt").read().split("#character-encoding UTF-8")[1:], start=1):
    lines = game.split("\n")[3:]
    times = [int(i.split()[-1]) for i in lines if 'Time' in i]
    p1 = [int(i.split("+")[1].split()[0]) for i in lines if 'p1' in i]
    p2 = [int(i.split("+")[1].split()[0]) for i in lines if 'p2' in i]
    p1s.extend(p1)
    p2s.extend(p2)
    ts.extend(times)
    plt.subplot(10, 10, i)
    plt.plot(range(len(times)), times)
    plt.axis('off')

    for j, bl in enumerate([i for i in lines if 'p1' in i or 'p2' in i]):
        n = bl.count('?')
        if n > 0:
            try:
                plt.plot([j], [times[j]], ' ry'[n]+'o')
            except: pass

plt.show()
