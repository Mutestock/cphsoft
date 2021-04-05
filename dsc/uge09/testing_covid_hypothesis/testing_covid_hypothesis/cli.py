import click
import logic
from path_aliases import REGIONAL_FILE_TREE, COMMUNAL_BASE_PATH

@click.group()
def manager():
    pass

@manager.command()
@click.option("--show", "-s")
def analysis(show):
    pass

@manager.command()
@click.option("--table", "-t", is_flag=True, help="Just shows one of the tables in the terminal")
def basic(table):
    logic.display_dataframe(REGIONAL_FILE_TREE.get("noegle_tal"))