import pandas as pd
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE, LAU_CODES
import matplotlib.pyplot as plt


def display_dataframe(file):
    df = pd.read_csv(file)
    print(df)


def compare_municipalities_confirmed_cases(a="copenhagen", b="aarhus"):

    df = pd.read_csv(COMMUNAL_FILE_TREE.get(
        "bekraeftede_tilfaelde_pr_dag_pr_kommune"), sep=";")
    a = a.capitalize()
    b = b.capitalize()

    a_code = ""
    b_code = ""

    try:
        a_code = int(LAU_CODES.get(a))
    except:
        raise Exception(f"No hits on lau value: {a_code}")

    try:
        b_code = int(LAU_CODES.get(b))
    except:
        raise Exception(f"No hits on lau value: {b_code}")

    #df = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]
    df_a = df[df["Kommune"] == a_code]
    df_b = df[df["Kommune"] == b_code]
    
    
    plt.plot(df_a["Bekraeftede tilfaelde"])
    plt.plot(df_b["Bekraeftede tilfaelde"])
    plt.show()

