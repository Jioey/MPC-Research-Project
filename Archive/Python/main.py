# import networkx as nx
# import matplotlib.pyplot as plt
import numpy as np
import hashlib
import time
# import threading

# my functions
# from plot import Plotter
# from worker import worker_main
from BinaryHeap import BinaryHeap

# i, j order matters!
def getHash(i, j):
    s = seed + str(i) + str(j)
    key = hashlib.md5(s.encode()) # generate md5
    return int.from_bytes(key.digest(), "big") # convert hash to int and return

# global variables
N = 100 # Number of workers
N_V = np.pow(N, 2) # number of verticies in the graph
T = N / 2 # Initial threashold
seed = "seed?"

# main
if __name__ == "__main__":
    print("Starts with N=" + str(N) + " and N_V=" + str(N_V))
    start_time = time.time()

    # STEP 1: Create global degrees list
    degrees = [0 for i in range(N_V)]

    for i in range(N_V):
        for j in range(i+1, N_V):
            keyInt = getHash(i, j)

            # if even, increment degree for both nodes
            # note: change mod number to alter edge probability; e.g. 3 for p=0.3
            if keyInt % 2 == 0:
                degrees[i] += 1
                degrees[j] += 1
    
    # STEP 1.1: Convert to heap
    globalHeap = BinaryHeap(2 * N_V)
    for i in range(len(degrees)):
        globalHeap.insert((i, degrees[i]))
    
    step1_time = time.time()
    print(f"STEP 1 Complete: {step1_time - start_time} seconds")

    # STEP 2: Partition graph
    partitions = [[] for i in range(N)]
    for node in range(N_V):
        i = np.random.randint(N)
        partitions[i].append(node) # randomly assign are not equal sizes, fix later?
    print("STEP 2 Complete")

    # STEP 3: MATCH THEMM
    # for machine in range(N):
    #     localNodes = partitions[machine]
    #     # STEP 3.1: Find out local degree
    #     # 3.1.1 Initialize dictionary
    #     localDegrees = {}
    #     for node in localNodes:
    #         localDegrees[node] = 0
    #     # 3.1.2 Count
    #     for i in range(len(localNodes)):
    #         for j in range(i+1, len(localNodes)):
    #             keyInt = getHash(localNodes[i], localNodes[j])

    #             # if even, increment degree for both nodes
    #             if keyInt % 2 == 0:
    #                 localDegrees[localNodes[i]] += 1
    #                 localDegrees[localNodes[j]] += 1
        
    #     # 3.1.3 Convert to heap
    #     localHeap = BinaryHeap(2 * len(localNodes))
    #     for key in localDegrees.keys():
    #         localHeap.insert((key, localDegrees[key]))

    #     # STEP 3.2: Do 1 round of eliminations# sort by largest degree to smallest
        
    #     # 3.2.1 Find matching and remove matched
    #     toRemove = []
    #     while localHeap.size != 0:


    #     for r in toRemove:
            


    
    end_time = time.time()
    print(f"Runtime: {end_time - start_time} seconds")






        
        



    # for node in G.nodes():
    #     i = 
    #     # print(node, i)
    #     partitions[i].append(node)
    #     # print(partitions)
    # # turn them into nx.Graph objects
    # partGraphs = []
    # for part in partitions:
    #     partGraphs.append(G.subgraph(part))

    # send to workers for some rounds (currently emulated iteratively)
    # resultGraphs = []
    # for i in range(N_PARTITIONS):
    #     print("Worker", i)
    #     G, partG = worker_main(G, nx.Graph(partGraphs[i]), 2)
    #     resultGraphs.append(partG)

    # # calculate subgraph sum degrees -- ?
    # for i in range(len(resultGraphs)):
    #     plotter.plotDegreeDistribution("Worker " + str(i) + "'s Resulting Dist", resultGraphs[i])

    # plotter.plotDegreeDistribution("Final Distribution", G)
        
    # plotter.show()
    # calculate and graph global degree's decrease