use bevy::prelude::*;

pub fn iso_to_world(p: &Vec2) -> Vec2 {
    Vec2::new((p.x - p.y) * 8., -(p.y + p.x) * 4.)
}

pub fn project_iso(p: &Vec2) -> Vec2 {
    Vec2::new((p.x / 16.) - (p.y / 8.), (-p.x / 16.) - (p.y / 8.))
}
