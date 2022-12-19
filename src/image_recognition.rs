use image::GenericImageView;


pub fn run(){
    let mut i = true;
    //while i = false {
    //
    //}

    let img = image::open("src/images/example.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    let (width,height) = img.dimensions();
    while i {
        for y in 0..height{
            for x in 0..1344{
                let pix = img.get_pixel(x,y);
                if (pix[0] == 156 && pix[1] == 150 && pix[2] == 117){
                    i = false;
                    let newX = x + 20;
                    //call mouse click function with x y as coords
                }
            }
        }
    }
    
}