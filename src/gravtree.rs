use entity::{AsEntity, Entity};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use Node;

/// The main struct you will interact with. This is a k-d tree containing all of your gravitational
/// entities.
pub struct GravTree<T: AsEntity + Clone> {
    /// A GravTree consists of a root [[Node]]. A [[Node]] is a recursive binary tree data structure.
    /// Tragically must be public for now for testing reasons. Perhaps could be replaced by various
    /// getter methods later.
    pub root: Node<T>,
    /// This is just the number of entities in the tree. This is used in testing to verify that no
    /// entities are being dropped.
    number_of_entities: usize,
    /// This coefficient determines the granularity of the simulation, i.e. how much each frame of
    /// the simulation actually moves the individual entities.
    time_step: f64, // the time coefficient; how large each simulation frame is time-wise.
}

impl<T: AsEntity + Clone + Send + Sync> GravTree<T> {
    pub fn new(pts: &mut Vec<T>, time_step: f64) -> GravTree<T>
    where
        T: AsEntity,
    {
        let size_of_vec = pts.len();
        GravTree {
            root: Node::<T>::new_root_node(&mut pts[..]),
            number_of_entities: size_of_vec,
            time_step,
        }
    }
    /// Traverses the tree and returns a vector of all entities in the tree.
    pub fn as_vec(&self) -> Vec<T> {
        let node = self.root.clone();
        let mut to_return: Vec<T> = Vec::new();
        if let Some(node) = &node.left {
            to_return.append(&mut node.traverse_tree_helper());
        }
        if let Some(node) = &node.right {
            to_return.append(&mut node.traverse_tree_helper());
        } else {
            to_return.append(
                &mut (node
                    .points
                    .as_ref()
                    .expect("unexpected null node #9")
                    .clone()),
            );
        }
        to_return
    }
    /// Gets the total number of entities contained by this tree.
    pub fn get_number_of_entities(&self) -> usize {
        self.number_of_entities
    }

    /// This function creates a vector of all entities from the tree and applies gravity to them.
    /// Returns a new GravTree.
    // of note: The c++ implementation of this just stores a vector of
    // accelerations and matches up the
    // indexes with the indexes of the entities, and then applies them. That way
    // some memory is saved.
    // I am not sure if this will be necessary or very practical in the rust
    // implementation (I would have to implement indexing in my GravTree struct).
    pub fn time_step(&self) -> GravTree<T> {
        // TODO currently there is a time when the entities are stored twice.
        // Store only accelerations perhaps?
        // First, we extract the entities out into a vector
        let post_gravity_entity_vec: Vec<T> = self.root.traverse_tree_helper();
        // Then, we construct a new grav tree after the gravitational acceleration for each
        // entity has been calculated.
        GravTree::<T>::new(
            &mut post_gravity_entity_vec
                .par_iter()
                .map(|x| {
                    x.apply_velocity(
                        x.as_entity().interact_with(&self.root, self.time_step),
                        self.time_step,
                    )

                })
                .collect(),
            self.time_step,
        )
    }

    // For now, data files are text files where there is one entity per line.
    // entities are stored as
    // x y z vx vy vz mass radius
    // TODO perhaps write the reading so that it doesn't require newlines?

    /// Reads a data file generated by this program. To see the format of this file,
    /// go to [[write_data_file]]. Takes in a file path and a time_step value.
    /// These are not encoded in the data files to allow for SwiftViz to read
    /// the files without issue. There is currently no way to offload theta, max_pts, and time_step
    /// into the data file.
    /// Returns a new GravTree with the data from the file on success, or an error
    /// message if the data format is incorrect.
    /// Panics if the file path provided is incorrect.
    pub fn from_data_file(
        file_string: String,
        time_step: f64,
    ) -> Result<GravTree<Entity>, &'static str> {
        let file_path = Path::new(&file_string);
        let display = file_path.display();
        let mut file = match File::open(&file_path) {
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", display, Error::description(&why))
        }
        let mut tmp_str: String = String::new();
        let mut tmp: Vec<String> = Vec::new();
        let mut entities: Vec<Entity> = Vec::new();
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
                    // In the future, I'd like to make a super-error enum   to contain all the errors that could happen in here.
                    let x_val: f64 = tmp[0].parse().unwrap();
                    let y_val: f64 = tmp[1].parse().unwrap();
                    let z_val: f64 = tmp[2].parse().unwrap();
                    let vx_val: f64 = tmp[3].parse().unwrap();
                    let vy_val: f64 = tmp[4].parse().unwrap();
                    let vz_val: f64 = tmp[5].parse().unwrap();
                    let mass_val: f64 = tmp[6].parse().unwrap();
                    let radius_val: f64 = tmp[7].parse().unwrap();
                    let tmp_part = Entity {
                        x: x_val,
                        y: y_val,
                        z: z_val,
                        vx: vx_val,
                        vy: vy_val,
                        vz: vz_val,
                        mass: mass_val,
                        radius: radius_val,
                    };
                    entities.push(tmp_part);
                    tmp.clear();
                } else {
                    return Err("Input file invalid.");
                }
            }
        }
        Ok(GravTree::new(&mut entities, time_step))
    }

    /// Writes a utf8 file with one entity per line, space separated values of the format:
    /// x y z vx vy vz mass radius. Must have a newline after the final entity.
    /// This is compatible with SwiftVis visualizations.
    pub fn write_data_file(self, file_path: String) {
        let mut file = File::create(file_path).unwrap(); //TODO unwraps are bad
        let mut to_write: Vec<Entity> = self
            .as_vec()
            .iter()
            .map(|x| x.as_entity().clone())
            .collect();
        let mut to_write_string: String;
        to_write_string = to_write.pop().expect("").as_string().to_string();
        while !to_write.is_empty() {
            to_write_string = format!(
                "{}\n{}",
                to_write_string,
                to_write.pop().expect("").as_string()
            );
        }
        to_write_string = format!("{}\n", to_write_string);
        assert_eq!(
            file.write(to_write_string.as_bytes()).unwrap(),
            to_write_string.as_bytes().len()
        );
    }
}
#[test]
fn test_input() {
    let test_tree =
        GravTree::<Entity>::from_data_file("test_files/test_input.txt".to_string(), 0.2);
    assert!(test_tree.unwrap().as_vec().len() == 3601);
}
#[test]
fn test_output() {
    let mut test_vec: Vec<Entity> = Vec::new();
    for _ in 0..1000 {
        test_vec.push(Entity::random_entity());
    }
    let kd = GravTree::new(&mut test_vec, 0.2);
    GravTree::write_data_file(kd, "test_files/test_output.txt".to_string());
    let test_tree =
        GravTree::<Entity>::from_data_file("test_files/test_output.txt".to_string(), 0.2);
    assert!(test_vec.len() == test_tree.unwrap().as_vec().len());
}
