//! World grid simulation logic.
//!
//! This module contains the core simulation grid where particles are stored
//! and updated according to their material behaviors.

use crate::core::behavior::Behavior;
use crate::core::registry::MaterialRegistry;
use std::time::{SystemTime, UNIX_EPOCH};

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

                if material_id != 0 {
                    let behavior = materials.get_behavior(material_id);
                    let new_pos = self.get_behavior_callback(behavior, x, y, &new_cells, self.tick);

                    if let Some((nx, ny)) = new_pos {
                        let new_index = self.get_index(nx, ny);
                        new_cells[new_index] = material_id;
                    } else {
                        new_cells[index] = material_id;
                    }
                }
            }
        }

        self.cells = new_cells;
        self.tick = self.tick.wrapping_add(1);
    }

    fn get_behavior_callback(
        &self,
        behavior: Option<&Behavior>,
        x: u32,
        y: u32,
        new_cells: &Vec<u32>,
        tick: u32,
    ) -> Option<(u32, u32)> {
        match behavior? {
            Behavior::Granular => self.find_fall_position(x, y, new_cells),
            Behavior::Liquid => self.find_liquid_position(x, y, new_cells, tick),
            _ => Some((x, y)),
        }
    }

    fn find_fall_position(&self, x: u32, y: u32, new_cells: &Vec<u32>) -> Option<(u32, u32)> {
        if y + 1 >= self.height {
            return None;
        }

        if self.is_empty(x, y + 1, new_cells) {
            return Some((x, y + 1));
        }

        let can_left = x > 0;
        let can_right = x + 1 < self.width;

        let mut options: Vec<(u32, u32)> = Vec::new();

        if can_left && self.is_empty(x - 1, y + 1, new_cells) {
            options.push((x - 1, y + 1));
        }
        if can_right && self.is_empty(x + 1, y + 1, new_cells) {
            options.push((x + 1, y + 1));
        }

        if !options.is_empty() {
            Some(options[rand_index(options.len())])
        } else {
            None
        }
    }

    fn find_liquid_position(
        &self,
        x: u32,
        y: u32,
        new_cells: &Vec<u32>,
        tick: u32,
    ) -> Option<(u32, u32)> {
        if y + 1 < self.height && self.is_empty(x, y + 1, new_cells) {
            return Some((x, y + 1));
        }

        let can_left = x > 0;
        let can_right = x + 1 < self.width;
        let can_down = y + 1 < self.height;

        let mut below_options: Vec<(u32, u32)> = Vec::new();

        if can_down && can_left && self.is_empty(x - 1, y + 1, new_cells) {
            below_options.push((x - 1, y + 1));
        }
        if can_down && can_right && self.is_empty(x + 1, y + 1, new_cells) {
            below_options.push((x + 1, y + 1));
        }

        if !below_options.is_empty() {
            return Some(below_options[rand_index(below_options.len())]);
        }

        let mut side_options: Vec<(u32, u32)> = Vec::new();

        if can_left && self.is_empty(x - 1, y, new_cells) {
            side_options.push((x - 1, y));
        }
        if can_right && self.is_empty(x + 1, y, new_cells) {
            side_options.push((x + 1, y));
        }

        if !side_options.is_empty() {
            let should_settle = (tick.wrapping_add(x).wrapping_add(y)) % 3 == 0;

            if should_settle {
                None
            } else {
                Some(side_options[rand_index(side_options.len())])
            }
        } else {
            None
        }
    }

    fn is_empty(&self, x: u32, y: u32, new_cells: &Vec<u32>) -> bool {
        self.get_cell(x, y).is_none() && new_cells[self.get_index(x, y)] == 0
    }
}

fn rand_index(len: usize) -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    nanos % len
}
