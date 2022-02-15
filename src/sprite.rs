// use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
pub struct CharacterAnimation(pub Timer, pub bool, pub usize);

pub struct Plugin;
//
// impl BevyPlugin for Plugin {
//     fn build(&self, app: &mut App) {}
// }
