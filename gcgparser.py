from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 james james
#player2 simon simon
>james: EIPRS 8G SPIRE +14 14
>simon: ENT 9I NET +11 11
>james: AEILNST L9 SALIENT +70 84
>simon: AOP 15K A.OP +6 17"""
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
