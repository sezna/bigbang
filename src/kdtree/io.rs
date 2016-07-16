use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use kdtree::particle::Particle;

// For now, data files are text files where there is one particle per line.
// Particles are stored as
// x y z vx vy vz mass radius
// TODO perhaps write the reading so that it doesn't require newlines?

/// Opens a utf8 file with one particle per line, space separated values of the format:
/// x y z vx vy vz mass radius
/// Must have a newline after the final particle.
pub fn open_data_file(file_string: String) -> Vec<Particle> {
    let file_path = Path::new(&file_string);
    let display = file_path.display();
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    }
    let mut tmp_str: String = String::new();
    let mut tmp: Vec<String> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    for i in s.chars() {
        if i != '\n' && i != ' ' {
            tmp_str = format!("{}{}", tmp_str, i);
        } else if i == ' ' {
            tmp.push(tmp_str);
            tmp_str = "".to_string();
        } else {
            tmp.push(tmp_str.clone());
            tmp_str = "".to_string();
            if tmp.len() == 8 {

                let x_val: f64 = tmp[0].parse().unwrap(); // TODO unwraps are bad
                let y_val: f64 = tmp[1].parse().unwrap();
                let z_val: f64 = tmp[2].parse().unwrap();
                let vx_val: f64 = tmp[3].parse().unwrap();
                let vy_val: f64 = tmp[4].parse().unwrap();
                let vz_val: f64 = tmp[5].parse().unwrap();
                let mass_val: f64 = tmp[6].parse().unwrap();
                let radius_val: f64 = tmp[7].parse().unwrap();
                let tmp_part = Particle {
                    x: x_val,
                    y: y_val,
                    z: z_val,
                    vx: vx_val,
                    vy: vy_val,
                    vz: vz_val,
                    mass: mass_val,
                    radius: radius_val,
                };
                particles.push(tmp_part);
                tmp.clear();
            } else {
                println!("vec_len: {}", particles.len());
                println!("Input file invalid.");
                return vec![Particle::new()];
            }
        }
    }
    return particles;
}
// pub fn write_data_file(kdtree: KDTree, file_path: String) {
// let mut file = File::create(file_path).unwrap(); //TODO unwraps are bad
// let mut to_write = traverse_tree(&kdtree);
// let mut to_write_string: String = "".to_string();
// println!("to_write.len() = {}", to_write.len());
// to_write_string = format!("{}", to_write.pop().expect("").as_string());
// while !to_write.is_empty() {
// to_write_string = format!("{}\n{}",
// to_write_string,
// to_write.pop().expect("").as_string());
// }/*
// for i in to_write {
// to_write_string = format!("{}\n{}", to_write_string, i.as_string());
// }*/
// to_write_string = format!("{}\n", to_write_string);
// assert!(file.write(to_write_string.as_bytes()).unwrap() == to_write_string.as_bytes().len());
// }
