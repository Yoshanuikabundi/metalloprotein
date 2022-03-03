use crate::chemicals::{AtomPosition, Element};
use crate::representations::{AtomMesh, ElementMaterials, Representation};
use crate::wprintln;
use bevy::prelude::*;
use std::hash::Hash;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone)]
pub struct BallAndStick {
    ball_radius: u32,
    stick_radius: u32,
}

impl Default for BallAndStick {
    fn default() -> Self {
        Self {
            ball_radius: 30,
            stick_radius: 10,
        }
    }
}

impl BallAndStick {
    pub fn from_radii(ball: f32, stick: f32) -> Self {
        let mut out = Self::default();
        out.set_ball_radius(ball);
        out.set_stick_radius(stick);
        out
    }

    pub fn ball_radius(&self) -> f32 {
        self.ball_radius as f32 / 100.0
    }

    pub fn stick_radius(&self) -> f32 {
        self.stick_radius as f32 / 100.0
    }

    pub fn set_ball_radius(&mut self, new: f32) {
        self.ball_radius = (new * 100.0) as u32;
    }

    pub fn set_stick_radius(&mut self, new: f32) {
        self.stick_radius = (new * 100.0) as u32;
    }
}

impl Representation for BallAndStick {
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
        let material = match elem.atomic_number {
            n @ 0..=118 => element_mats.0[n as usize].clone(),
            _ => element_mats.0[0].clone(),
        };

        commands.entity(parent).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    material,
                    mesh,
                    transform: Transform::from_translation(pos.0)
                        .with_scale(Vec3::splat(self.ball_radius())),
                    ..Default::default()
                })
                .insert(self.clone());
        });
    }

    fn spawn_bond(
        &self,
        commands: &mut Commands,
        parent: Entity,
        elem: (&Element, &Element),
        pos: (&AtomPosition, &AtomPosition),
        meshes: &mut Assets<Mesh>,
        element_mats: &ElementMaterials,
    ) {
        // Construct the materials for each half of the bond
        let material_a = match elem.0.atomic_number {
            n @ 0..=118 => element_mats.0[n as usize].clone(),
            _ => element_mats.0[0].clone(),
        };

        let material_b = match elem.1.atomic_number {
            n @ 0..=118 => element_mats.0[n as usize].clone(),
            _ => element_mats.0[0].clone(),
        };

        let xyz_a = pos.0 .0;
        let xyz_b = pos.1 .0;

        let radius = self.stick_radius();
        let depth = (xyz_a - xyz_b).length() / 2.0;

        // Construct the mesh for the half-bond
        let mesh = meshes.add(Mesh::from(shape::Capsule {
            depth,
            radius,
            ..Default::default()
        }));

        // Place the two half bonds in the right spot and rotation
        let transform_a = Transform::from_translation((3.0 * xyz_a + xyz_b) / 4.0).with_rotation(
            Quat::from_rotation_arc_colinear(Vec3::Y, (xyz_a - xyz_b).normalize()),
        );
        let transform_b = Transform::from_translation((xyz_a + 3.0 * xyz_b) / 4.0).with_rotation(
            Quat::from_rotation_arc_colinear(Vec3::Y, (xyz_a - xyz_b).normalize()),
        );

        commands.entity(parent).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    material: material_a,
                    mesh: mesh.clone(),
                    transform: transform_a,
                    ..Default::default()
                })
                .insert(self.clone());
            parent
                .spawn_bundle(PbrBundle {
                    material: material_b,
                    mesh,
                    transform: transform_b,
                    ..Default::default()
                })
                .insert(self.clone());
        });
    }
}
