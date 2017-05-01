import csv
import math
import numpy as np

m = None
t = None

with open("0.csv") as csvfile:
    reader = csv.reader(csvfile, delimiter=",")
    for row in reader:
        d = [float(x) for x in row]

        mi = np.array([
            [
                d[0],
                d[1],
                2 * d[0] * math.cos(d[5]) + d[1] * math.cos(d[5]) - 2 * d[2] * d[3] * math.sin(d[5]) - d[3] * d[3] * math.sin(d[5]),
                d[2],
                0,
                math.copysign(1, d[2]),
                0],
            [
                0,
                d[0] + d[1],
                d[0] * math.cos(d[5]) + d[2] * d[2] * math.sin(d[5]),
                0,
                d[3],
                0,
                math.copysign(1, d[3])]
            ])

        ti = np.array([[d[6]], [d[7]]])

        if m is None:
            m = mi
            t = ti
        else:
            m = np.vstack((m, mi))
            t = np.vstack((t, ti))

rho = np.linalg.inv(m.T.dot(m)).dot(m.T.dot(t))
print(rho)
