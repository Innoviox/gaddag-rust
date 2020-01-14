import statistics as st
import matplotlib.patches as mpatches

leaves = {i.split()[0]: float(i.split()[-1]) for i in open("resources/leaves.txt").readlines()}

with open("testcogo_3000.txt") as f:
    s = f.read().split("#player2 p2 p2")
    # e = [0] * 50
    e = []
    for game in s[1:]:
        game = [i for i in game.split("#p")[0].split("\n")[1:-1]
            if i.startswith(">")][:-1]

        for idx, play in enumerate(game):
            if '+0' in play:
                e.append(idx)
                # e[idx] += 1
        # e.extend(filter(bool,
        #                 [leaves.get(''.join(sorted(i.split()[2][1:])), None)
        #           for i in game if '+0' in i]))
##        for idx, play in enumerate(game):
##            if '+0' in play:
##                if val := leaves.get(''.join(sorted(play.split()[2][1:]))):
##                    e.append(val)
##                    try:
##                        rack = ''.join(sorted(game[idx + 2].split()[1]))
##                        after.append(st.mean([leaves[rack[:i] + rack[(i + 1):]] for i in range(7)]))
##                    except IndexError:
##                        pass
##                    


import matplotlib.pyplot as plt
# import seaborn as sns
# e = e[:e.index(0)]
# plt.bar(range(len(e)), e)
plt.hist(e)
##ax=sns.violinplot(x=e)
##a1=sns.violinplot(after, color='r')
##
##plt.title("Distribution of Leaves Before and After Exchange")
##plt.xlabel("Evaluation")
##plt.ylabel("Count")
##
##red_patch = mpatches.Patch(color='red', label='Before')
##blue_patch = mpatches.Patch(color='blue', label='After')
##
##plt.legend(title='Legend', loc='upper left', handles=(red_patch, blue_patch))

plt.show()
