s="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: RSENRLE H6 ERN +6 6
>p2: ZHRNCWO 6F WR.NCH +28 28
>p1: SRLEEUR 6F ......ER +16 22
>p2: ZOLEFIE M1 FOZIE. +36 64
>p1: SLEURIP K2 PLUS.IER +76 98
>p2: LEB?AAN J8 BALANcE +68 132
>p1: RILRU?Y 15H fURRILY +89 187
>p2: XOASOMG 1M .OX +39 171
>p1: VDAKTIE 8J ..AKED +42 229
>p2: ASOMGTO 14L GAM +31 202
>p1: VTIIION 5C VINO +19 248
>p2: SOTOLTT 4D LOTTO +24 226
>p1: TIIDNBN 11I B.NDIT +18 266
>p2: STDESAO N1 .DAS +29 255
>p1: INHIGAU 13G HUI. +11 277
>p2: TESOMQA 4J Q.A.. +28 283
>p1: NIGAIVW 9I V..IA +22 299
>p2: TESOMPU 10M MU +20 303
>p1: NGIWOGT O10 GOWN +28 327
>p2: TESOPEA 12J .AE +14 317
>p1: IGTEYCE 7D GEY +17 344
>p2: TSOPEIA 12A OPIATES +75 392
>p1: ITCEEOE A11 C.OEE +21 365
>p2: SJTDF C11 D.F +14 406
>p1: ITE 14E TIE +11 376
>p1:  (SJT) +20 396"""

bag = []
w1, lr1, w2, lr2 = "", [], "", []
lines = s.split("\n")[3:]
lines = [lines[i:i+2] for i in range(0, len(lines), 2)]
for (l1, l2) in lines:
    _, r1, _, nw1, *_ = l1.split()
    r1 = list(r1)
    for i in lr1:
        print(i, r1)
        r1.remove(i)
    bag.extend(r1)
    lr1 = r1[:]
    for i in nw1:
        lr1.remove(i)
    w1 = nw1

    _, r2, _, nw2, *_ = l2.split()
    r2 = list(r2)
    for i in w2:
        r2.remove(i)
    bag.extend(r1)
    w2 = nw2

print(bag)
    
