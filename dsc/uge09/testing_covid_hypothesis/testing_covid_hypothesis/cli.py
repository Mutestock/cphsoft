import click
import logic
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE


@click.group()
def manager():
    pass


@manager.command()
@click.option("--show", "-s")
@click.option("--compare", "-c", is_flag=True, help="Compares copenhagen and aarhus as per assignment description")
@click.option("--compare-choice", "-cc", nargs=2, help="Compare any two municpalities in Denmark (Assuming it's part of the data set) ")
def analysis(show, compare, compare_choice):
    if show:
        pass
    elif compare:
        logic.compare_municipalities_confirmed_cases()
    elif compare_choice:
        logic.compare_municipalities_confirmed_cases(compare_choice[0], compare_choice[1])


@manager.command()
@click.option("--table", "-t", is_flag=True, help="Just shows one of the tables in the terminal")
def basic(table):
    logic.display_dataframe(REGIONAL_FILE_TREE.get("noegle_tal"))
    
    
