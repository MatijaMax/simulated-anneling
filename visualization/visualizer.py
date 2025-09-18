# visualizer.py
import matplotlib.pyplot as plt
import seaborn as sns
import glob
import numpy as np

sns.set(style="whitegrid")
ns, seq_times, par_times, speedups, seq_dists, par_dists = [], [], [], [], [], []

for fname in sorted(glob.glob("../sa/performance_n_*.txt")):
    with open(fname) as f:
        data = f.read()
        n = int(data.split("n =")[1].split()[0])
        seq_time = float(data.split("Sequential time:")[1].split()[0])
        par_time = float(data.split("Parallel time:")[1].split()[0])
        speedup = float(data.split("Speedup:")[1].split("x")[0])
        seq_dist = int(data.split("Sequential distance:")[1].split()[0])
        par_dist = int(data.split("Parallel distance:")[1].split()[0])
        
        ns.append(n)
        seq_times.append(seq_time)
        par_times.append(par_time)
        speedups.append(speedup)
        seq_dists.append(seq_dist)
        par_dists.append(par_dist)


sorted_data = sorted(zip(ns, seq_times, par_times, speedups, seq_dists, par_dists))
ns, seq_times, par_times, speedups, seq_dists, par_dists = map(list, zip(*sorted_data))

# TIMES
x = np.arange(len(ns)) 
width = 0.35 

plt.figure(figsize=(10,5))
plt.bar(x - width/2, seq_times, width, label="Sequential", color="blue")
plt.bar(x + width/2, par_times, width, label="Parallel", color="red")

plt.xlabel("n")
plt.ylabel("Time (s)")
plt.title("Sequential vs parallel execution time")
plt.xticks(x, ns)
plt.legend()
plt.grid(axis='y', linestyle='--', alpha=0.5)
plt.tight_layout()
plt.savefig("performance_times.png", dpi=300)
print("Saved performance_times.png")

# SPEED UP/SLOW DOWN
plt.figure(figsize=(10,5))
plt.plot(ns, speedups, marker='o', color='green')
plt.xlabel("n")
plt.ylabel("Speed up/slow down (seq_time / par_time)")
plt.title("Parallel speed up/slow down")
plt.grid(True)
plt.tight_layout()
plt.savefig("performance_speedup.png", dpi=300)
print("Saved performance_speedup.png")

# DISTANCES
plt.figure(figsize=(10,5))
plt.plot(ns, seq_dists, marker='o', label="Sequential distance")
plt.plot(ns, par_dists, marker='o', label="Parallel distance")
plt.xlabel("n")
plt.ylabel("Distance")
plt.title("Sequential vs parallel distance")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("performance_distance.png", dpi=300)
print("Saved performance_distance.png")
