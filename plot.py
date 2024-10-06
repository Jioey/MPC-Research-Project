import networkx as nx
import matplotlib.pyplot as plt
import numpy as np

'''
Plotter class for plotting all demo charts
Sampled Code: https://networkx.org/documentation/stable/auto_examples/drawing/plot_degree.html 
'''
class Plotter:
    figureCounter = 0

    # TODO: Change figure into subplots so a new window isn't created every time
    def plotDegreeDistribution(self, title:str, G:nx.Graph):
        if G.number_of_nodes() > 0:
            plt.figure(self.figureCounter)

            # Create a gridspec for adding subplots of different sizes
            plt.title(title)
            plt.xlabel("Degree")
            plt.ylabel("# of Nodes")

            degrees = list(zip(*list(G.degree)))[1]
            plt.hist(degrees, density=False, color="skyblue", bins=np.arange(min(degrees), max(degrees)))

            self.figureCounter += 1
        else:
            print("Graph is empty!")

    # def getBins(self, G:nx.Graph): # broken bc matplotlib does not support non-linear bin sizes
    #     bins = []
    #     n = G.size()
    #     for i in range(int(np.sqrt(G.size()))): 
    #         bins.append(int(n))
    #         n /= 2

    #     return bins
    
    def show(self):
        plt.show()

if __name__ == "__main__":
    # testing for this class
    plotter = Plotter()
    # G = nx.erdos_renyi_graph(500, 0.8)
    G = nx.gnp_random_graph(100, 0.02, seed=10374196)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions", G)
    plotter.show()
