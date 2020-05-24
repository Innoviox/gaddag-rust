def word_has(word, subs):
    return any(map(lambda s: all(i in word for i in s), subs))

def useful(word):
    return len(word) < 6 or (len(word) in [7, 8] and word_has(word, ['RETINA', 'SATINE', 'SATIRE']))

with open("new.txt", "w") as out, open("nwl18.txt") as new_dictionary:
    for word in new_dictionary.readlines():
        if useful(word) and word not in open(f"resources/{word[:2]}.txt").readlines():
            out.write(word)
