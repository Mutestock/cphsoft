# Knn or k nearest neighbour is supposedly the simplest machine learning algorithm.
# d(xi,xl)=sqrt((xi1−xl1)²+(xi2−xl2)²+...+(xip−xlp)²)
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
# https://scikit-learn.org/stable/modules/generated/sklearn.datasets.load_iris.html
from sklearn.datasets import load_iris
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.neighbors import KNeighborsClassifier
from sklearn.metrics import classification_report


def knn_thing():
    iris = load_iris()
    x = iris.data
    y = iris.target
    
    x_train, x_test, y_train, y_test = train_test_split(x,y, test_size = 0.4)
    
    scaler = StandardScaler()
    scaler.fit(x_train)
    
    x_train = scaler.transform(x_train)
    x_test = scaler.transform(x_test)
    
    classifier = KNeighborsClassifier(n_neighbors = 8)
    classifier.fit(x_train, y_train)
    
    y_pred = classifier.predict(x_test)
    print(classification_report(y_test, y_pred))
    print(classifier.score(x_test, y_test))
    
if __name__ == '__main__':
    knn_thing()