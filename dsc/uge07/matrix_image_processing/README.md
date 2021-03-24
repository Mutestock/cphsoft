

<p>This project has been created as a CLI tool.</p>
<p>You might not have the required dependencies. Try the commands below. If they aren't working, read virtual environment stuff</p>

<p> CD to this directory and run these: </p>
<p> a: python matrix_image_processing/main.py extra -s ./resources/sample.png </p>
<p> b: python matrix_image_processing/main.py extra -cv ./resources/sample.png</p>
<p> c && d: python matrix_image_processing/main.py extra -m ./resources/sample.png <p>
<br>
<p> You have the option to preserve the image size if you want with --preserve or -p, e.g.</p>
<p> python matrix_image_processing/main.py extra -m ./resources/sample.png -p</p>
<br><br>
<p> Note that you should be able to point at any image in your file system with this setup. Just replace the ./resource/sample.png with the path to your image</p>
<br>
<p> I've created some functionality with randomly generated images which resemble static </p>
<p> python matrix_image_processing/main.py assignment -s -cd 128 256 </p>
<p> -cd here is short for custom_dimensions. Removing -cd 128 256 from the statement above, will run the default 32x32 </p>
<p> python matrix_image_processing/main.py assignment -cv -cd 519 125 </p>
<p> python matrix_image_processing/main.py assignment -m -cd 128 256 </p>

<p>____ Virtual Environment stuff ____</p>
<p>
This project uses the virtual environment and package manager Poetry:
https://python-poetry.org/
</p>

<p>
You can use venv instead, if you also use command below this link.
https://docs.python.org/3/library/venv.html
</p>

<p>
pip install -r requirements.txt
Note: I can't promise that installing the requirements won't make package conflicts if you don't use a virtual environment
<br>
___
</p>

<p> Stock image https://www.pexels.com/photo/crystal-glasses-3197553/ </p>
