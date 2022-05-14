#![feature(derive_default_enum)]
use bevy::prelude::*;

// TODO: Implement file reading for settings states
fn main() {
    App::new()
        .insert_resource::<Cells>(Cells::new(10, 10, 10, 5, None))
        .insert_resource::<Rule>(Rule {
            survive: vec![4],
            birth: vec![4],
            neighbourhood: NeighbourhoodType::Moore,
            state: 5,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Clone)]
struct Cells {
    x_len: usize,
    y_len: usize,
    z_len: usize,
    cells: Vec<Cell>,
}

#[derive(Default)]
enum NeighbourhoodType {
    #[default]
    Moore,
    VonNeumann,
}

#[derive(Default)]
struct Rule {
    survive: Vec<u32>,
    birth: Vec<u32>,
    neighbourhood: NeighbourhoodType,
    state: u8,
}

#[derive(Clone)]
struct Cell {
    alive: bool, // false for dead
    state: u8,
}

impl Cells {
    fn new(
        x_len: usize,
        y_len: usize,
        z_len: usize,
        lifetime: u8,
        start_state: Option<Vec<bool>>,
    ) -> Cells {
        let mut cells: Vec<Cell> = Vec::new();
        match start_state {
            Some(start_cells) => {
                for cell in start_cells {
                    let _ = &cells.push(Cell {
                        alive: cell,
                        state: lifetime,
                    });
                }
            }
            None => {
                cells = vec![
                    Cell {
                        alive: false,
                        state: 0
                    };
                    x_len * y_len * z_len
                ]
            }
        }

        assert!(cells.len() == x_len * y_len * z_len);

        return Cells {
            x_len,
            y_len,
            z_len,
            cells,
        };
    }

    fn flat_index_to_xyz(&self, index: usize) -> (usize, usize, usize) {
        (
            index % self.x_len,
            index / self.x_len,
            index / (self.x_len * self.y_len),
        )
    }

    fn xyz_to_flat_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.x_len + z * self.x_len * self.y_len
    }

    fn get_neighbour_indices(&self, index: usize) -> Vec<usize> {
        let (x_index, y_index, z_index) = self.flat_index_to_xyz(index);

        let mut indices: Vec<usize> = Vec::new();

        let slice: Vec<i32> = vec![-1, 0, 1];

        for i in &slice {
            for j in &slice {
                for k in &slice {
                    let new_x = x_index as i32 + *i;
                    let new_y = y_index as i32 + *j;
                    let new_z = z_index as i32 + *k;

                    if new_x > -1 && new_y > -1 && new_z > -1 {
                        let new_index = new_x as usize
                            + new_y as usize * self.x_len
                            + new_z as usize * self.x_len * self.y_len;

                        if new_index != index {
                            indices.push(new_index)
                        };
                    }
                }
            }
        }

        indices
    }
}

fn setup(commands: Commands, cells: Res<Cells>) -> () {
    println!("{:?}", cells.cells.len());
}

fn calculate_next_state(commands: Commands, cells: ResMut<Cells>, rule: Res<Rule>) {
    let mut next_state: Cells = cells.clone();

    for index in 0..cells.x_len * cells.y_len * cells.z_len {
        let this_cell = &cells.cells[index];

        let mut sum = 0;
        for neighbour in cells.get_neighbour_indices(index) {
            sum += cells.cells[neighbour].alive as u32;
        }

        if this_cell.alive {
            if rule.survive.contains(&sum) {
                next_state.cells[index] = Cell {
                    alive: true,
                    state: rule.state,
                };
            } else {
                next_state.cells[index] = Cell {
                    alive: false,
                    state: rule.state,
                };
            }
        } else {
            if rule.birth.contains(&sum) {
                next_state.cells[index] = Cell {
                    alive: true,
                    state: rule.state,
                }
            } else {
                next_state.cells[index] = Cell {
                    alive: false,
                    state: this_cell.state - 1,
                }
            }
        }
    }
}
