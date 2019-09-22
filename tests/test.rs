extern crate bigbang;
use bigbang::{AsEntity, CollisionResult, Entity, GravTree};

#[derive(Clone, PartialEq)]
struct MyEntity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
}

impl AsEntity for MyEntity {
    fn as_entity(&self) -> Entity {
        return Entity {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            mass: if self.radius < 1. { 0.5 } else { 105. },
        };
    }

    fn apply_velocity(&self, collision_result: CollisionResult, time_step: f64) -> Self {
        let (vx, vy, vz) = collision_result.velocity;
        let (x, y, z) = collision_result.position;
        MyEntity {
            vx,
            vy,
            vz,
            x: x + (vx * time_step),
            y: y + (vy * time_step),
            z: z + (vz * time_step),
            radius: self.radius,
        }
    }

    fn set_position(&self, position: (f64, f64, f64)) -> Self {
        let (x, y, z) = position;
        MyEntity {
            x,
            y,
            z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
        }
    }
}

impl MyEntity {
    pub fn random_entity() -> MyEntity {
        MyEntity {
            vx: 0f64,
            vy: 0f64,
            vz: 0f64,
            x: rand::random::<f64>() * 50f64,
            y: rand::random::<f64>() * 50f64,
            z: rand::random::<f64>() * 50f64,
            radius: rand::random::<f64>() / 10f64,
        }
    }
}

#[test]
fn test_traversal() {
    let mut vec: Vec<MyEntity> = Vec::new();
    for _ in 0..100 {
        let entity = MyEntity::random_entity();
        vec.push(entity);
    }
    let vec_clone = vec.clone();
    let tree = GravTree::new(&vec, 0.2);
    let traversed_vec = tree.as_vec();
    let mut all_found = true;
    for i in vec_clone {
        if !traversed_vec.contains(&i) {
            all_found = false;
        }
    }

    assert!(all_found);
}

#[test]
fn test_time_step() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..1000 {
        let entity = MyEntity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let test_tree = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2);
    let after_time_step = test_tree.time_step();
    assert_eq!(after_time_step.as_vec().len(), 1000);
}

#[test]
fn test_tree() {
    let mut vec_that_wants_to_be_a_kdtree: Vec<MyEntity> = Vec::new();
    for _ in 0..100 {
        for _ in 0..100 {
            for _ in 0..10 {
                let entity = MyEntity::random_entity();
                vec_that_wants_to_be_a_kdtree.push(entity);
            }
        }
    }
    let kdtree_test = GravTree::new(&vec_that_wants_to_be_a_kdtree, 0.2);
    assert!(kdtree_test.get_number_of_entities() == 100_000);
    // kdtree_test.display_tree();
    go_to_edges(kdtree_test, 14usize, 15usize);
    let mut smaller_vec: Vec<MyEntity> = Vec::new();
    for _ in 0..50 {
        let entity = MyEntity::random_entity();
        smaller_vec.push(entity);
    }
    // Testing center of mass assignment
    let mut vector = vec![
        Entity {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
        Entity {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            x: 2.0,
            y: 1.0,
            z: 3.0,
            mass: 2.0,
            radius: 1.0,
        },
    ];
    let center_of_mass_test = GravTree::new(&vector, 0.2);
    assert!(center_of_mass_test.root.center_of_mass == (1.5, 1.5, 3.0));
}
/// This function is used for testing. It checks the number of nodes on each side, along the "edge" of the tree.
/// left_nodes is how many nodes you expect to see along the left size, and right_nodes is how many you expect to
///  see along the right.
fn go_to_edges(grav_tree: GravTree<MyEntity>, left_nodes: usize, right_nodes: usize) {
    let mut count_of_nodes = 0;
    let mut node = grav_tree.root.left.expect("null root node\n");
    let mut node2 = node.clone();
    while node.left.is_some() {
        count_of_nodes += 1;
        node = node.left.expect("unexpected null node #1\n");
    }
    assert_eq!(count_of_nodes, left_nodes);
    count_of_nodes = 0;
    while node2.right.is_some() {
        count_of_nodes += 1;
        node2 = node2.right.expect("unexpected null node #2\n");
    }
    assert_eq!(count_of_nodes, right_nodes);
}
