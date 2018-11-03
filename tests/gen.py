

import sys


try:
    args = sys.argv[1]
except IndexError:
    args = 100


for i in range(6, int(args)):

    print(f"{str(i).ljust(3)}:", end="")
    # print(",".join([str(n) for n in range(0, i)]), end="")
    print("A" * (i - 5), end="")
    print("E")



