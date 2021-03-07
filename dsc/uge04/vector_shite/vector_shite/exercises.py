# Using Python and numpy, implement a mag(vec) function to return the
# magnitude (length) of a 2-dimensional vector (as a numpy array).
import numpy as np


def mag(values):
    return np.linalg.norm(values)
    
    
def unit(values):
    return np.linalg.norm(values, ord=1)
    
    
def rot90(values):
    return np.array([-values[0], values[1]], int)

