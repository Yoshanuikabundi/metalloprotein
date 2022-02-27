use crate::chemicals::{AtomPosition, BondElements, BondPositions, Element};
use crate::elements::ELEMENTRADII;
use crate::representations::{
    AtomMesh, BondMesh, ElementMaterials, Representation, RepresentationList,
};
use bevy::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone)]
pub(crate) struct SpaceFill;

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
        let &AtomPosition(x, y, z) = pos;

        commands.entity(parent).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    material,
                    mesh,
                    transform: Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
                    ..Default::default()
                })
                .insert(self.clone());
        });
    }

    fn spawn_bond(
        &self,
        _commands: &mut Commands,
        _parent: Entity,
        _elem: &BondElements,
        _pos: &BondPositions,
        _atom_mesh: &BondMesh,
        _element_mats: &ElementMaterials,
    ) {
        ()
    }
}

#[derive(Component, Default, Debug)]
pub(crate) struct SpaceFillList(HashSet<SpaceFill>);

impl RepresentationList for SpaceFillList {
    type Rep = SpaceFill;

    fn get_set(&self) -> &HashSet<Self::Rep> {
        &self.0
    }

    fn get_set_mut(&mut self) -> &mut HashSet<Self::Rep> {
        &mut self.0
    }
}
