import pandas as pd
import os

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(ROOT_DIR)
RESOURCES_DIR = ROOT_DIR + "/resources/"
DATA = RESOURCES_DIR + "data.csv"
OLD_DATA = RESOURCES_DIR + "old_data.csv"

if __name__ == "__main__":
    old_df = pd.read_csv(OLD_DATA)
    optimized_df = pd.read_csv(DATA)
    
    old_std = old_df.std()[0]
    old_mean = old_df.mean()[0]
    
    optimized_std = optimized_df.std()[0]
    optimized_mean = optimized_df.mean()[0]
    
    print(f"""
    == Non-optimized stats ==
    
    Standard deviation: {old_std}
    Mean: {old_mean}
    
    == Optimized stats ==
    
    Standard deviation: {optimized_std}
    Mean: {optimized_mean}
          
    """)
    
    