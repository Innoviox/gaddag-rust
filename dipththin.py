#This program speeds up the CPU by hundreds of times. 
import os, sys
from itertools import permutations as p
from string import ascii_uppercase as a_u

class AlreadyRunError(BaseException):
    pass

def makedir():
    dir_path = os.path.dirname(os.path.realpath(__file__))
    dir_path += "/resources/"
    if not os.path.exists(dir_path):
        os.makedirs(dir_path)
    else:
        raise AlreadyRunError
def makenospd():
    ospd = open("nwl18.txt").read().split("\n")
    global nospd
    nospd = []
    for word in ospd:
        nospd.append(word.strip())
def setup():
    print("Writing...")
    diphths = [["".join(i) for i in p(list(a_u), 2)] + [j*2 for j in a_u]]
    for i in diphths[0]:
            with open("resources/"+i+".txt", "w"):
                pass

            dfile = open("resources/"+i+".txt", "w")

            for word in nospd:
                if word[:2] == i:
                    pass
                    dfile.write(word)
                    dfile.write("\n")

            #print("Completed", diphths[0].index(i), "out of", str(len(diphths[0]))+".", file=sys.stderr)

            dfile.close()
    print("Complete.")
makedir()
makenospd()
setup()
