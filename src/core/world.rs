//! World grid simulation logic.
//!
//! This module contains the core simulation grid where particles are stored.
//! Movement behavior is delegated to the behaviors module.

use crate::core::behaviors;
use crate::core::registry::MaterialRegistry;

pub struct World {
    pub width: u32,
    pub height: u32,
    cells: Vec<u32>,
    tick: u32,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        World {
            width,
            height,
            cells: vec![0; (width * height) as usize],
            tick: 0,
        }
    }

    pub fn set_cell(&mut self, x: u32, y: u32, material_id: u32) {
        if x < self.width && y < self.height {
            let index = self.get_index(x, y);
            self.cells[index] = material_id;
        } else {
            log::error!("Attempted to set cell ({}, {}) out of bounds", x, y);
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.width && y < self.height {
            let material_id = self.cells[self.get_index(x, y)];
            if material_id == 0 {
                None
            } else {
                Some(material_id)
            }
        } else {
            None
        }
    }

    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width == self.width && new_height == self.height {
            return;
        }

        let mut new_cells = vec![0u32; (new_width * new_height) as usize];

        let copy_width = self.width.min(new_width);
        let copy_height = self.height.min(new_height);

        for y in 0..copy_height {
            for x in 0..copy_width {
                let old_index = self.get_index(x, y);
                let new_index = (y * new_width + x) as usize;
                new_cells[new_index] = self.cells[old_index];
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.cells = new_cells;
    }

    pub fn clear(&mut self) {
        let len = self.cells.len();
        self.cells = vec![0; len];
        self.tick = 0;
    }

    pub fn cells(&self) -> impl Iterator<Item = (u32, u32, u32)> + '_ {
        self.cells.iter().enumerate().filter_map(|(i, &id)| {
            if id != 0 {
                let x = (i % self.width as usize) as u32;
                let y = (i / self.width as usize) as u32;
                Some((x, y, id))
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, materials: &MaterialRegistry) {
        let mut new_cells = vec![0u32; self.cells.len()];

        let left_to_right = self.tick % 2 == 0;

        for y in (0..self.height).rev() {
            let x_iter: Box<dyn Iterator<Item = u32>> = if left_to_right {
                Box::new(0..self.width)
            } else {
                Box::new((0..self.width).rev())
            };

            for x in x_iter {
                let index = self.get_index(x, y);
                let material_id = self.cells[index];

                if material_id == 0 {
                    continue;
                }
                if new_cells[index] != 0 {
                    continue;
                }

                let behavior = materials.get_behavior(material_id);

                let ctx = behaviors::BehaviorContext {
                    x,
                    y,
                    material_id,
                    tick: self.tick,
                    width: self.width,
                    height: self.height,
                    cells: &self.cells,
                    new_cells: &new_cells,
                    materials,
                };

                let new_pos = match behavior {
                    Some(b) => behaviors::dispatch(b, &ctx),
                    None => None,
                };

                match new_pos {
                    Some((nx, ny)) => {
                        let new_index = self.get_index(nx, ny);

                        if new_cells[new_index] == 0 {
                            new_cells[new_index] = material_id;
                        } else {
                            new_cells[index] = material_id;
                        }
                    }
                    None => {
                        new_cells[index] = material_id;
                    }
                }
            }
        }

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let material_id = new_cells[index];

                if material_id == 0 {
                    continue;
                }

                if y + 1 < self.height {
                    let below_index = self.get_index(x, y + 1);
                    let below_id = new_cells[below_index];

                    if below_id != 0 && below_id != material_id {
                        let my_behavior = materials.get_behavior(material_id);
                        let below_behavior = materials.get_behavior(below_id);

                        let my_can_move = matches!(
                            my_behavior,
                            Some(
                                crate::core::behavior::Behavior::Granular
                                    | crate::core::behavior::Behavior::Liquid
                            )
                        );
                        let below_can_move = matches!(
                            below_behavior,
                            Some(
                                crate::core::behavior::Behavior::Granular
                                    | crate::core::behavior::Behavior::Liquid
                            )
                        );

                        if !my_can_move || !below_can_move {
                            continue;
                        }

                        let my_density = match materials.get_density(material_id) {
                            Some(d) => d,
                            None => continue,
                        };
                        let below_density = match materials.get_density(below_id) {
                            Some(d) => d,
                            None => continue,
                        };

                        if my_density > below_density {
                            new_cells[index] = below_id;
                            new_cells[below_index] = material_id;
                        }
                    }
                }
            }
        }

        self.cells = new_cells;
        self.tick = self.tick.wrapping_add(1);
    }
}
