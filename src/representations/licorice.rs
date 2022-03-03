use super::ball_and_stick::BallAndStick;
use crate::chemicals::{AtomPosition, Element};
use crate::representations::{ElementMaterials, Representation};
use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone)]
pub struct Licorice(BallAndStick);

impl Licorice {
    pub fn radius(&self) -> f32 {
        self.0.stick_radius()
    }
    pub fn set_radius(&mut self, r: f32) {
        self.0.set_stick_radius(r)
    }
    pub fn from_radius(r: f32) -> Self {
        Self(BallAndStick::from_radii(r, r))
    }
}

impl Default for Licorice {
    fn default() -> Self {
        Self::from_radius(0.2)
    }
}

impl Representation for Licorice {
    fn spawn_bond(
        &self,
        commands: &mut Commands,
        parent: Entity,
        elem: (&Element, &Element),
        pos: (&AtomPosition, &AtomPosition),
        meshes: &mut Assets<Mesh>,
        element_mats: &ElementMaterials,
    ) {
        self.0
            .spawn_bond(commands, parent, elem, pos, meshes, element_mats)
    }
}
