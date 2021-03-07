use std::str;
use std::char;
use std::fmt::Binary;
use image::{EncodableLayout, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use byteorder::LittleEndian;

fn push_char_n(n: usize, mut to_extract_from: String, mut to_add_to: String) -> String{
    if n >= to_extract_from.len(){
        panic!("Can't extract info from string since n is larger than length of string")
    }
    for nums in 0..n {
        let last_char = to_extract_from.chars().last().unwrap();
        to_add_to.push(last_char);
        to_extract_from.pop();
        println!("{}", to_extract_from)
    }
    to_add_to
}

fn main() {
    let image = image::open("steg.png")
        .expect("Image could not be found or processed");
    let rgba_vec = image.pixels()
        .map(|pixel| pixel.2.0)
        .collect::<Vec<[u8; 4]>>();
    
    let (height, width) = image.dimensions();
    //for y in 0..height{
    //    for x in 0..width{
    //        println!("{} - {}",x,y)
    //    }
    //}

    let rgba_vec_8 = image.into_rgba8();
    //println!("{:?}", rgba_vec_8[(50,50)]);

//    let image

    //println!("{:?}", image[(100,100)]);

    (0..width).for_each(|y| {
        (0..height)
            .for_each(|x|{
                let intermediate = (rgba_vec_8[(x,y)].0);
                //println!("{:?}", intermediate);
                //let u8: concat = 0;
                for stuff in intermediate.as_bytes(){
                    let mut concat = "".to_string();
                    for garbage in stuff.to_string().as_bytes(){

                        let mut extract_string = format!("0{:b}", garbage);
                        let mut extract = "".to_string();

                        let last_char = extract_string.chars().last().unwrap();
                        extract.push(last_char);
                        extract_string.pop();
                        let last_char = extract_string.chars().last().unwrap();
                        extract.push(last_char);
                        extract_string.pop();

                        concat.push_str(&extract);

                        //extract = push_char_n(1,extract_string, extract);

                        //println!("{}", extract);
                    }
                    //println!("{}", concat);
                    //let intermediate = match str::from_utf8(concat){
                    //    Ok(v)=>v,
                    //    Err(e) => println!("Can't be translated: {}", e),
                    //};
                    println!("{:?}", String::from_utf8_lossy(&concat));
                }
                //let binaries = u32::from_le_bytes(intermediate);
                //println!("{:?}", binaries);
                //let u32num: u32 = intermediate[0]+intermediate[1]+intermediate[2]+intermediate[3];
                //println!("{:?}", thing);
                //println!("{:?}",u8::from_le_bytes(intermediate));
                
                //println!("{} - {}",x,y);
            })
        });

//
    //for instance in rgba_vec{
    //    //println!("{:?}", instance)
    //    //let intermediate = match str::from_utf8(instance){
    //    //    Ok(v)=>v,
    //    //    Err(e) => println!("Can't be translated: {}", e),
    //    //};
    //    println!("{:?}", String::from_utf8_lossy(&instance));
    //}

    //rgba_vec.for_each(|x|{
    //    let intermediate = match str::from_utf8(x){
    //        Ok(v)=>v,
    //        Err(e) => println!("Can't be translated: {}", e)
    //    };
    //    println!("{}",intermediate);
    //})


    //for pixel in image.pixels(){
    //    println!("{:?}", pixel.2.0);
    //}
}
