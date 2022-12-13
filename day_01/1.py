import pathlib

lines = open("1.txt").readlines()
lines = [l.replace("\n", "") for l in lines]

totals = []
total = 0

for line in lines:
    if line == "":
        totals.append(total)
        total = 0
    else:
        total += int(line)

totals.append(total)

print(max(totals))

print(totals)
print(sum(sorted(totals)[-3:]))
