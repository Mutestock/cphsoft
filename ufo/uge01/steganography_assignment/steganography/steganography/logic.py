import stepic
from PIL import Image

def stepic_decode(image):
    try:
        stego_image = stepic.decode(image)
        stego_image
    except:
        print("Image not compatible with stepic. Moving on...")
        
def geek_decode(img):
    try:
        image = Image.open(img, 'r')
    
        data = ''
        imgdata = iter(image.getdata())
    
        while (True):
            pixels = [value for value in imgdata.__next__()[:3] +
                                    imgdata.__next__()[:3] +
                                    imgdata.__next__()[:3]]
    
            # string of binary data
            binstr = ''
    
            for i in pixels[:8]:
                if (i % 2 == 0):
                    binstr += '0'
                else:
                    binstr += '1'
    
            data += chr(int(binstr, 2))
            if (pixels[-1] % 2 != 0):
                print(data)
                return data
    except:
        print("Image could not be decoded with function from geeksforgeeks. Moving on...")

def custom_decode(img):
    try:
        image = Image.open(img, 'r')
        for x in range(image.width):
            for y in range(image.height):
                pix = image.getpixel((x,y))
                print(pix)
                if pix[3] !=255:
                    print(pix)
    except:
        print("Custom decode failed :(")    
        
def compare(first, second):
    first = Image.open(first, 'r')
    second = Image.open(second, 'r')
    second = second.resize(first.size)
    second_data = list(second.getdata())
    second = Image.new(first.mode, first.size)
    second.putdata(second_data)
    
    count = 0
    for x in range(first.width):
        for y in range(first.height):
            pix01 = first.getpixel((x,y))
            pix02 = second.getpixel((x,y))
            
            if pix01 == pix02:
                count=count+1
                print(f"{count} - {pix01} - {pix02}")
