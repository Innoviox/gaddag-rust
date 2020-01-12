from string import ascii_uppercase

def get_words(game):
    game = [i for i in game.split("#p")[0].split("\n")[1:-1]
        if i.startswith(">")][:-1]
    words = [(i.split()[2], i.split()[3]) for i in game]

    board = [['' for i in range(16)] for i in range(16)]
    nw = []
    for (pos, word) in words:
        if pos[0] in ascii_uppercase:
            row = ascii_uppercase.index(pos[0])
            col = int(pos[1:])
            d = 'A'
        else:
            col = int(pos[:-1])
            row = ascii_uppercase.index(pos[-1])
            d = 'D'
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

with open("txt/gcgtest.txt") as f:
    s = f.read().split("#player2 p2 p2")
    for game in s[1:]:
        print(get_words(game))
        input()
        
