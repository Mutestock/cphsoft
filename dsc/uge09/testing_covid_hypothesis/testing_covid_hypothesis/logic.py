import pandas as pd
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE, LAU_CODES, CUSTOM_IMAGES_PATH
import matplotlib.pyplot as plt

# Simple test to see whether or not pandas is capable of reading a specified file.
# Used in cli basic show
# Disregard.


def display_dataframe(file):
    df = pd.read_csv(file)
    print(df)


# Simple comparison of copenhagen and aarhus.
def compare_municipalities_confirmed_cases(a="copenhagen", b="aarhus"):

    # Read a file in pandas and make it a dataframe.
    # The files used in the csv files are separated by semicolons
    df = pd.read_csv(COMMUNAL_FILE_TREE.get(
        "bekraeftede_tilfaelde_pr_dag_pr_kommune"), sep=";")

    # The LAU codes I found were capitalized.
    # I am using those values as keys in a dictionary in aliases.py, so I'm capitalizing.
    a = a.capitalize()
    b = b.capitalize()

    a_code = ""
    b_code = ""

    # Making sure there's a hit. Informing user if there aren't.
    try:
        a_code = int(LAU_CODES.get(a))
    except:
        print(f"{a_code} is not a recognized input value")
        return
    try:
        b_code = int(LAU_CODES.get(b))
    except:
        print(f"{b_code} is not a recognized input value")
        return

    #df = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]
    df_a = df[df["Kommune"] == a_code]
    df_b = df[df["Kommune"] == b_code]

    plt.plot(df_a["Bekraeftede tilfaelde"])
    plt.plot(df_b["Bekraeftede tilfaelde"])
    plt.show()
