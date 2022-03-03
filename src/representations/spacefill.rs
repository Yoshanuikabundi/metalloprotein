use crate::chemicals::{AtomPosition, Element};
use crate::elements::ELEMENTRADII;
use crate::representations::{AtomMesh, ElementMaterials, Representation};
use bevy::prelude::*;
use std::hash::Hash;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Default)]
pub struct SpaceFill;

impl Representation for SpaceFill {
    fn spawn_atom(
        &self,
        commands: &mut Commands,
        parent: Entity,
        elem: &Element,
        pos: &AtomPosition,
        atom_mesh: &AtomMesh,
        element_mats: &ElementMaterials,
    ) {
        let mesh = atom_mesh.0.clone();
        let (material, scale) = match elem.atomic_number {
            n @ 0..=118 => (element_mats.0[n as usize].clone(), ELEMENTRADII[n as usize]),
            _ => (element_mats.0[0].clone(), ELEMENTRADII[0]),
        };

        commands.entity(parent).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    material,
                    mesh,
                    transform: Transform::from_translation(pos.0).with_scale(Vec3::splat(scale)),
                    ..Default::default()
                })
                .insert(self.clone());
        });
    }
}
