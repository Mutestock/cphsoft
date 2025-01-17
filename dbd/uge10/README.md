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
        <p>If you're somehow referring to the bloomfilter I just created, then you're having too many unknowns in the formula: total capacity, false positive rate and amount of hash function calls. 
        If you're referring to a hash table, then hash tables have a complexity rate of O(1~) due to collision.
        As per: http://java-performance.info/memory-consumption-of-java-data-types-2/
        It will take 32 * SIZE + 4 * CAPACITY bytes. Where default CAPACITY is calculated by SIZE/0.75
        Supposedly 1 character is 1 byte since it's ascii. 10 Characters would be 10 bytes. As far as I can remember, it's not exactly that simple, but I'm gonna go with it.
        Outside the context of the hashmap, one million element * 10 characters(byte), so 10 million bytes or 10000 kilobytes or 10 megabytes. 
        <br>
        Each entry in a hashmap (Looking at Java documentation) (or hash set. Unique key) contains both a key and a value. So it's larger than that. Now, an entry in a hashmap per default takes 32 bytes. The documentation states, that a hashmap with default load factors and a size of 100 will occupy 3736 bytes(32 * 100 + 4 * 134 = 3200 + 536 = 3736). This excludes the actual size of both the keys and the values. Taking this into consideration, if we insert the 1 million ascii strings from before:
        <br>
        32*1000000 + 4 * (1000000/0.75) = 32.000.000 + 1.333.333 = 33.333.333 bytes
        <br>
        Now as stated before, this excludes the actual size of the keys and values. With the strings as keys from before, you'd add that up to 43.333.333. Then there's the values of these keys. According to this link, every key is hashed twice, but is stored as a 32-bit value. https://stackoverflow.com/questions/9364134/what-hashing-function-does-java-use-to-implement-hashtable-class
        This means, that we can add an additional 32 * 1.000.000 to the calculation. Which means we're at 75.333.333 bytes or 75.33 megabytes. That's probably wrong but eh.
        </p>
    </li>
    <li>
        <h5>6. The folling equation gives the required number of bits of space per inserted key, where E is the false positive rate. b = 1.44log2(1/E) </h5>
        <p>Duly noted...</p>
    </li>
    <li>
        <h5>7. How many bits per element are required for a 1% false positve rate?</h5>
        <p>9,5851. Result printed by running program with cargo r. </p>
    </li>
    <li>
        <h5>8. How many bits per element are required for a 5% false positive rate?</h5>
        <p>6,2352. Result printed by changing desired_error_rate in main.rs to 0.05 and running with cargo r </p>
    </li>
    <li>
        <h5>9. If you are to store one million ASCII strings with an average size of 10 characters in a bloom filter, what would be the approximate space consumption, given a false positive rate of 5%?</h5>
        <p>Supposedly 6,2352 bits * 1.000.000 extra bits or 6.235.200 bits or 779400 bytes or 77.94 kilobytes extra. Which is miniscule considering how the sizes of the strings themselves are 10mb. I.e. 10.078 kilobytes.  </p>
    </li>
</ul>
<h4>Task 3 - Huffman Coding</h4>
<ul>
    <li>
        <h5>1. Generate Huffmann Code (and draw the Huffman Tree) based on the following string: "beeps beeps!!!!! their eerie ears hear pears"</h5>
        <p>Image is in resources. My code produces something different. It's using the "greedy" algorithm https://www.codesdope.com/course/algorithms-huffman-codes/ . It all gets even less convincing when I tell you, that I almost got it to function, but I can't really figure out the last bit. It's the very last leaf node connecting to wrong value node.  The code is mine and not copied. It's also rust, so there's not much help to find. It's sort of strange looking as well. I've pretty printed the result and I've put it in the resources folder under task 3. It'll have consequences later on as the compression and decompression starts.</p>
    </li>
    <li>
        <h5>2. How many bits is the compressed string? How many bits is the raw ASCII string?</h5>
        <p>I've tried for too long too implement this now. I'm not having too much success. Such is the consequences of working alone with a different langauge I suppose. </p>
        <p>The raw ascii string has 3855 bits. Calculation in main.rs</p>
    </li>
    <li>
        <h5>3. Compress "pete is here" with the Huffmann tree from before.</h5>
        <p>I can make a tree out of it. I can't compress it. Too stupid</p>
    </li>
    <li>
        <h5>4. Write your own 10 word sentence. Generate the Huffmann Code (a new Huffmann Tree), and write a new compressed message (i.e. in binary). Swap with one of your fellow students, and de compress each other's message.</h5>
        <p>Sentence is "We weep for the blood of a bird, but not for the blood of a fish. Blessed are those who have voice.". Pretty print of tree can be found in resources, alongside a generated image of a non-greedy algorithm calculation. I'm in a group with me, myself and I. All of the group's members had the same sentence, which drained the 'fun' out of it. All the group's members were also too stupid to actually know how to implement functions for compression and decompression. It's recursion with the setup I've made. Just couldn't make them function.</p>
    </li>
</ul>
<h4>Task 4 - Map and Reduce</h4>
<p>I have to be honest - I have no idea how this is related to wide-column databases</p>
<ul>
    <li>
        <h5>1. Map the list of numbers to a list of their square roots: [1, 9, 16, 100]</h5>
        <p>Despite having ptsd for working with nodes for a two-digited amount of hours, I'm going to use Node.js with typescript. The typescript file has been transcompiled and can be seen in the dist folder. Running 'npm start' in your terminal in the root directory will run the script (assuming you have node installed)</p>
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
