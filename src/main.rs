mod kdtree;
extern crate rand;

fn main() {
  
    let mut to_be_inserted:Vec<kdtree::Particle> = Vec::new();
    for i in 0..100000 {
        to_be_inserted.push(kdtree::Particle::random_particle());
    }
	kdtree::new_kdtree(to_be_inserted, 3);
    println!("Hello, world");
}
