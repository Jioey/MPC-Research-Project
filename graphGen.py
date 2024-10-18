import networkx as nx
import matplotlib.pyplot as plt

'''
Short scripts to generate networkx graph and write its adjacency list to a text file.
'''
def erdos_renyi_to_txt(N:int, p:float):
    G = nx.erdos_renyi_graph(N, p)
    fileName = "graphs/renyi" + str(N) + "-" + str(int(p*100)) + ".txt"
    nx.write_adjlist(G, fileName)
    # Remeber to remove info lines!!

def stochastic_bloc_to_txt(sizes:list[int], p:list[list[float]], i:int):
    G = nx.stochastic_block_model(sizes, p)
    fileName = "graphs/stochastic" + str(i) + ".txt"
    nx.write_adjlist(G, fileName)
    # Remeber to remove info lines!!

if __name__ == "__main__":
    # G = nx.erdos_renyi_graph(50, 0.3)
    # # print(list(G.nodes))
    # subG = G.subgraph([0,1,2,3,4,5]) # gets subgraph

    # fig = plt.figure()
    # gs = fig.add_gridspec(1, 2)
    # ax1 = fig.add_subplot(gs[0, 0])
    # nx.draw(G, ax=ax1)
    # ax2 = fig.add_subplot(gs[0, 1])
    # nx.draw(subG,ax=ax2)
    # plt.show()

    erdos_renyi_to_txt(4000000, 0.5)
    # sizes = [75, 125, 300, 500]
    # p = [[0.1, 0.2, 0.7, 0.4], 
    #      [0.2, 0.4, 0.5, 0.1], 
    #      [0.7, 0.5, 0.2, 0.7], 
    #      [0.4, 0.1, 0.7, 0]]
    # stochastic_bloc_to_txt(sizes, p, 1)