
import numpy as np
from PIL import Image

# https://numpy.org/devdocs/reference/random/generated/numpy.random.rand.html
def generate_noise(custom_dimensions=(32,32)):
    random_arr = np.random.rand(custom_dimensions[0],custom_dimensions[1])
    img = Image.fromarray(np.uint8(random_arr * 255), 'L')
    return img
    
    
def generate_greyscale_from_image(file_path, preserve_dimensions=False, use_greyscale=True):
    img = Image.open(file_path)
    if not preserve_dimensions:
        img = img.resize((32,32))
    img.show()
    
    
        

