import os

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(ROOT_DIR)

REGIONAL_BASE_PATH = ROOT_DIR + "/resources/Regionalt_DB/"
COMMUNAL_BASE_PATH = ROOT_DIR + "/resources/Kommunalt_DB/"

REGIONAL_FILE_TREE = {
    "noegle_tal": REGIONAL_BASE_PATH+"01_noegle_tal.csv",
    "hospital_belaegning": REGIONAL_BASE_PATH+"02_hospitalsbelaegning.csv",
    "bekraeftede_tilfaelde_doede_indlagte_pr_dag_pr_koen": REGIONAL_BASE_PATH+"03_bekraeftede_tilfaelde_doede_indlagte_pr_dag_pr_koen.csv",
    "indlagte_pr_alders_grp_pr_region": REGIONAL_BASE_PATH+"04_indlagte_pr_alders_grp_pr_region.csv",
    "bekraeftede_tilfaelde_doede_pr_region_pr_alders_grp": REGIONAL_BASE_PATH+"05_bekraeftede_tilfaelde_doede_pr_region_pr_alders_grp.csv",
    "nye_indlaeggelser_pr_region_pr_dag": REGIONAL_BASE_PATH+"06_nye_indlaeggelser_pr_region_pr_dag.csv",
    "antal_doede_pr_dag_pr_region": REGIONAL_BASE_PATH+"07_antal_doede_pr_dag_pr_region.csv",
    "bekraeftede_tilfaelde_pr_dag_pr_regions": REGIONAL_BASE_PATH+"08_bekraeftede_tilfaelde_pr_dag_pr_regions.csv",
    "bekraeftede_tilfaelde_pr_test_region_pr_uge": REGIONAL_BASE_PATH+"09_bekraeftede_tilfaelde_pr_test_region_pr_uge.csv",
    "testede_pr_uge_pr_samfundsspor_opgjort_paa_bopaelsregion": REGIONAL_BASE_PATH+"10_testede_pr_uge_pr_samfundsspor_opgjort_paa_bopaelsregion.csv",
    "noegletal_pr_region_pr_aldersgruppe": REGIONAL_BASE_PATH+"11_noegletal_pr_region_pr_aldersgruppe.csv",
    "noegletal_pr_region_pr_aldersgruppe_de_seneste_7_dage": REGIONAL_BASE_PATH+"12_noegletal_pr_region_pr_aldersgruppe_de_seneste_7_dage.csv",
    "noegletal_pr_region_pr_aldersgruppe_de_seneste_7_dage": REGIONAL_BASE_PATH+"12_noegletal_pr_region_pr_aldersgruppe_de_seneste_7_dage.csv",
    "regionale_kort": REGIONAL_BASE_PATH+"13_regionale_kort.csv",
    "testede_pr_test_region_pr_uge": REGIONAL_BASE_PATH+"14_testede_pr_test_region_pr_uge.csv",
    "indlagte_pr_region_pr_dag": REGIONAL_BASE_PATH+"15_indlagte_pr_region_pr_dag.csv",
    "pcr_og_antigen_test_pr_region": REGIONAL_BASE_PATH+"16_pcr_og_antigen_test_pr_region.csv"
}

COMMUNAL_FILE_TREE = {
    "noegletal": COMMUNAL_BASE_PATH+"01_noegletal.csv",
    "indlaeggelser_pr_dag_kummulativt": COMMUNAL_BASE_PATH+"02_indlaeggelser_pr_dag_kummulativt.csv",
    "indlaeggelser_pr_aldersgrp": COMMUNAL_BASE_PATH+"03_indl├жggelser_pr_aldersgrp.csv",
    "bekraeftede_tilfaelde_doed_pr_aldersgrp": COMMUNAL_BASE_PATH+"04_bekraeftede_tilfaelde_doed_pr_aldersgrp.csv",
    "nye_indlaeggelser_pr_dag": COMMUNAL_BASE_PATH+"05_nye_indlaeggelser_pr_dag.csv",
    "doed_pr_dag_kumuleret": COMMUNAL_BASE_PATH+"06_doed_pr_dag_kumuleret.csv",
    "bekraeftede_tilfaelde_pr_dag_pr_kommune": COMMUNAL_BASE_PATH+"07_bekraeftede_tilfaelde_pr_dag_pr_kommune.csv",
    "proever_pr_uge_pr_region": COMMUNAL_BASE_PATH+"08_proever_pr_uge_pr_region.csv",
    "tilfaelde_aldersgrp_kommuner": COMMUNAL_BASE_PATH+"09_tilfaelde_aldersgrp_kommuner.csv",
    "kort_pr_kommune": COMMUNAL_BASE_PATH+"10_kort_pr_kommune.csv",
    "pcr_og_antigen_test_pr_kommune": COMMUNAL_BASE_PATH+"11_pcr_og_antigen_test_pr_kommune.csv",
    "mistaenkte_pos_pr_kommune_pr_dag": COMMUNAL_BASE_PATH+"12_mistaenkte_pos_pr_kommune_pr_dag.csv"
}
