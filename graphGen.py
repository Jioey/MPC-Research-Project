import networkx as nx

'''
Short script to generate networkx graph and write its adjacency list to a text file.
'''
G = nx.erdos_renyi_graph(500, 0.9)
# print(list(G.nodes))

subG = G.subgraph([0,1,2,3,4,5]) # gets subgraph
print(subG.edges)