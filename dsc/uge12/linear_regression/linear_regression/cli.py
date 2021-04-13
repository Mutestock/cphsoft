import click
from numpy import log
import logic
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE


@click.group()
def manager():
    pass


@manager.command()
def analysis():
    pass

def no_tags_error_message(show, print, excel):
    # Most commands needs extra informamtion about what they're supposed to do with the specified data.
    # E.g. --compare would just run the function and process the data as needed.
    # You need to tell the program what to do with it, i.e. do you want to print it, show it, and/or convert it to excel.
    # You must the command with at least one extra tag, but you can supply all of them tags.
    # These extra tags being: -s, -p or -ex. Respectively.
    if show == False and print == False and excel == False:
        print("Nothing will happen unless you supply the command with either a -p (print), -s (show) or -ex (excel) tag")
        return False
    else:
        return True
