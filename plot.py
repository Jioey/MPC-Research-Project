import networkx as nx
import matplotlib.pyplot as plt
import numpy as np

'''
Plotter class for plotting all demo charts
Sampled Code: https://networkx.org/documentation/stable/auto_examples/drawing/plot_degree.html 
'''
class Plotter:
    def __init__(self, nrow:int, ncol:int):
        self.fig = plt.figure()
        self.gs = self.fig.add_gridspec(nrow, ncol)
        self.currRow = self.currCol = 0

    # TODO: Change figure into subplots so a new window isn't created every time
    def plotDegreeDistribution(self, title:str, G:nx.Graph):
        if G.number_of_nodes() > 0:
            # check if currRow and currCol are out of bounds
            if self.currCol >= self.gs.ncols or self.currRow >= self.gs.nrows:
                print("Plot full! Accessing (" + str(self.currRow) + "," + str(self.currCol) + ") on gridspace of (" + str(self.currRow) + "," + str(self.currCol) + "); The data is", list(nx.generate_adjlist(G)))
                return
            
            # Create plt Axe object
            ax = self.fig.add_subplot(self.gs[self.currRow, self.currCol])
            # Create HISTOGRAM
            degrees = list(zip(*list(G.degree)))[1]
            ax.hist(degrees, density=False, color="skyblue", bins=np.arange(min(degrees), max(degrees)))

            ax.set_title(title)
            ax.set_xlabel("Degree")
            ax.set_ylabel("# of Nodes")

            # Increment current col/row number
            if self.currCol < self.gs.ncols-1:
                self.currCol += 1
            else:
                self.currCol = 0
                self.currRow += 1
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
        plt.tight_layout()
        plt.show()

if __name__ == "__main__":
    # testing for this class
    plotter = Plotter(2,2)
    # G = nx.erdos_renyi_graph(500, 0.8)
    G = nx.gnp_random_graph(100, 0.02, seed=10374196)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions1", G)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions2", G)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions3", G)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions4", G)
    plotter.plotDegreeDistribution("Test Plotter: Degree Distributions4", G)
    plotter.show()
