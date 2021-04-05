import pandas as pd

def display_dataframe(file):
    df = pd.read_csv(file)
    print(df)