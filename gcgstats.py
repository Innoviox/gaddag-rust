from statistics import mean

s1s, s2s = [], []
with open("test.txt") as file:
    s = file.read().split("#player2 p2 p2")
    for game in s[1:]:
        game = game.split("#")[0].split("\n")
        s1 = int([i for i in game if i.startswith(">p1")][-1].split()[-1])
        s2 = int([i for i in game if i.startswith(">p2")][-1].split()[-1])
        s1s.append(s1)
        s2s.append(s2)

print(mean(s1s), mean(s2s))

