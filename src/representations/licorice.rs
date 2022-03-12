use super::ball_and_stick::BallAndStick;
use crate::chemicals::{AtomPosition, Element};
use crate::prelude::*;
use crate::representations::{ElementMaterials, Representation};

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone)]
pub struct Licorice {
    radius: u32,
}

impl Licorice {
    pub const fn new() -> Self {
        Self { radius: 20 }
    }

    pub fn from_radius(r: f32) -> Self {
        Self {
            radius: (r * 100.0) as u32,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius as f32 / 100.0
    }

    pub fn set_radius(&mut self, new: f32) {
        self.radius = (new * 100.0) as u32;
    }
}

impl Default for Licorice {
    fn default() -> Self {
        Self::new()
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
        BallAndStick::from_radii(0.0, self.radius()).spawn_bond(
            commands,
            parent,
            elem,
            pos,
            meshes,
            element_mats,
        )
    }

    fn ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
        let mut r = self.radius();
        ui.label(Self::name());
        ui.add(bevy_egui::egui::Slider::new(&mut r, 0.0..=1.0));
        self.set_radius(r);
    }

    fn name() -> &'static str {
        "Licorice"
    }
}
