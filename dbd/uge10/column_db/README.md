<h3>Databases for developers</h3>


<h4>Task 1 - Investigation</h4>
<ul>
    <li>
        <h5>1. What is the point of NoSQL databases?</h5>
        <p>
            Compared to RDBMS it sacrifices management of complex relations for performance. 
            Often it is used for scaleability. These attributes are useful in the world of big data analytics.
            It can be an advantage for programmers, to not have to learn SQL to interact with the database.
            Some database variations might have limited manageability outside specific langauges.
            E.g. Java might be the best language when managing HBase. 
            Note that some of these NoSQL databases have their own query languages.
            E.g. CQL for Cassandra, or GraphQL for Neo4j.
            Examples of database paradigms and underlying platforms include:
            Key-value - e.g. Redis
            Document - e.g. MongoDB
            Wide-Column - e.g. Apache Cassandra (and HBase)
            Graph - e.g. Neo4j
        </p>
    </li>
    <li>
        <h5>2. What is the CAP theorem?</h5>
        <p>C(Consistency) A(Availability) P(Partition Tolerance). Those are three attributes or promises of a distributed system, i.e. a system consisting of multiple machines/nodes.
        <ul>
            <h5>Consistency</h5>
            <li>
                <p>Writing some data to one node, and then reading said data from another node, will return the same content. Connecting to one node over another should have the same result from all clients. To achieve this, all data must be replicated between all nodes in the system.</p>
            </li>
            <h5>Availability</h5>
            <li>
                <p>Sending information to one node should provoke a response, regardless of whether or not one or more nodes in the network are down.</p>
            </li>
            <h5>Partition Tolerance</h5>
            <li>
                <p>A lost or delayed connection between nodes, must not stop the cluster from working.</p>
            </li>
        </ul>
        </p>
    </li>
    <li>
        <h5>3. What are the ideal use cases of HBase?</h5>
        <p>If you're working with big data. If you need something snappy. If you need scaleability. If you don't need relations. And, in the case of HBase, if you use Java. If those are all true, then you might consider using HBase.</p>
    </li>
</ul>
<h4>Task 2 - Bloom Filters</h4>
<ul>
    <li>
        <h5>1. What is a bloom filter?</h5>
        <p>It can search the set like object, and result in whether it exists or whether it PROBABLY exists. Bloom filters can very much result in false positives. Because of this, it falls under the probablistic data structure category. This will be accomplished by using hash functions. The accuracy and time consumption increases as more hash functions are used. Entries can't be deleted from a bloom filter</p>
    </li>
    <li>
        <h5>2. What is an advantage of bloom filters over hash tables?</h5>
        <p>Large hashtables would require an increased amount of memory as it scales in comparison to bloom filters. So much more memory, that it makes more sense to store data created with hashtables on a disk. Bloom filters' efficiency makes them much more suitable for executing them in memory.</p>
    </li>
    <li>
        <h5>3. What is a disadvantage of bloom filters?</h5>
        <p>Its use cases are limited by its probablistic nature. You need to know the approximate size of the input. If the largest possible entry would require 1 billion dedicated bits, then 1 billion bits must be pre allocated regardless of how the average input size actually is. </p>
    </li>
    <li>
        <h5>4. Using a language of your choice, implement a bloom filter with add and check functions. The backing bit-array can simply be a long</h5>
        <p>Ok. bloomfilter.rs. Using 64 bit unsigned integer instead.</p>
    </li>
    <li>
        <h5>5. If you are to store one million ASCII strings with an average size of 10 characters in a hash set, what would be the approximate space consumption?</h5>
        <p></p>
    </li>
    <li>
        <h5>6. The folling equation gives the required number of bits of space per inserted key, where E is the false positive rate. b = 1.44log2(1/E) </h5>
        <p></p>
    </li>
    <li>
        <h5>7. How many bits per element are required for a 1% false positve rate?</h5>
        <p></p>
    </li>
    <li>
        <h5>8. How many bits per element are required for a 5% false positive rate?</h5>
        <p></p>
    </li>
    <li>
        <h5>9. If you are to store one million ASCII strings with an average size of 10 characters in a bloom filter, what would be the approximate space consumption, given a false poitive rate of 5%?</h5>
        <p></p>
    </li>
</ul>
<h4>Task 3 - Huffman Coding</h4>
<ul>
    <li>
        <h5>1. Generate Huffmann Coded (and draw the Huffman Tree) based on the following string: "beeps beeps!!!!! their eerie ears hear pears"</h5>
        <p></p>
    </li>
    <li>
        <h5>2. How many bits is the compressed string? How many bits is the raw ASCII string?</h5>
        <p></p>
    </li>
    <li>
        <h5>3. Compress "pete is here" with the Huffmann tree from before.</h5>
        <p></p>
    </li>
    <li>
        <h5>4. Write your own 10 word sentence. Generate the Huffmann Code (a new Huffmann Tree), and write a new compressed message (i.e. in binary). Swap with one of your fellow students, and de compress each other's message.</h5>
        <p></p>
    </li>
</ul>
<h4>Task 4 - Map and Reduce</h4>
<ul>
    <li>
        <h5>1. Map the list of numbers to a list of their square roots: [1, 9, 16, 100]</h5>
        <p></p>
    </li>
    <li>
        <h5>2. Map the list of words so each is wrapped in a h1 tag: ["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"</h5>
        <p></p>
    </li>
    <li>
        <h5>3. Use map to uppercase the words (all letters): ["I'm", "yelling", "today"]</h5>
        <p></p>
    </li>
    <li>
        <h5>4. Use map to transform words into their lengths: ["I", "have", "loooooong", "words"]</h5>
        <p></p>
    </li>
    <li>
        <h5>5. Get the json file comics.json from the course site. Paste it into your browser's Javascript console. Use map to get the image urls, and wrap them in img-tags.</h5>
        <p></p>
    </li>
    <li>
        <h5>6. Use reduce to sum the array of numbers: [1,2,3,4,5]</h5>
        <p></p>
    </li>
    <li>
        <h5>7. Use reduce to sum the x-value of the objects in the array: [{x: 1}, {x: 2}, {x: 3}]</h5>
        <p></p>
    </li>
    <li>
        <h5>8. Use reduce to flatten an array of arrays: [[1,2] [3,4], [5,6]]</h5>
        <p></p>
    </li>
    <li>
        <h5>9. Use reduce to return an array of the positive numbers: [-3, -1, 2, 4, 5]</h5>
        <p></p>
    </li>
    <li>
        <h5>10: Optional: The accumulator function can obviously use objects outside of itself. Use reduce to implement groupBy. For example:      
        people= [
            {name: ’Rikke’, age:46},
            {name: ’Michael’, age:47},
            {name: ’Mathias’, age:46}
        ];
        should be turned into:
            groupedPeople = groupBy(people, ’age’);
            /*
            groupPeople: {
                46: [
                    {name: ’Rikke’, age:46},
                    {name: ’Mathias’, age:46}
                ],
                47: [
                    {name: ’Michael’, age:47}
                ]
            }
            */
        </h5>
        <p></p>
    </li>
</ul>


NoSQL database types 
https://hur.st/bloomfilter/