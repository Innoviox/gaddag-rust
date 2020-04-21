import tkinter as tk
import subprocess
import base64

puzzle = str(base64.b64decode(subprocess.check_output(["./target/release/gaddag-rust", "puzzle"]).split()[-1]))[2:-1].replace(r"\n", "\n")
board, rack, *moves = puzzle.split()
board = "\n".join([board[i:i+15] for i in range(0, len(board), 15)])
