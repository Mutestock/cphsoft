
<h3>--- The reviewer ---</h3>

<h6>
run with: 
<br><br>
docker-compose up

Sadly, due to some time constraints, you will still need Rust to execute this program.



<br><br> 
Make sure none of your other processes uses port 13337, 8008 and 16969. Otherwise change them in docker-compose.prod.yml, .env and/or main.rs respectively.
<br><br>
pg info:

    POSTGRES\_USER: softdbd 

    POSTGRES\_PASSWORD: softdbd 

    POSTGRES\_DB: pg_garbage

The restricted user's info is:

    POSTGRES_USER: restricted_user
    POSTGRES_PASSWORD: restricted_user_password

This user will be created on a specific route which swaps the user to the restricted user.

This project utilizes the table-per-class strategy. It's simple, semi-fast and it's closest to what I'm used to. It's much more convenient with the INHERIT keywords than the alternatives imo.

Single table is ugly as all hell and not worth the hassle. 

Joint-table seems like a nightmare to program correctly. The dynamic of the joint-table strategy really doesn't seem necessary in this project.


<br><br>
I've decided to create a Rest Api. This is because I don't think it's sufficient to learn sql. I need to put it into a relevant context. I don't think just creating an SQL script will bring you very far in the industry...

Currently open routes:
<br><br>

City:

    (POST) localhost:16969/api/city
    (GET) localhost:16969/api/city/{id}
    (PUT) localhost:16969/api/city/{id}
    (DELETE) localhost:16969/api/city/{id}
    (GET) localhost:16969/api/city

Pet:

    (POST) localhost:16969/api/pet
    (GET) localhost:16969/api/pet/{id}
    (PUT) localhost:16969/api/pet/{id}
    (DELETE) localhost:16969/api/pet/{id}
    (GET) localhost:16969/api/pet

Vet:

    (POST) localhost:16969/api/vet
    (GET) localhost:16969/api/vet/{id}
    (PUT) localhost:16969/api/vet/{id}
    (DELETE) localhost:16969/api/vet/{id}
    (GET) localhost:16969/api/vet

Caretaker:

    (POST) localhost:16969/api/caretaker
    (GET) localhost:16969/api/caretaker/{id}
    (PUT) localhost:16969/api/caretaker/{id}
    (DELETE) localhost:16969/api/caretaker/{id}
    (GET) localhost:16969/api/caretaker

Cat:

    (POST) localhost:16969/api/cat
    (GET) localhost:16969/api/cat/{id}
    (PUT) localhost:16969/api/cat/{id}
    (DELETE) localhost:16969/api/cat/{id}
    (GET) localhost:16969/api/cat
    (GET) localhost:16969/api/cat/extended/{id}
    (PUT) localhost:16969/api/cat/extended/{id}
    (GET) localhost:16969/api/cat/extended

Dog:

    (POST) localhost:16969/api/dog
    (GET) localhost:16969/api/dog/{id}
    (PUT) localhost:16969/api/dog/{id}
    (DELETE) localhost:16969/api/dog/{id}
    (GET) localhost:16969/api/dog
    (GET) localhost:16969/api/dog/extended/{id}
    (PUT) localhost:16969/api/dog/extended/{id}
    (GET) localhost:16969/api/dog/extended

// This swaps the user to and from the restricted variant.
Misc:

    (POST) localhost:16969/api/misc  

The 'extended' path signifies calls the functions including the "non-standard" sql queries.
This includes execution of stored procedures and views.

<br><br>

Primary crates(comparable to packages):

This project uses the asynchronous run time called Tokio. 

Instead of an ORM framework (Usually Diesel in Rust), this particular project uses SQLx. 

SQLx is a crate which provides asynchronous compile-time checked queries. 

Lastly, the project uses Warp, which is an asynchronous web server framework (equivalent of flask in python).


<br><br>
About how the assignment has been solved:
<br><br>

<h4> Conceptual level implementation </h4>


Create an SQL-script for a PostgreSQL™database that creates the tables accordingly. Beware that the script should be reentrant:

The database creation sql script is split into many separate migration scripts.
See ./migrations

<br><br>

Create an SQL-script with sample data for your tables. You should have at least two vetenarians, twenty pets of various kinds including some that areneither cats nor dogs, and ten caretakers some with common pets. Also this script should be reentrant:

The database itself is created during the docker container's setup
See ./docker-compose.yml

The script for populating the database has been substituted with a function which calls the entities' create functions. This function is called when the program is started.

<br><br>

<h4> External level implementation </h4>

Create views and/or stored procedures to deal with the chosen inheritance strategy

I've taken the liberty to do neither and instead use the INHERITS keyword... Because that's what it's there for..
https://www.postgresql.org/docs/9.1/tutorial-inheritance.html
See cat / dog migration scripts.

It should be possible to:
•See cats and dogs as separate views
•See all pets as in the single table strategy
•Update cats and dogs with a single SQL call, stored procedure or update on a view with a trigger.

See rest routes












https://tokio.rs/ 

https://diesel.rs/ 

https://github.com/launchbadge/sqlx 

https://github.com/seanmonstar/warp

<br>
<br>
<br>
<br>



</h6>

<h3> --- Mindset(for me) --- </h3>


<h6>

There will be multiple implementations of each entity.

The scripts in cat / dog will create the descriminators

Rust can't create inheritance for us. This is fine.

Postgres views must be implemented. Access must be restricted. This would be complicated for an ORM framework to handle. SQLx will be used for custom sql scripts.

Need pop script with these characterica:

2 cities
2 vets
10 general pets
5 dogs
5 cats
10 caretakers



Ideas: project will use docker-compose to initialize both rust and Postgres.

Postgres first (wait for it) -\> Rust migrations gets started.

<br><br>

Two migration tactics: 

    1. function call in main function 
    
    2. SQLx cli commands in docker file.

Complication: 

    SQLx macros somehow checks for database coherence. 
    This would be cool, if it wasn't because it stops the program from compiling if the table doesn't exist. 
    However, we're reliant on the SQLx function to be called in the main method.

    In other words: We can't execute the code, because we don't have the tables in the database,
    because we can't execute the code, because we don't have the tables in the database,
    because ... etc...
    So that's fucking annoying. - Fixed. Compile checking occurs only on macro queries.

    Migration tactic 1 therefore requires an alternative to the query! macros. 

Constraint: 

    I am the only one using Rust.

    The entire setup must be able to be created automatically by everyone with ONLY docker-compose.

    Portability. I can't expect that the client(or reviewer) will install all the dependencies related to the project. Especially because of Rust.







Other Links:

    https://www.postgresql.org/docs/9.1/tutorial-inheritance.html





</h6>