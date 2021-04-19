import click

@click.group()
def manager():
    pass


@manager.command()
@click.option("--bayes", '-b', is_flag=True)
def assignment(bayes):
    if bayes:
        print("boop")
        
        