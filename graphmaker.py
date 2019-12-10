import matplotlib.pyplot as plt

for i, game in enumerate(open("test.txt").read().split("#character-encoding UTF-8")[1:], start=1):
    lines = game.split("\n")[3:]
    times = [int(i.split()[-1]) for i in lines if 'Time' in i]
    p1 = [int(i.split("+")[1].split()[0]) for i in lines if 'p1' in i]
    p2 = [int(i.split("+")[1].split()[0]) for i in lines if 'p2' in i]

    plt.subplot(2, 6, i)
    plt.plot(range(len(times)), times)
    for j, bl in enumerate([i for i in lines if 'p1' in i or 'p2' in i]):
        n = bl.count('?')
        if n == 1:
            plt.plot([j], [times[j]], 'ro')
        elif n == 2:
            plt.plot([j], [times[j]], 'yo')

plt.show()
