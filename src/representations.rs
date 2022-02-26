use super::chemicals::Element;
use super::{AtomMesh, ElementMaterials};
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct SpaceFill;

pub(crate) fn spacefill_rep(
    mut commands: Commands,
    q_parent: Query<&Children, With<SpaceFill>>,
    q_child: Query<&Element, Without<SpaceFill>>,
    atom_mesh: Res<AtomMesh>,
    element_mats: Res<ElementMaterials>,
) {
    for children in q_parent.iter() {
        for &child in children.iter() {
            if let Ok(Element { atomic_number: n }) = q_child.get(child) {
                let mesh = atom_mesh.0.clone();
                let mat = match n {
                    0..=118 => element_mats.0[*n as usize].clone(),
                    _ => element_mats.0[0].clone(),
                };

                commands.entity(child).insert_bundle((SpaceFill, mesh, mat));
            }
        }
    }
}
