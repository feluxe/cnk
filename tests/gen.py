import sys


try:
    args = sys.argv[1]
except IndexError:
    args = 100


for i in range(6, int(args)):

    print(f"{str(i).ljust(3)}:", end="")
    print("A" * (i - 5), end="")
    print("E")



