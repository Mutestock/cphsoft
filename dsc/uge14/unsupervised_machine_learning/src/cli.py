import click

@click.group()
def manager():
    pass

@manager.command()
@click.option('--stuff', '-s', is_flag=True)
def assignment(stuff):
    pass

