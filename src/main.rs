extern crate image;

fn main() {
    let imgs = [
        image::open("map_gray_q1.jpg").unwrap().to_luma(),
        image::open("map_gray_q10.jpg").unwrap().to_luma(),
        image::open("map_gray_q20.jpg").unwrap().to_luma(),
        image::open("map_gray_q30.jpg").unwrap().to_luma(),
        image::open("map_gray_q40.jpg").unwrap().to_luma(),
        image::open("map_gray_q50.jpg").unwrap().to_luma(),
        image::open("map_gray_q60.jpg").unwrap().to_luma(),
        image::open("map_gray_q70.jpg").unwrap().to_luma(),
        image::open("map_gray_q80.jpg").unwrap().to_luma(),
        image::open("map_gray_q90.jpg").unwrap().to_luma(),
        image::open("map_gray_q100.jpg").unwrap().to_luma(),
        ];
    let png = image::open("map_gray.png").unwrap().to_luma();
    let (pngdimx, pngdimy) = png.dimensions();
    let numpixels = pngdimx * pngdimy as u32;

    for img in &imgs {
        let mut accum_error :u32 = 0;
        for (x, y, px) in img.enumerate_pixels(){
            let pngvalue = png.get_pixel(x, y).data[0] as i32;
            let jpgvalue = px.data[0] as i32;
            let error = (jpgvalue - pngvalue).abs() as u32;

            accum_error += error * error as u32;
        }
        let mse = (accum_error / numpixels) as f64;
        let psnr = 20.0_f64 * (255.0_f64 / mse.sqrt()).ln();
        println!("{}", psnr);
    }
}
