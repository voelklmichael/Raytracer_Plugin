use crate::{Vec2, Intersection};

pub struct Ray {
    position: Vec2,
    direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn direction(&self) -> Vec2 {
        self.direction
    }
    pub fn position(&self) -> Vec2 {
        self.position
    }
    pub fn propagete(&mut self, intersection: Intersection) {
        self.position = self.position + self.direction * intersection.distance;
        self.direction = self.direction.reflect(intersection.normal)
    }
}