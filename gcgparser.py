from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: SREOIFI H7 IF +10 10
>p2: ASRENRX G5 RAX +35 35
>p1: SREOIIA 7F I..A +12 22
>p2: SENRDEZ F6 Z.N +63 98
>p1: SREOIIA E8 OI +4 26
>p2: SERDEWE D7 RESEWED +80 178
>p1: SREIADN J1 RANDIES +74 100
>p2: VIEAOKI 1H KO.AI +30 208
>p1: ESTOUTA 11A OUT.ASTE +72 172
>p2: VEITWNR H10 R.VIEW +48 256
>p1: TUEONNB 3F UNBO.NET +74 246
>p2: TNLPNSQ 13G Q. +21 277
>p1: RETP?RY A10 R.PERY +42 288
>p2: TNLPNSO 5I P.NON +14 291
>p1: T?ADTEE 8H .ATTEnED +97 385
>p2: TLSHGCM 9D ..GH +26 317
>p1: ILADHBE 14F HI. +25 410
>p2: TLSCMAY 4L CALM +35 352
>p1: LADBEGI H6 G.. +20 430
>p2: TSYOLUJ 4D JUT +22 374
>p1: LADBEIG 2B DEBAG +34 464
>p2: SYOLOEU 1A YO +18 392
>p1: LI?VOFC O1 FIL. +27 491
>p2: SLOEULA 12H .ALUE +16 408
>p1: ?VOCMI 15D VIM +19 510
>p2: SOL 5C LO +11 419
>p1: ?OC H1 .O. +9 519
>p2: S I12 .S +16 435
>p2:  (?C) +6 441"""
for move in gcg.split("\n")[3:]:
    player, rack, pos, word, score, total = move.split()
    if pos[0].isdigit():
        direc = "utils::Direction::Across"
        row, col = pos[:-1], pos[-1]
    else:
        direc = "utils::Direction::Down"
        col, row = pos[0], pos[1:]
    col = au.index(col)

    print('board.play_word(utils::Position { row: %s, col: %s }, String::from("%s"), %s, true);' % (row, col, word, direc))
