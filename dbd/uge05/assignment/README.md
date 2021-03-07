
<h3>--- The reviewer ---</h3>

<h6>
run with: docker-compose up

That should be all. Make sure none of your other processes uses port 13337 Otherwise change it in docker-compose.yml

pg info:

    POSTGRES\_USER: softdbd 

    POSTGRES\_PASSWORD: softdbd 

    POSTGRES\_DB: garbage


<br><br>
This is a Rest Api.

Currently open routes:
!!

<br><br><br>


FILL ME DADDY OwO


<br><br><br>

!!




Primary crates:

This project uses the asynchronous run time called Tokio. 

Instead of an ORM framework (Usually Diesel in Rust), this particular project uses SQLx. 

SQLx is a crate (similar to package) which provides asynchronous compile-time checked queries. 

Lastly, the project uses Warp, which is an asynchronous web server framework (equivalent of flask in python).

<br><br>


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

Ideas: project will use docker-compose to initialize both rust and Postgres.

Postgres first (wait for it) -\> Rust migrations gets started.

<br><br>

Two migration tactics: 

    1. function call in main function 
    
    2. SQLx cli commands in docker file.

Complication: 

    SQLx macros somehow checks for database coherence. This would be cool, if it wasn't because it stops the program from compiling if the table doesn't exist. However, we're reliant on the SQLx function to be called in the main method.

    In other words: We can't execute the code, because we don't have the tables in the database,
    because we can't execute the code, because we don't have the tables in the database,
    because ... etc...
    So that's fucking annoying.

    Migration tactic 1 therefore requires an alternative to the query! macros. 
    Possibly execution of an .sql file?

Constraint: 

    I am the only one using Rust.

    The entire setup must be able to be created automatically by everyone with ONLY docker-compose.

    I can't expect that the client(or reviewer) will install all the dependencies related to the project.

</h6>