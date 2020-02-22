from string import ascii_uppercase
from collections import defaultdict

def get_words(game):
    game = [i for i in game.split("#p")[0].split("\n")[1:-1]
        if i.startswith(">")][:-1]
    words = [(i.split()[2], i.split()[3]) for i in game]

    board = [['' for i in range(16)] for i in range(16)]
    nw = []
    for (pos, word) in words:
        try:
            if pos[0] in ascii_uppercase:
                row = ascii_uppercase.index(pos[0])
                col = int(pos[1:])
                d = 'A'
            else:
                col = int(pos[:-1])
                row = ascii_uppercase.index(pos[-1])
                d = 'D'
        except ValueError:
            continue
        n = ''
        for l in word:
            if l == '.':
                n += board[row][col]
            else:
                n += l
                board[row][col] = l
            if d == 'D':
                row += 1
            else:
                col += 1
        nw.append(n)
    return nw

x = []

looks = {i: defaultdict(int) for i in 'QJXZKWVH'} # 'QJXKZVWHCPM'} # QJXKZVWHCPM

with open("testcogo_3000.txt") as f:
    s = f.read().split("#player2 p2 p2")
    for game in s[1:]:
        words = get_words(game)

        for word in words:
            for letter in looks:
                if letter in word:
                    looks[letter][word.upper()] += 1

import matplotlib.pyplot as plt
import numpy as np

for letter in looks:
    words = looks[letter]
    sort = list(reversed(sorted(words, key=words.get)))
    x = list(range(len(words)))
    y = [words[sort[0]]]
    for i in sort[1:]:
        y.append(words[i] + y[-1])
    y = np.array(y) / y[-1]
        
    plt.plot(x, y, label=letter)
plt.legend()
plt.xlabel("Unique Words")
plt.ylabel("Proportion of Total Words")
plt.title("Amount of Words to Know")
plt.show()
