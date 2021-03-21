import click
from numpy import log
import logic


# https://click.palletsprojects.com/en/7.x/

@click.group()
def manager():
    pass



# Program generates noise for you
@manager.command()
@click.option("--show", "-s", is_flag=True, help="Show a randomly generated image")
def assignment(show):
    if show:
        logic.generate_noise().show()


# Use any image you want
# add -p to the command if you want the dimensions to be preserved
@manager.command()
@click.option("--show", "-s", help="Show an image of your choice")
@click.option("--preserve", "-p", is_flag=True, help="Preserves the dimensions of the image")
@click.option("--no_greyscale",'-n', is_flag=True, help="Skips usage of greyscale")
def extra(show, preserve, no_greyscale):
    if show:
        logic.generate_greyscale_from_image(show, preserve, no_greyscale)
    