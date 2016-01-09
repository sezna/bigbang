mod kdtree;

fn main() {
  
    let to_be_inserted:Vec<kdtree::Particle> = vec![kdtree::Particle{vx: 0.0, vy: 0.0, vz: 0.0, x:
        1.0, y: 1.0, z: 0.1},kdtree::Particle{vx: 0.0, vy: 0.0, vz: 0.0, x:
        1.1, y: 1.1, z: 10.0},kdtree::Particle{vx: 0.0, vy: 0.0, vz: 0.0, x:
        1.2, y: 1.2, z: 20.1},kdtree::Particle{vx: 0.0, vy: 0.0, vz: 0.0, x:
        0.9, y: 1.5, z: 100.0},kdtree::Particle{vx: 0.0, vy: 0.0, vz: 0.0, x:
        1.3, y: 1.6, z: 50.1}];
	kdtree::new_kdtree(to_be_inserted, 3);
    println!("Hello, world");
}
