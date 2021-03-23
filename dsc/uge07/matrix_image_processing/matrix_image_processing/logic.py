
import numpy as np
from PIL import Image, ImageOps

# https://numpy.org/devdocs/reference/random/generated/numpy.random.rand.html


def generate_noise(custom_dimensions=(32, 32)):
    random_arr = np.random.rand(
        int(custom_dimensions[0]), int(custom_dimensions[1]))
    img = Image.fromarray(np.uint8(random_arr * 255), 'L')
    return img


def generate_greyscale_from_image(file_path, preserve_dimensions=False, use_greyscale=False):
    img = Image.open(file_path)
    if not preserve_dimensions:
        img = img.resize((32, 32))
    if not use_greyscale:
        img = ImageOps.grayscale(img)
    return img


def convolve(file_path, preserve_dimensions=False, use_greyscale=False):
    kernel = np.array([0, 1, 0, 0.5, 1, 0, 0.5, 0, -2])
    kernel.reshape(3, 3)
    img = generate_greyscale_from_image(
        file_path, preserve_dimensions, use_greyscale)
    res = np.convolve(img, kernel)
    print(res)
