import click
from numpy import log
import logic


# https://click.palletsprojects.com/en/7.x/

@click.group()
def manager():
    pass

# Program generates noise for you
# Use -c some_number1 some_number2 for custom dimensions, e.g. -c 128 256


@manager.command()
@click.option("--show", "-s", is_flag=True, help="Show a randomly generated image")
@click.option("--custom_dimensions", "-c", nargs=2, help="Use this if you want to use something else than 32x32")
@click.option("--result", "-r", is_flag=True, help="Shows the result of the image after manipulation")
def assignment(show, custom_dimensions, result):
    if not custom_dimensions:
        custom_dimensions = (32, 32)
    if show:
        logic.generate_noise(custom_dimensions=custom_dimensions).show()
    elif result:
        logic.convolve(custom_dimensions=custom_dimensions)


# Use any image you want
# add -p to the command if you want the dimensions to be preserved
@manager.command()
@click.option("--show", "-s", help="Show an image of your choice")
@click.option("--preserve", "-p", is_flag=True, help="Preserves the dimensions of the image")
@click.option("--use_greyscale", '-n', is_flag=True, help="Use greyscale to complete the assignment")
@click.option("--result", "-r", help="Shows the result of the image after manipulation")
def extra(show, preserve, use_greyscale, result):
    if show:
        logic.generate_greyscale_from_image(
            show, preserve, use_greyscale).show()
    elif result:
        logic.convolve(show, preserve, use_greyscale)
