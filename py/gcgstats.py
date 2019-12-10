from statistics import mean, median

time=lambda s: sum(map(int, [i.split()[-1] for i in s.split("\n") if 'Time' in i]))
s1s, s2s, ts = [], [], []
with open("../test.txt") as file:
    s = file.read().split("#player2 p2 p2")
    for game in s[1:]:
        game = game.split("#p")[0].split("\n")
        s1 = int([i for i in game if i.startswith(">p1")][-1].split()[-1])
        s2 = int([i for i in game if i.startswith(">p2")][-1].split()[-1])
        ts.append(time('\n'.join(game)))
        s1s.append(s1)
        s2s.append(s2)

print(mean(s1s), mean(s2s), mean(ts), len(s1s), median(s1s), median(s2s), median(ts))

