import pandas as pd
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE, LAU_CODES, CUSTOM_IMAGES_PATH, CUSTOM_EXCEL_PATH
import matplotlib.dates as mdates
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import os

# Simple test to see whether or not pandas is capable of reading a specified file.
# Used in cli basic show
# Disregard.

# This is a decorator definition. 
# It contains information about what is called before and after a function is called.
# Used with @basic_bar
def basic_bar(func):
    def inner(*args, **kwargs):
        print(kwargs)
        a = kwargs.get("a").capitalize()
        b = kwargs.get("b").capitalize()

        df, df_c, a_code, b_code = get_dataframes(a, b)

        df_res = func(*args, **kwargs, df_c=df_c)

        data = [df_res[0], df_res[1]]
        parts = [f"{a}", f"{b}"]
        _, ax = plt.subplots()

        ax.bar(parts, data, align='center', alpha=0.5)
        ax.set_title(f"{func.__name__} - {a} - {b}")

        if kwargs.get("print_file"):
            plt.savefig(CUSTOM_IMAGES_PATH +
                        f'{a}_{b}_bekraeftede_tilfælde_kommune_{func.__name__}.png')
        if kwargs.get("excel"):
            # df_ab = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]
            if not os.path.exists(CUSTOM_EXCEL_PATH):
                os.makedirs(CUSTOM_EXCEL_PATH)
            with pd.ExcelWriter(CUSTOM_EXCEL_PATH + f"{a}_{b}_bekraeftede_tilfælde_kommune_{func.__name__}.xlsx") as writer:
                df_res.to_excel(
                    writer, sheet_name=f"{a}_{b} bekraeftede tilfælde pr dag pr kommune_{func.__name__}")
        if kwargs.get("show"):
            plt.show()
    return inner


def display_dataframe(file):
    df = pd.read_csv(file)
    print(df)


def get_dataframes(a, b):

    # Read a file in pandas and make it a dataframe.
    # The files used in the csv files are separated by semicolons
    df = pd.read_csv(COMMUNAL_FILE_TREE.get(
        "bekraeftede_tilfaelde_pr_dag_pr_kommune"), sep=";")

    # The LAU codes I found were capitalized.
    # I am using those values as keys in a dictionary in aliases.py, so I'm capitalizing.

    a_code = ""
    b_code = ""

    # Making sure there's a hit. Informing user if there isn't.

    try:
        a_code = int(LAU_CODES.get(a))
    except:
        print(f"\n\n{a} is not a recognized input value\n\n")
        raise Exception
    try:
        b_code = int(LAU_CODES.get(b))
    except:
        print(f"\n\n{b} is not a recognized input value\n\n")
        raise Exception

    df = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]

    #df = df[(df["Kommune"] == a_code )]

    df_a = df[df["Kommune"] == a_code]
    df_b = df[df["Kommune"] == b_code]
    df_c = df_a.merge(df_b, left_on="Dato", right_on="Dato")
    df_c = df_c.drop(columns=["Kommune_x", "Kommune_y"])
    df_c = df_c.rename(columns={"Bekraeftede tilfaelde_x": f"Bekraeftede tilfaelde {a}",
                                "Bekraeftede tilfaelde_y": f"Bekraeftede tilfaelde {b}"})
    print(df_c)

    return df, df_c, a_code, b_code


@basic_bar
def standard_deviation(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False, df_c=None):
    return df_c.std()

@basic_bar
def mean(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False, df_c=None):
    return df_c.mean()

@basic_bar
def variance(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False, df_c=None):
    return df_c.var()


# Simple comparison of copenhagen and aarhus.
def compare_municipalities_confirmed_cases(a="copenhagen", b="aarhus", excel=False, show=False, print_file=False):
    a = a.capitalize()
    b = b.capitalize()

    df, df_c, a_code, b_code = get_dataframes(a, b)

    fig, ax = plt.subplots()
    myLocator = mticker.MultipleLocator(30)
    ax.xaxis.set_major_locator(myLocator)

    ax.plot(df_c["Dato"], df_c[f"Bekraeftede tilfaelde {a}"], '-', label=a)
    ax.plot(df_c["Dato"], df_c[f"Bekraeftede tilfaelde {b}"], '-', label=b)
    ax.set_title(f'Bekræftede tilfælde pr dag pr kommune - {a} vs {b}')
    ax.legend([f'{a}', f'{b}'])
    # ax.xaxis_date()
    fig.autofmt_xdate()
    plt.xlabel("Date")
    plt.ylabel("Confirmed cases per day")
    if print_file:
        plt.savefig(CUSTOM_IMAGES_PATH +
                    f'{a}_{b}_bekraeftede_tilfælde_pr_dag_pr_kommune.png')
    if excel:
        # df_ab = df[(df["Kommune"] == a_code) | (df["Kommune"] == b_code)]
        if not os.path.exists(CUSTOM_EXCEL_PATH):
            os.makedirs(CUSTOM_EXCEL_PATH)
        with pd.ExcelWriter(CUSTOM_EXCEL_PATH + f"{a}_{b} bekraeftede tilfælde pr dag pr kommune.xlsx") as writer:
            df_c.to_excel(
                writer, sheet_name=f"{a}_{b} bekraeftede tilfælde pr dag pr kommune")
    if show:
        plt.show()
