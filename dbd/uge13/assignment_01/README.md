 
1. 
Task:
Create a Movie node for the movie with a title Forrest Gump.

Answer:
CREATE (ForrestGump:Movie {title:'Forrest Gump', released:1994, tagline:'Run Forrest'})

Image:


2.
Task:
Add the following properties to the movie Forrest Gump:
a. released: 1995
b. tagline: Life is like a box of chocolates…you never know what you’re gonna get.

Answer:
MATCH (m:Movie {title: 'Forrest Gump'})
SET m.released = '1995'
SET m.tagline = 'Life is like a box of chocolates…you never know what you’re gonna get.'
RETURN m

Image:

3.
Task:
Update the released property of movie Forrest Gump, as it has actually been
released in 1994.

Answer:
MATCH (m:Movie {title: 'Forrest Gump'})
SET m.released = '1994'
RETURN m

Image:

4.
Task:
Find the movie with the tagline Free your mind

Answer:
MATCH (m:Movie {tagline: 'Free your mind'})
RETURN m

Image:

5.
Task:
Retrieve the movie The Matrix and all its relationships.

Answer:
MATCH (m:Movie {title: 'The Matrix'})-[r]-(b)
RETURN r, m, b

Image:


6.
Task:
Find the names and relationship type of all people who have any type of relationship
to the movie The Matrix

Answer:

MATCH (people:Person)-[relatedTo]-(:Movie {title: "The Matrix"}) 
RETURN people.name, Type(relatedTo), relatedTo

Image:


7.
Task:
Find all people born in the previous century.

Answer:
MATCH (people:Person)
WHERE people.born<2000
RETURN people.name, people.born

Image:

8.
Task:
Find all people who gave the movie The Da Vinci Code a rating of 65, returning their
names.

Answer:
MATCH (people:Person)-[relatedTo]-(:Movie {title: "The Da Vinci Code"})
WHERE Type(relatedTo) = 'REVIEWED' 
AND (properties(relatedTo)).rating = 65
RETURN people.name, (properties(relatedTo)).rating

Image:

9.
Task:
Find all people who follow Angela Scope and those who Angela Scope follows.

Answer:
Follows Angela:
MATCH (:Person{name: 'Angela Scope'}) <- [:FOLLOWS]-(follower)
RETURN follower

Angela follows:
MATCH (:Person{name: 'Angela Scope'})-[:FOLLOWS]->(follower)
RETURN follower

Image:

10.
Task:
Find all people who follow anybody who follows Jessica Thompson returning them as
nodes

Answer:
MATCH (:Person{name: 'Jessica Thompson'})<-[:FOLLOWS]-(follower)
RETURN follower

Image:

11. 
Task:
Tom Hanks hasn’t HELPED Gary Sinise in a research. Remove this property from
the relation.

Answer:
Searching through the script doesn't reveal any information related to "HELPED" or "research".
Judging task as malformed and skipping.

image:


12.
Task:
Delete the whole person-to-person relationship HELPED from the graph

Answer:
Searching through the script doesn't reveal any information related to "HELPED" or "research".
Judging task as malformed and skipping.





