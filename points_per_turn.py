from statistics  import mean, median
from collections import defaultdict

turns = defaultdict(list)

with open("testcogo_3000.txt") as f:
    s = f.read().split("#player2 p2 p2")
    for game in s[1:]:
        game = [i for i in game.split("#p")[0].split("\n")[1:-1]
                if i.startswith(">")][:-1]
        scores = [[int(j[1:]) for j in i.split() if '+' in j][0] for i in game]
        for i, j in enumerate(scores):
            turns[i].append(j)

mean_turns = list(map(sum, turns.values()))
median_turns = list(map(median, turns.values()))
games = list(map(len, turns.values()))

import matplotlib.pyplot as plt

plt.plot(list(range(len(mean_turns))), mean_turns)
# plt.plot(list(range(len(mean_turns))), games)
plt.xlabel("Turn")
plt.ylabel("Mean Score")
plt.title("Score over Time")
# plt.plot(list(range(len(median_turns))), median_turns)
plt.show()
        
        
    
