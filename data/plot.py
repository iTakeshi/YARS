import csv
import math
import sys

import numpy as np
import matplotlib.pyplot as plt

y1 = []
with open(sys.argv[1]) as csvfile:
    reader = csv.reader(csvfile, delimiter=",")
    for row in reader:
        d = [float(x) for x in row]
        y1.append(d[10])

y2 = []
with open(sys.argv[2]) as csvfile:
    reader = csv.reader(csvfile, delimiter=",")
    for row in reader:
        d = [float(x) for x in row]
        y2.append(d[2])

l = min(len(y1), len(y2))
x = [x for x in range(0, l)]

plt.plot(x, y1[:l], marker=None, label="true")
plt.plot(x, y2[:l], marker=None, label="est")
plt.legend()
plt.show()
