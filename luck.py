worths = dict(i.strip().split() for i in open("worths").readlines())


def calc_luck(tiles):
    return sum(map(float, [worths[i.upper()] for i in tiles]))

print('Simon:', calc_luck('BOOUPHAZELDCOINEDWIGSFXOWTRIORNSJEDGL'))
print('Nate: ', calc_luck('HINGEFAUNADITBISEMAUVAONYEICATEQUAPOMEYERANK'))
