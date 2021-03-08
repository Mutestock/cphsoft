import click
from logic import read_files

@click.group()
def manager():
    pass

 
@manager.command()
@click.option("--kek", '-k', nargs=3)
def exercise(kek):
    if kek:
        read_files(kek)