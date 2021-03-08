import re
import math
from collections import Counter


def file_to_arr(file):
    with open(file) as file_reader:
        content = file_reader.read().splitlines()
    content_split = []
    for line in content:
        for line_split in line.split():
            content_split.append(re.sub("[,:]","", line_split))
    return content_split


# Yoinked from https://stackoverflow.com/questions/15173225/calculate-cosine-similarity-given-2-sentence-strings
def get_cosine(vec1, vec2):
    intersection = set(vec1.keys()) & set(vec2.keys())
    numerator = sum([vec1[x] * vec2[x] for x in intersection])

    sum1 = sum([vec1[x] ** 2 for x in list(vec1.keys())])
    sum2 = sum([vec2[x] ** 2 for x in list(vec2.keys())])
    denominator = math.sqrt(sum1) * math.sqrt(sum2)

    if not denominator:
        return 0.0
    else:
        return float(numerator) / denominator


def read_files(files):
    arr_a = Counter(file_to_arr(files[0]))
    arr_b = Counter(file_to_arr(files[1]))
    arr_c = Counter(file_to_arr(files[2]))
    
    shite = {
        "a-b": get_cosine(arr_a, arr_b),
        "a-c": get_cosine(arr_a, arr_c),
        "b-c": get_cosine(arr_b, arr_c)
    }
    
    shite = dict(sorted(shite.items(), key=lambda item: item[1]))
    match = list(shite.items())[-1]
    print(f"Best match was {match[0]} at {match[1]}")

    
    