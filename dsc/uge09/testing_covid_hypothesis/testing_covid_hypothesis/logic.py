import pandas as pd
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE, LAU_CODES, CUSTOM_IMAGES_PATH, CUSTOM_EXCEL_PATH
import matplotlib.dates as mdates
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import os
import math
from scipy.stats import ttest_ind

# Simple test to see whether or not pandas is capable of reading a specified file.
# Used in cli basic show
# Disregard.

# This is a decorator function. 
# It executes code before and after a subscribed function is called.
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

def t_value(a="copenhagen", b="aarhus"):
    # https://www.educba.com/t-test-formula/
    # t = ( x̄1 – x̄2) / √ [(s21 / n 1 ) + (s22 / n 2 )]
    # x̄1 = Observed Mean of 1st Sample
    # x̄2 = Observed Mean of 2nd Sample
    # s1 = Standard Deviation of 1st Sample
    # s2= Standard Deviation of 2nd Sample
    # n 1 = Size of 1st Sample
    # n 2 = Size of 2nd Sample
    a = a.capitalize()
    b = b.capitalize()
    
    _, df_c, a_code, b_code = get_dataframes(a, b)
    means = df_c.mean()
    sexually_transmitted_diseases = df_c.std()
    variances = df_c.var()
    
    mean_a = means[0]
    mean_b = means[1]
    
    std_a = sexually_transmitted_diseases[0]
    std_b = sexually_transmitted_diseases[1]
    
    variance_a = variances[0]
    variance_b = variances[1]
    
    sample_size = df_c.shape[0]
    
    degrees_of_freedom = 2 * (sample_size - 1)

    t_value = (mean_a - mean_b) / math.sqrt((math.pow(std_a,2)/sample_size) + (math.pow(std_b,2)/sample_size))
    
    # I'm going to be committing the cheatsies by using stats for the p-value.
    # Couldn't find any resources online describing any other way.
    # ¯\_(ツ)_/¯
    scipy_vals = ttest_ind(df_c[f"Bekraeftede tilfaelde {a}"], df_c[f"Bekraeftede tilfaelde {b}"])
    
    print(f"""
    Report time!
    
    Mean {a} : {mean_a}
    Std {a} : {std_a}
    Variance {a} : {variance_a}
    
    Mean {b} : {mean_b}
    Std {b} : {std_b}
    Variance {b} : {variance_b}
    
    Degrees of freedom : {degrees_of_freedom}
    t-value : {t_value}
    
    === Values from Scipy ===
    
    t_value : {scipy_vals[0]}
    p_value : {scipy_vals[1]}
    
    """)
    

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
