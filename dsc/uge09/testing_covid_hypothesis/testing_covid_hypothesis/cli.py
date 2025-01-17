import click
from numpy import log
import logic
from aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE


@click.group()
def manager():
    pass


@manager.command()
@click.option("--compare", "-c", is_flag=True, help="Compares copenhagen and aarhus as per assignment description")
@click.option("--compare-choice", "-cc", nargs=2, help="Compare any two municpalities in Denmark (Assuming it's part of the data set) ")
@click.option("--print", '-p', is_flag=True, help="Sets flag to print the graph to a .png file in the ./resources/generated_images folder")
@click.option("--show", "-s", is_flag=True, help="Shows the graph via. matplotlib")
@click.option("--excel", "-ex", is_flag=True, help="Prints the data to an excel file in ./resources/excel")
@click.option("--mean", "-m", is_flag=True, help="Gets mean of copenhagen and aarhus and compares them as per assignment description")
@click.option("--mean-choice", "-mc", nargs=2, help="Same as mean, but lets you pick which two munipalities you want to inspect the mean values of(Assuming it's part of the data set")
@click.option("--standard-deviation", "-std", is_flag=True, help="Standard deviation for copenhagen and aarhus")
@click.option("--standard-deviation-choice", "-stdc", nargs=2, help="Standard deviation with two choices")
@click.option("--variance", "-v", is_flag=True, help="Variance for copenhagen and aarhus")
@click.option("--variance-choice", "-vc", nargs=2, help="Variance with two choices")
@click.option("--t-value", "-t", is_flag=True, help="Prints out le t-value of copenhagen and aarhus")
@click.option("--t-value-choice", "-tc", nargs=2, help="Prints out le t-value of two choices")
def analysis(
        show, print, excel, compare, compare_choice, mean, mean_choice,
        standard_deviation, standard_deviation_choice, variance, variance_choice, t_value, t_value_choice):
    if compare:
        if no_tags_error_message(show, print, excel):
            logic.compare_municipalities_confirmed_cases(
                show=show, print_file=print, excel=excel)
    elif compare_choice:
        if no_tags_error_message(show, print, excel):
            logic.compare_municipalities_confirmed_cases(
                compare_choice[0], compare_choice[1], show=show, print_file=print, excel=excel)
    elif mean:
        if no_tags_error_message(show, print, excel):
            logic.mean(a="copenhagen", b="aarhus", show=show,
                       print_file=print, excel=excel)
    elif mean_choice:
        if no_tags_error_message(show, print, excel):
            logic.mean(a=mean_choice[0], b=mean_choice[1],
                       show=show, print_file=print, excel=excel)
    elif standard_deviation:
        if no_tags_error_message(show, print, excel):
            logic.standard_deviation(
                a="copenhagen", b="aarhus", show=show, print_file=print, excel=excel)
    elif standard_deviation_choice:
        if no_tags_error_message(show, print, excel):
            logic.standard_deviation(
                a=standard_deviation_choice[0], b=standard_deviation_choice[1], show=show, print_file=print, excel=excel)
    elif variance:
        if no_tags_error_message(show, print, excel):
            logic.variance(a="copenhagen", b="aarhus", show=show,
                           print_file=print, excel=excel)
    elif variance_choice:
        if no_tags_error_message(show, print, excel):
            logic.variance(
                a=variance_choice[0], b=variance_choice[1], show=show, print_file=print, excel=excel)
    elif t_value:
        logic.t_value()
    elif t_value_choice:
        logic.t_value(t_value_choice[0], t_value_choice[1])


@manager.command()
@click.option("--table", "-t", is_flag=True, help="Just shows one of the tables in the terminal")
def basic(table):
    logic.display_dataframe(REGIONAL_FILE_TREE.get("noegle_tal"))


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
