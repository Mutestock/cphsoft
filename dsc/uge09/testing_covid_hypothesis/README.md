<p>Supposedly there should be a "municipality_cases.xslx" file in some github repo. Well, I couldn't find it. Using this: https://covid19.ssi.dk/overvagningsdata/download-fil-med-overvaagningdata</p>
<br>
<h4>Virtual environment:</h4>
<p>I recommend watching this video: https://www.youtube.com/watch?v=APOPm01BVrk&ab_channel=CoreySchafer </p>
<p>cd to this folder in a terminal</p>
<p>python3 -m venv venv</p>
<p>If windows: </p>
<p>If cmd:</p>
<p>> venv\scripts\activate.bat</p>
<p>If powershell:</p>
<p>> venv\scripts\activate.ps1</p>
<br>
<p>If unix bash terminal:</p>
<p>> source/bin/activate</p>
<br>
<p>> pip3 list</p>
<p>Your list of installed modules should be empty except for setuptools and pip</p>
<p>> pip3 install -r requirements.txt</p>
<p>You now have an environment with module versions which matches mine exactly.</p>
<p>Any issues with installing the requirements.txt is likely to be from a wrong python version.</p>
<p>venv is built into python.</p>
<p>If you look inside the venv folder, you'll see the libraries you install under lib/pythonx.x/site-packages</p>
<p>This kind of setup is much more alike something you'd see in other languages such as javascript with npm modules.</p>
<p>to deactivate the virtual environment, simply type: 'deactivate' in your terminal.
<p>I prefer poetry. I am, however, trying to make my projects more accessible to others, hence the guidelines for venv</p>
<p>https://python-poetry.org/ </p>
<br><br>
<h4>Commands:</h4>
<p>It's a bit complicated. So hang in there.</p>
<p>After having entered the virtual environment:</p>
<br>
<p>cd to the root of this project</p>
<p>> python testing_covid_hypothesis/main.py analysis -t</p>
<p>This will display the solution to the assignment</p>
<p>But you can choose whichever municipalities you want:</p>
<br>
<p>> python testing_covid_hypothesis/main.py analysis -tc gladsaxe ishøj</p>
<br>
<p>The 'c' in '-tc' means 'choice'</p>
<p>You can display a simple graph comparing copenhagen and aarhus: </p>
<br>
<p>> python testing_covid_hypothesis/main.py analysis -c -s</p>
<br>
<p>You can of course select whichever municipalities you want here as well</p>
<p>To almost display the full potential of the program:</p>
<br>
<p>> python testing_covid_hypothesis/main.py analysis -cc ballerup roskilde -s -p -ex</p>
<br>
<p>Where '-s' means 'show'. '-p' means 'print', '-ex' means 'excel'</p>
<p>Graphs created with -p gets printed to ./resources/generated_images</p>
<p>Excel files created with -ex gets published in ./resources/excel</p>
<p>Finally, this setup works for mean, standard variation, and variance as well:</p>
<br>
<p>> python testing_covid_hypothesis/main.py analysis -mc ballerup roskilde -s -p -ex</p>
<p>> python testing_covid_hypothesis/main.py analysis -vc ballerup roskilde -s -p -ex </p>
<p>> python testing_covid_hypothesis/main.py analysis -stdc ballerup roskilde -s -p -ex </p>
<br>
<p>You have to use at least 1 extra tag (-s, -p, -ex). You will get an error if you don't.</p>
<p>Check out the cli.py file if you're curious.</p>
<br>
<p>For arbitrary tests: </p>
<p>nose2 -v</p>
<br>
<h4>Assignment</h4>
<p>So the hypothesis which is pre-defined is very open:</p>
<p>Test the following null hypothesis: Covid infection spreads randomly and similarly in big
Danish cities such as Copenhagen and Aarhus.</p>
<p>I'm going to take the liberty to expand on this.</p>
<p>The definition of a "big city" should be defined by population density</p>
<p>"Similarly" should mean the swings from the day to day basis, pr. municaplity, in accordance with population density. </p>
<p>"Random" should mean upholding the similarity above, with a predefined percentage of accuracy, before correlation can be justified</p>
<br>
<p>To adhere to the definition of the assignment, I also need to find the following:</p>
<ul>
    <h5>t-Test</h5>
    <p>For each set dispute: </p>
    <ul>
        <li>□ mean 𝑋</li>
        <li>□ std deviation - S</li>
        <li>□ variance V = 𝑆 #</li>
        <li>□ t-value</li>
    </ul>
    <br>
    <p>Where the calculation of t-value is:</p>
    <ul>
        <li>𝑡−𝑣𝑎𝑙𝑢𝑒 = signal/𝑛𝑜𝑖𝑠𝑒</li>
        <li>signal = |𝑋𝑐 - 𝑋𝑎 |</li>
        <li>noise = sqrt((Vc/nc)/(Vc/nc))</li>
        <li>Vc = 𝑆𝑐² </li>
        <li>Va = 𝑆𝑎² </li>
    </ul>
</ul>

