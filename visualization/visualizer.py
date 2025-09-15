import matplotlib.pyplot as plt
import seaborn as sns

sns.set(style="whitegrid") 

def load_tour(filename):
    """Load tour from TXT file and extract city names and total distance."""
    with open(filename) as f:
        lines = [line.strip() for line in f if line and not line.startswith("Total distance")]
        cities = [line.split(",")[0] for line in lines]  
    total_distance = 0
    with open(filename) as f:
        for line in f:
            if line.startswith("Total distance"):
                total_distance = int(line.split(":")[1].strip())
                break
    return cities, total_distance

seq_cities, seq_total = load_tour("../sa/tour_seq.txt")
par_cities, par_total = load_tour("../sa/tour_par.txt")

seq_y = [1] * len(seq_cities)
par_y = [0] * len(par_cities)

plt.figure(figsize=(15, 3))

palette = sns.color_palette("Set1", n_colors=2)

plt.plot(range(len(seq_cities)), seq_y, marker='o', label=f'Sequential (Total {seq_total})', color=palette[0])
plt.plot(range(len(par_cities)), par_y, marker='o', label=f'Parallel (Total {par_total})', color=palette[1])

for i, city in enumerate(seq_cities):
    plt.text(i, seq_y[i]+0.05, city, rotation=45, ha='right', fontsize=8, color=palette[0])

for i, city in enumerate(par_cities):
    plt.text(i, par_y[i]-0.05, city, rotation=45, ha='right', fontsize=8, color=palette[1])

plt.yticks([0,1], ["Parallel","Sequential"])
plt.xticks([]) 
plt.legend()
plt.grid(axis='x', linestyle='--', alpha=0.5)
sns.despine()
plt.tight_layout()
plt.savefig("seq_vs_par.png", dpi=300)
print("Plot saved as seq_vs_par.png")
