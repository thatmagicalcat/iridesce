use glam::Vec2;

#[derive(Debug)]
pub struct AABB {
    pub v1: Vec2,
    pub v2: Vec2,
}

impl AABB {
    pub fn new(v1: Vec2, v2: Vec2) -> Self {
        Self { v1, v2 }
    }

    pub fn center(&self) -> Vec2 {
        (self.v1 + self.v2) * 0.5
    }

    pub fn contains(&self, p: Vec2) -> bool {
        if p.x < self.v1.x || p.x > self.v2.x {
            return false;
        }
        if p.y < self.v1.y || p.y > self.v2.y {
            return false;
        }

        true
    }

    pub fn expand(&mut self, amount: f32) {
        self.v1 -= Vec2::splat(amount);
        self.v2 += Vec2::splat(amount);
    }
}
