from flask import Flask, render_template_string, send_from_directory
import os

app = Flask(__name__, static_url_path='/resources')
	
app.config['RESOURCE_FOLDER'] = "../resources"
 
 
@app.route('/download/<path:filename>')
def download_file(filename):
    try:
	    return send_from_directory(app.config['RESOURCE_FOLDER'], filename, as_attachment=False)
    except FileNotFoundError as ex:
        return "404" + ex
    
@app.route('/display')
def display_file():
    return render_template_string("""
    <img src="ASD.jpeg">
    """)
    

if __name__ == "__main__":
    app.run()
    
# <img src="..resources/out_of_window.gif">