from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: ALARGRY H8 GLARY +26 26
>p2: IOEFDPO G10 POOF +24 24
>p1: AROEAMA I9 AMA +19 45
>p2: IEDIUEQ J5 EQUID +39 63
>p1: ROEAEGD K1 OGEED +20 65
>p2: IEAUTNA 2J I.UANA +18 81"""
for move in gcg.split("\n")[3:]:
    player, rack, pos, word, score, total = move.split()
    if pos[0].isdigit():
        direc = "utils::Direction::Across"
        row, col = pos[:-1], pos[-1]
    else:
        direc = "utils::Direction::Down"
        col, row = pos[0], pos[1:]
    row = str(int(row) - 1)
    col = au.index(col)

    print('board.play_word(utils::Position { row: %s, col: %s }, String::from("%s"), %s, true);' % (row, col, word, direc))
