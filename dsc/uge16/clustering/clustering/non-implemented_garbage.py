import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


# https://www.javatpoint.com/k-means-clustering-algorithm-in-machine-learning

# How does the K-Means Algorithm Work?
# The working of the K-Means algorithm is explained in the below steps:
# 
# Step-1: Select the number K to decide the number of clusters.
# 
# Step-2: Select random K points or centroids. (It can be other from the input dataset).
# 
# Step-3: Assign each data point to their closest centroid, which will form the predefined K clusters.
# 
# Step-4: Calculate the variance and place a new centroid of each cluster.
# 
# Step-5: Repeat the third steps, which means reassign each datapoint to the new closest centroid of each cluster.
# 
# Step-6: If any reassignment occurs, then go to step-4 else go to FINISH.
# 
# Step-7: The model is ready.


if __name__ == '__main__':
    df = pd.read_csv("../resources/congress.csv")
    x = df.iloc[:, [3, 4]].values      
    print(x)
