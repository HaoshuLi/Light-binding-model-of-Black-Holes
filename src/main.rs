extern crate image;
extern crate core;

use std::fs::File;
use std::path::Path;
use core::ops::Mul;

use image::{GenericImage,Pixel};

extern crate nalgebra as na;
use na::*;

mod ode;
use ode::solve_final_result;

const M : f64 = 1.0;
const E : f64 = 0.1;
const L : f64 = 1.4;
const R0 : f64 = 30.0;
const R1 : f64 = 15.0;

fn square(x : f64) -> f64 {
    x*x
}
fn cube(x : f64) -> f64 {
    x*x*x
}

fn f1(_ : f64, ys : &Vec<f64>) -> f64 {
    ys[1]
}
fn f2(_ : f64, ys : &Vec<f64>) -> f64 {
    -M*E*E/(1.0-2.0*M/ys[0])/square(ys[0])+M/(1.0-2.0*M/ys[0])/square(ys[0])*square(ys[1])+(1.0-2.0*M/ys[0])*square(L)/cube(ys[0])
}
fn f3(_ : f64, ys : &Vec<f64>) -> f64 {
    L/square(ys[0])
}


fn term(ys : &Vec<f64>) -> bool {
    (-ys[0] * ys[2].cos() >= R1)
}
fn fail(ys : &Vec<f64>) -> bool {
    (ys[0]<2.0*M) || (ys[0] * ys[2].cos() > R0+1.0) || ((ys[0] * ys[2].sin()).abs() > 60.0)
}

fn main() {
    let img = image::open(&Path::new("sample.jpg")).unwrap();

    let (imgx, imgy) = img.dimensions();
    let half_imgx : u32 = imgx / 2;
    let half_imgy : u32 = imgy / 2;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let fs : Vec<fn(f64,&Vec<f64>)->f64> = vec![f1 , f2, f3];

    const R : f64 = R0+R1;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let fx = (x as f64)-(half_imgx as f64);
        let fy = (y as f64)-(half_imgy as f64);
        let dist : f64 = (square(fx) + square(fy)).sqrt()/25.0;
        //let theta : f64 = (dist/R).atan();
        if dist < 2.0*M {
            *pixel = image::Rgba([0, 0, 0, 255]) ; continue;
        }
        let ws = vec![R0, -L/(dist/R)/R0, 0.0];

        match solve_final_result(&fs, 0.0, term, fail, &ws, 2., 0.3) {
            None => { *pixel = image::Rgba([0, 0, 0, 255]) ; },
            Some(result) => {
                let r : f64 = result[0];
                let phi : f64 = result[2];
                let t_dist : f64 = r * (phi.sin());

                let fx1 : f64 = (t_dist/dist*fx) + (half_imgx as f64);
                let fy1 : f64 = (t_dist/dist*fy) + (half_imgy as f64);
                
                let s_x : i32 = fx1 as i32;
                let s_y : i32 = fy1 as i32;
                let b_x : i32 = s_x+1;
                let b_y : i32 = s_y+1;
                
                if s_x>=0 && b_x<(imgx as i32) && s_y>=0 && b_y<(imgy as i32) {
                    let r1 : f64; let r2 : f64; let r3 : f64; let r4 : f64;
                    let g1 : f64; let g2 : f64; let g3 : f64; let g4 : f64;
                    let b1 : f64; let b2 : f64; let b3 : f64; let b4 : f64;
                    let a1 : f64; let a2 : f64; let a3 : f64; let a4 : f64;

                    {
                        let p1 = img.get_pixel(s_x as u32, s_y as u32);
                        let sli : &[u8] = p1.channels();
                        r1 = sli[0] as f64; g1 = sli[1] as f64; b1 = sli[2] as f64; a1 = sli[3] as f64;
                        let p2 = img.get_pixel(s_x as u32, b_y as u32);
                        let sli : &[u8] = p2.channels();
                        r2 = sli[0] as f64; g2 = sli[1] as f64; b2 = sli[2] as f64; a2 = sli[3] as f64;
                        let p3 = img.get_pixel(s_x as u32, b_y as u32);
                        let sli : &[u8] = p3.channels();
                        r3 = sli[0] as f64; g3 = sli[1] as f64; b3 = sli[2] as f64; a3 = sli[3] as f64;
                        let p4 = img.get_pixel(s_x as u32, b_y as u32);
                        let sli : &[u8] = p4.channels();
                        r4 = sli[0] as f64; g4 = sli[1] as f64; b4 = sli[2] as f64; a4 = sli[3] as f64;
                    }
                    
                    let x1 : f64 = s_x as f64;
                    let x2 : f64 = b_x as f64;
                    let y1 : f64 = s_y as f64;
                    let y2 : f64 = b_y as f64;

                    let pos : Vector4<f64> = Vector4::new(fx1, fy1, fx1*fy1, 1.);
                    
                    let mut mat : Matrix4<f64> = Matrix4::new(x1, y1, x1*y1, 1., x1, y2, x1*y2, 1., x2, y1, x2*y1, 1., x2, y2, x2*y2, 1.);
                    if mat.inverse_mut() == false {
                        *pixel = image::Rgba([0, 0, 0, 255]);
                        continue;
                    }
                    let vec : Vector4<f64> = Vector4::new(r1,r2,r3,r4);
                    let factor : Vector4<f64> = mat.mul(vec);
                    let r : f64 = pos.dot(&factor);

                    let vec : Vector4<f64> = Vector4::new(g1,g2,g3,g4);
                    let factor : Vector4<f64> = mat.mul(vec);
                    let g : f64 = pos.dot(&factor);

                    let vec : Vector4<f64> = Vector4::new(b1,b2,b3,b4);
                    let factor : Vector4<f64> = mat.mul(vec);
                    let b : f64 = pos.dot(&factor);

                    let vec : Vector4<f64> = Vector4::new(a1,a2,a3,a4);
                    let factor : Vector4<f64> = mat.mul(vec);
                    let a : f64 = pos.dot(&factor);

                    *pixel = image::Rgba([r as u8, g as u8, b as u8, a as u8]);
                }
                else {
                    *pixel = image::Rgba([0, 0, 0, 255]);
                }
            },
        }
        println!("{} {}", x, y);
    }

    let ref mut fout = File::create(&Path::new("result.png")).unwrap();

    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
}

            
