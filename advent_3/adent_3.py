import re
import collections

def mul(a,b):
    return a*b


locations = {0:True}

with open("input.txt", "r") as f:
    content = f.read()

length = len(content)

for start in re.finditer("do\(\)", content):
    locations[start.start()] = True

for end in re.finditer("don\'t\(\)", content): 
    locations[end.start()] = False

results = re.finditer("mul\([0-9]{1,3},[0-9]{1,3}\)", content)


orderedLocations = collections.OrderedDict(sorted(locations.items()))

total = 0
allowed = True

for res in results:
    for key in orderedLocations.keys():
        if (key > res.start()):
            break
        allowed = orderedLocations[key]
    if (allowed):
        val = eval(res.group())
        total += val
    else:
        print("Not allowed")

print(total)
