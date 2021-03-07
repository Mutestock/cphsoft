import click
import exercises as exercises
import numpy as np


@click.group()
def manager():
    pass


@manager.command()
@click.option('--mag', nargs=2)
@click.option('--unit', nargs=2)
@click.option('--rot', nargs=2)
@click.option('--solutions', is_flag=True)
def solve(mag, unit, rot, solutions):
    if mag:
        print(exercises.mag(np.array(mag)))
    elif unit:
        print(exercises.unit(np.array(unit)))
    elif rot:
        print(exercises.rot90(np.array(rot)))
    elif solutions:
        a = np.array([3,2])
        b = np.array([8,7])
        c = np.array([1,5])
        print("\n========== \n")
        print(f"v: {2*a}")
        print(f"w: {a+b-c}")
        print(f"y: \n np: {np.dot(a,a)} \n mag: {exercises.mag(a) * exercises.mag(a)}")
        print(f"z: \n np: {np.dot(a,b)} \n mag: {exercises.mag(a) * exercises.mag(b)}")
        a90 = exercises.rot90(a)
        print(f"æ: {np.dot(a, a90)}")
        print("\n========== \n")
        
        
        
        
