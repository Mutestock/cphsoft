import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sklearn.cluster import KMeans
from sklearn import metrics
from scipy.spatial.distance import cdist

DATA_CLUSTERING_PATH = "../resources/data_clustering.txt"

def thing():
    df = pd.read_csv(DATA_CLUSTERING_PATH, header=None)
    print(df.head())
    df.info()
    df.describe()
    print(df.shape)
    df_values = df.to_numpy()
    return df_values


def visualize(numpy_array):
    plt.figure()
    plt.title('Input data')
    
    x_min = numpy_array[:, 0].min()
    x_max = numpy_array[:, 0].max()
    
    y_min = numpy_array[:, 1].min()
    y_max = numpy_array[:, 1].max()
    
    plt.xlim(x_min -1, x_max+1)
    plt.ylim(y_min -1, y_max +1)
    
    plt.scatter(numpy_array[:,0], numpy_array[:,1], marker='o', facecolors='none', edgecolors='black', s=30)
    plt.show()
    

def distortion(numpy_array):
    distortions = []
    K = range(2,20)
    for k in K:
        model = KMeans(n_clusters=k)
        model.fit(numpy_array)
        distortions.append(sum(np.min(cdist(numpy_array, model.cluster_centers_, 'euclidean'), axis=1) / numpy_array.shape[0]))
    return distortions, K

def visualize_distortion(distortion, k):
    plt.title('Elbow method for optimal k')
    plt.plot(k, distortion, 'bx-')
    plt.xlabel('K')
    plt.ylabel('Distortion')
    plt.show()

    
if __name__ == "__main__":
    #visualize(thing())
    distortions = distortion(thing())
    
    visualize_distortion(distortions[0], distortions[1])
    