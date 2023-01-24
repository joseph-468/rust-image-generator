use image::imageops::overlay;
use std::fs;

fn main() {
    // Assets
    let mut canvas = image::open("images/background.png").unwrap();
    let dirt = image::open("images/dirt.png").unwrap();
    let grass = image::open("images/grass.png").unwrap();

    // Load map
    let map = fs::read_to_string("map.csv").expect("Unable to read file");
    let map = map.replace("\n", "").replace("\r", "");
    let map_vec: Vec<i32> = map.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let asset_array = [&dirt, &grass];

    // Draw images to canvas and save
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in map_vec {
        overlay(&mut canvas, asset_array[i as usize], x * 16, y * 16);
        x += 1;
        if x == 16 {
            x = 0;
            y += 1;
        }
    }

    canvas.save("output.png").unwrap();
}
