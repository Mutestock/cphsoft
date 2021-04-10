import pandas as pd
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE, LAU_CODES, CUSTOM_IMAGES_PATH, CUSTOM_EXCEL_PATH
import matplotlib.dates as mdates
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import os

# Simple test to see whether or not pandas is capable of reading a specified file.
# Used in cli basic show
# Disregard.


def display_dataframe(file):
    df = pd.read_csv(file)
    print(df)
    
def get_dataframes(a,b):
    
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
    
    return (df_a, df_b)

def mean(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False):
    df_a, df_b = (get_dataframes(a,b))


# Simple comparison of copenhagen and aarhus.
def compare_municipalities_confirmed_cases(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False):

    df_a, df_b = (get_dataframes(a,b))
    
    fig, ax = plt.subplots()
    myLocator = mticker.MultipleLocator(30)
    ax.xaxis.set_major_locator(myLocator)
    
    ax.plot(df_a["Dato"], df_a["Bekraeftede tilfaelde"], '-', label=a)
    ax.plot(df_b["Dato"], df_b["Bekraeftede tilfaelde"], '-', label=b)
    ax.set_title(f'Bekræftede tilfælde pr dag pr kommune - {a} vs {b}')
    ax.legend([f'{a}',f'{b}'])
    #ax.xaxis_date()
    fig.autofmt_xdate()
    plt.xlabel("Date")
    plt.ylabel("Confirmed cases per day")
    if print_file:
        plt.savefig(CUSTOM_IMAGES_PATH+f'{a}_{b}_bekraeftede_tilfælde_pr_dag_pr_kommune.png')
    if excel:    
        df_ab = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]
        if not os.path.exists(CUSTOM_EXCEL_PATH):
            os.makedirs(CUSTOM_EXCEL_PATH)
        with pd.ExcelWriter(CUSTOM_EXCEL_PATH + f"{a}_{b} bekraeftede tilfælde pr dag pr kommune.xlsx") as writer:    
            df_ab.to_excel(writer, sheet_name =f"{a}_{b} bekraeftede tilfælde pr dag pr kommune")
            df_a.to_excel(writer, sheet_name =f"{a} bekræftede tilfælde pr dag pr kommune")
            df_b.to_excel(writer, sheet_name =f"{b} bekræftede tilfælde pr dag pr kommune")
    if show:
        plt.show()
