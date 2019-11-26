from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 simon simon
#player2 james james
>simon: BEINPSU 8G PUB +14 14
>james: IINOORV 9C VIRION +21 21
>simon: EINNOSS J8 SONNIES +69 83
>james: GILLNOT E8 T.OLLING +68 89
>simon: ?ACEERS 12A CARE.EsS +76 159"""
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
