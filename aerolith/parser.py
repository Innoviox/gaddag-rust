import ast
from collections import defaultdict
import click

order = lambda s: ''.join(sorted(list(s)))


@click.command()
@click.argument('file', type=click.Path(exists=True))
def main(file):
    words = [i['w'] for i in ast.literal_eval(open(file).read())]

    sevens = defaultdict(list)
    for w in words:
        sevens[order(w)].append(w)

    with open(f"{file.split('.')[0]}-out.txt", "w") as f:
        for (k, v) in sevens.items():
            f.write(f"{k},{' '.join(v)}\n")

main()
