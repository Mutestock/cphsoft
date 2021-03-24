import click
from numpy import log
import logic


# https://click.palletsprojects.com/en/7.x/

@click.group()
def manager():
    pass

# Program generates noise for you
# Use -c some_number1 some_number2 for custom dimensions, e.g. -c 128 256
# non-greyscale is not implemented


@manager.command()
@click.option("--show", "-s", is_flag=True, help="Show a randomly generated image")
@click.option("--custom_dimensions", "-cd", nargs=2, help="Use this if you want to use something else than 32x32")
@click.option("--convulutional_multiplication", "-cv", is_flag=True, help="Shows the result of the image after manipulation")
@click.option("--max_pool","-m", is_flag=True, help="Shows the result of the image after max pool stuff")
def assignment(show, custom_dimensions, convulutional_multiplication, max_pool):
    if not custom_dimensions:
        custom_dimensions = (32, 32)
    if show:
        logic.generate_noise(custom_dimensions=custom_dimensions).show()
    elif convulutional_multiplication:
        logic.convolve(custom_dimensions=custom_dimensions).show()
    elif max_pool:
        logic.max_pool(custom_dimensions=custom_dimensions).show()


# Use any image you want
# add -p to the command if you want the dimensions to be preserved
@manager.command()
@click.option("--show", "-s", help="Show an image of your choice")
@click.option("--preserve", "-p", is_flag=True, help="Preserves the dimensions of the image")
@click.option("--use_greyscale", '-n', is_flag=True, help="Use greyscale to complete the assignment")
@click.option("--convulutional_multiplication", "-cv", help="Shows the result of the image after manipulation")
@click.option("--max_pool","-m", help="Shows the result of the image after max pool stuff")
def extra(show, preserve, use_greyscale, convulutional_multiplication, max_pool):
    if show:
        logic.generate_greyscale_from_image(
            show, preserve, use_greyscale).show()
    elif convulutional_multiplication:
        logic.convolve(file_path=convulutional_multiplication, preserve_dimensions=preserve, use_greyscale=use_greyscale).show()
    elif max_pool:
        logic.max_pool(file_path=max_pool, preserve_dimensions=preserve, use_greyscale=use_greyscale).show( )