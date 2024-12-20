import networkx as nx
import numpy as np
from plot import Plotter

def worker_main(originalG:nx.Graph, G:nx.Graph, rounds:int):
    matchings = []
    threshold = G.number_of_nodes() / 2.0

    # while G is not empty
    while(rounds > 0 and G.number_of_nodes() > 0):
        toRemove = []
        # for all nodes in G
        for node in G.nodes():
            # if degree(node) > threshold
            # print("Node", node, "with degree", G.degree(node), "at threshold", threshold)
            if G.degree(node) > threshold:
                # add to matching (randomly assign)
                # print("Matching nodes...")
                while True:
                    if G.degree(node) == 0: # possible if all neighbors are matched
                        print("No matches found")
                        break
                    
                    randNeighbor = int(np.random.choice(G[node])) # pick random neighbor to match
                    # print("Node", node, "with rand neighbor", randNeighbor)

                    if randNeighbor not in toRemove: # if valid matching
                        # add to match and prepare to remove
                        print("Match found")
                        matchings.append((node, randNeighbor))
                        toRemove.append(node)
                        toRemove.append(randNeighbor)
                        break
                    else:
                        G.remove_edge(node, randNeighbor)

        # remove node and matched neighbor from graph
        # print("Removing nodes...")
        G.remove_nodes_from(toRemove)
        originalG.remove_nodes_from(toRemove)
        
        print("Round", rounds, ", Matching", matchings)

        rounds -= 1
        threshold = threshold / 2
        
    return originalG, G

if __name__ == "__main__":
    plotter = Plotter()
    rounds = 1

    G = nx.erdos_renyi_graph(500, 0.3)
    plotter.plotDegreeDistribution("Initial Degree Distribution", G)

    G1 = worker_main(G, rounds)
    title = "Degree Distribution After " + str(rounds) + " Round(s) of Matching"
    print("Remaining nodes:", G1.number_of_nodes())
    plotter.plotDegreeDistribution(title, G1)
    plotter.show()