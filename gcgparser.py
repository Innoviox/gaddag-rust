from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 simon simon
#player2 james james
>simon: EIPRSS H8 SPIRES +18 18
>james: AHINTTU G13 IN +6 6
>simon: EHN 15E HEN +9 27
>james: BE 13F B..E +7 13
>simon: MU I13 .MU +5 32
>james: AT 15I .TA +3 16
>simon: AENOS 11H .EASON +12 44
>james: ACHO M11 .ACHO +20 36
>simon: N 15M .N +2 46"""
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
