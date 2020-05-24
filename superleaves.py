from string import ascii_uppercase

alph='?   '+ascii_uppercase

with open("superleaves", "rb") as file:
    with open("resources/leaves.txt", "w") as out:
        try:
            while True:
                leave_size = ord(file.read(1))
                leave_bytes = file.read(leave_size)

                leave = ''.join(alph[i - 1] for i in leave_bytes)
                
                int_value_frac, int_value_int = file.read(2)

                int_value = int_value_int * 256 + int_value_frac
                value = round((int_value / 256) - 128, 2)

                out.write(f"{leave} {value}\n")
        except EOFError as e:
            print(e)
