with open("testcogo_3000.txt") as f:
    s = f.read().split("#player2 p2 p2")
    L = {}
    for game in s[1:]:
        game = [i for i in game.split("#p")[0].split("\n")[1:-1]
            if i.startswith(">")][:-1]
        words = [i.split()[3] for i in game]
        # L.extend([len(i) for i in words])
        for i in words:
            L[len(i)] = i
raise ValueError

L = [L.count(i) for i in range(1, max(L) + 1)]

import matplotlib.pyplot as plt
ax = plt.bar(range(1, len(L) + 1), L)

plt.title("Distribution of Word Lengths")
plt.xlabel("Length")
plt.ylabel("Count")
plt.show()
