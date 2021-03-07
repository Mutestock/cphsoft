import click
import logic

@click.group()
def manager():
    pass


@manager.command()
@click.option('--decode', '-d')
@click.option('--compare', '-c', nargs=2)
def exercise(decode, compare):
    if(decode):
        logic.stepic_decode(decode)
        logic.geek_decode(decode)
        logic.custom_decode(decode)
    elif compare:
        logic.compare(compare[0],compare[1])
        
        
