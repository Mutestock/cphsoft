import click
import logic
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE


@click.group()
def manager():
    pass


@manager.command()
@click.option("--compare", "-c", is_flag=True, help="Compares copenhagen and aarhus as per assignment description")
@click.option("--compare-choice", "-cc", nargs=2, help="Compare any two municpalities in Denmark (Assuming it's part of the data set) ")
@click.option("--print", '-p', is_flag=True)
@click.option("--show", "-s", is_flag=True)
@click.option("--excel", "-ex", is_flag=True)
def analysis(show, print, excel, compare, compare_choice):
    if compare:
        if show == False and print == False and excel == False:
            print("Nothing will happen unless you supply the command with either a -p (print), -s (show) or -ex (excel) tag") 
        else:   
            logic.compare_municipalities_confirmed_cases(show=show, print_file=print, excel=excel)
    elif compare_choice:
        logic.compare_municipalities_confirmed_cases(compare_choice[0], compare_choice[1])
    


@manager.command()
@click.option("--table", "-t", is_flag=True, help="Just shows one of the tables in the terminal")
def basic(table):
    logic.display_dataframe(REGIONAL_FILE_TREE.get("noegle_tal"))
    
    
