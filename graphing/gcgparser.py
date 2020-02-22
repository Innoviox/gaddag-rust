from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: REALAN? H7 pREANAL +64 64
>p2: NYGAMRX 13H .ARYNX +34 34
>p1: AKCSESQ K9 CASK. +28 92
>p2: GMDUIYC 10J G.UDY +22 56
>p2: MICSIOT 9B COMITI.S +65 121
>p1: SETREPA M3 REPASTE. +76 168
>p2: UOAADZO L4 OUZO +47 168
>p1: LNEIRTH C8 H.TLINER +72 240
>p2: AADTOLR 12K .OALA +33 201
>p1: BGWEENG 15A BE.G +27 267"""
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
