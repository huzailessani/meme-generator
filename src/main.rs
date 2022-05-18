

use std::{io, path::Path};
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};


fn main() {
    struct Meme{
        name: String,
        text_top: String,
        text_bottom: String,
        image_path: String
    }

    let mut braceyourselves = Meme {
        name: String::from("Brace Yourselves"),
        text_top: String::from("Brace Yourselves"),
        text_bottom: String::new(),
        image_path: String::from("C:/Users/Huzail/Documents/mememachine/src/braceyourselves.jpg")
    };

    
    

fn make_meme(meme: &Meme) {

    let path = Path::new(&meme.image_path);
    let mut meme_image = image::open(path).unwrap();

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 15.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    let color = Rgba([0u8, 0u8, 255u8, 0u8]);

    draw_text_mut(&mut meme_image, color, 20, 0, scale, &font, &meme.text_top);
    draw_text_mut(&mut meme_image, color, 20, 150, scale, &font, &meme.text_bottom);
    let _ = meme_image.save(Path::new("new_meme.png")).unwrap();

    println!("Your meme has been created.");
}


    const APP_TITLE: &str = "The Meme Machine";
    println!("This App is called {}", APP_TITLE);

    println!("Pick the meme you want to make:");
    println!("1. {}", braceyourselves.name);
    
    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Failed to read line");

    let pick: u32 = pick.trim().parse().expect("Please type a number!");

    if pick == 1{
        println!("You picked Braceyourselves...");
        println!("What is your bottom text going to be?");

        let mut bottom = String::new();

        io::stdin()
            .read_line(&mut bottom)
            .expect("Failed to read line");
        
        braceyourselves.text_bottom = bottom.trim().to_string();
        
        make_meme(&braceyourselves);

    }else{
        println!("Please pick 1 or 2. Thanks.")
    }


}