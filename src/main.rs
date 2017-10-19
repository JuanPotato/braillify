/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate getopts;

use getopts::Options;
use std::env;

use std::path::Path;

use image::imageops::{dither, BiLevel};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("b", "braille", "output braille, include with -r for both");
    opts.optflag("r", "rect", "output rectangles, will output alone if ran without -b");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut image = match image::open(&Path::new(&input)) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error occured while trying to get '{}'. It may not exist", input);
            std::process::exit(1);
        }
    }.to_luma();

    dither(&mut image, &BiLevel);
    
    let do_braille = !matches.opt_present("r") || matches.opt_present("b");

    if matches.opt_present("r") {
        println!("{}", &generate_blocks(&image));
    }

    if do_braille {
        println!("{}", &generate_braille(&image));
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} IMAGE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn generate_braille(image: &image::GrayImage) -> String {
    lazy_static! {
        static ref BRAILLE_CHARS: Vec<char> =
            vec!['⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', 
                '⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', 
                '⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧', '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', 
                '⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', 
                '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇', '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏', 
                '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟', 
                '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯', 
                '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿', 
                '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', 
                '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', 
                '⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯', 
                '⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', 
                '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏', 
                '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟', 
                '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯', 
                '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷', '⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿'];

        static ref BRAILLE_TEMPLATE: Vec<u8> =
            vec![0, 3,
                 1, 4,
                 2, 5,
                 6, 7];
    }

    generate(image, &BRAILLE_CHARS, &BRAILLE_TEMPLATE, 2, 4)
}

fn generate_blocks(image: &image::GrayImage) -> String {
    lazy_static! {
        static ref BLOCKS_CHARS: Vec<char> =
            vec![' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█'];

        static ref BLOCKS_TEMPLATE: Vec<u8> =
            vec![0, 1,
                 2, 3];
    }

    generate(image, &BLOCKS_CHARS, &BLOCKS_TEMPLATE, 2, 2)
}

fn generate(img: &image::GrayImage, characters: &[char], dot_template: &[u8], template_w: u64, template_h: u64) -> String {
    let img_w = img.width() as u64;
    let img_h = img.height() as u64;

    let chars_w = ceil_multiple(img_w, template_w) / template_w;
    let chars_h = ceil_multiple(img_h, template_h) / template_h;

    let mut res = String::with_capacity(((chars_w + 1) * chars_h) as usize);

    for y in 0..chars_h {
        for x in 0..chars_w {
            let mut char_index = 0;
            
            for dot_x in 0..template_w {
                for dot_y in 0..template_h {
                    let pixel_x = x * template_w + dot_x;
                    let pixel_y = y * template_h + dot_y;

                    if pixel_x >= img_w || pixel_y >= img_h {
                        continue
                    }

                    let pixel = img.get_pixel(pixel_x as u32, pixel_y as u32);

                    
                    char_index |= (pixel.data[0] / 128) << dot_template[(dot_y * template_w + dot_x) as usize];
                }
            }

            res.push(characters[char_index as usize]);
        }

        res.push('\n');
    }

    res.pop();

    res
}

fn ceil_multiple(n: u64, m: u64) -> u64 {
    ((n + m - 1) / m) * m
}
