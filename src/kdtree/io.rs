use std::error::Error;
use std::fs::File;
use kdtree::{KDTree, traverse_tree};
use std::io::prelude::*;
use std::path::Path;
use kdtree::particle::Particle;

// For now, data files are text files where there is one particle per line. Particles are stored as
// x y z vx vy vz mass radius

/// Opens a utf8 file with one particle per line, space separated values of the format:
/// x y z vx vy vz mass radius
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
        Ok(_) => print!("{} contains:\n{}", display,s),
    }
    let mut tmp_str:String = "".to_string();
    let mut tmp:Vec<String> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    for i in s.chars() {
        if i != '\n' && i != ' ' {
            println!("found character: {}", i);
            tmp_str = format!("{}{}", tmp_str, i);
        }
        else if i == ' ' {
            tmp.push(tmp_str);
            tmp_str = "".to_string();
        }
        else {
            tmp.push(tmp_str.clone());
            tmp_str = "".to_string();
            println!("creating particle");
            if tmp.len() == 8 {
                println!("39 {}", tmp[0]);
                let x_val: f64 = tmp[0].parse().unwrap(); // TODO unwraps are bad
                println!("41");
                let y_val: f64 = tmp[1].parse().unwrap();
                println!("43");
                let z_val: f64 = tmp[2].parse().unwrap();
                println!("45");
                let vx_val:f64 = tmp[3].parse().unwrap();
                println!("47");
                let vy_val:f64 = tmp[4].parse().unwrap();
                println!("49");
                let vz_val:f64 = tmp[5].parse().unwrap();
                println!("51");
                let mass_val:f64 = tmp[6].parse().unwrap();
                println!("53");
                let radius_val:f64 = tmp[7].parse().unwrap();
                println!("55");
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
            }
            else {
                println!("vec_len: {}", particles.len());
                println!("Input file invalid.");
                return vec![Particle::new()];
            }
        }
    }
    return particles;
}

pub fn save_to_file(kdtree: KDTree, file_path: String) {
    let mut file = File::create(file_path).unwrap(); //TODO unwraps are bad
    let to_write = traverse_tree(&kdtree);
    let mut to_write_string:String = "".to_string();
    for i in to_write {
        to_write_string = format!("{}\n{}", to_write_string, i.as_string());
    }
    assert!(file.write(to_write_string.as_bytes()).unwrap() == to_write_string.as_bytes().len());
    
}
