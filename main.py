import networkx as nx
import matplotlib.pyplot as plt
import numpy as np
import threading

# my functions
from plot import Plotter
from worker import worker_main

# global variables
N = 1000000 # number of verticies in the graph
N_PARTITIONS = np.sqrt(N) # Number of workers

# main
if __name__ == "__main__":
    print("Greetings world")
    plotter = Plotter(2, 2)

    # read graph
    adjList = {}
    # degrees = []
    # f = open("renyi" + str(N) + "-90.txt", 'r').readlines()
    f = open("graphs/renyi2000-50.txt", 'r').readlines()
    # f = open("graphs/stochastic1.txt", 'r').readlines()
    for line in f:
        line = list(map(int, line.split(" ")))
        adjList[line[0]] = line[1:]

    # read adjList into graph
    G = nx.Graph(adjList) 
    # calculate original graph degree distribution for comparison
    plotter.plotDegreeDistribution("Original Graph Degree-Distribution", G)
        
    # partition graph
    partitions = [[] for i in range(N_PARTITIONS)]
    for node in G.nodes():
        i = np.random.randint(N_PARTITIONS)
        # print(node, i)
        partitions[i].append(node)
        # print(partitions)
    # turn them into nx.Graph objects
    partGraphs = []
    for part in partitions:
        partGraphs.append(G.subgraph(part))
    
    # testing partitions
    # fig = plt.figure()
    # gs = fig.add_gridspec(3, 2)
    # ax0 = fig.add_subplot(gs[:2, :])
    # nx.draw(G, ax=ax0)
    # ax1 = fig.add_subplot(gs[2, 0])
    # nx.draw(partGraphs[0], ax=ax1)
    # ax2 = fig.add_subplot(gs[2, 1])
    # nx.draw(partGraphs[1], ax=ax2)
    # plt.show()

    # send to workers for some rounds (currently emulated iteratively)
    resultGraphs = []
    for i in range(N_PARTITIONS):
        print("Worker", i)
        G, partG = worker_main(G, nx.Graph(partGraphs[i]), 2)
        resultGraphs.append(partG)

    # calculate subgraph sum degrees -- ?
    for i in range(len(resultGraphs)):
        plotter.plotDegreeDistribution("Worker " + str(i) + "'s Resulting Dist", resultGraphs[i])

    plotter.plotDegreeDistribution("Final Distribution", G)
        
    plotter.show()
    # calculate and graph global degree's decrease