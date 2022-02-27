use crate::chemicals::{AtomPosition, BondElements, BondPositions, Element};
use crate::ElementMaterials;
use bevy::prelude::*;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Difference;
use std::collections::HashSet;
use std::hash::Hash;

mod spacefill;
pub(crate) use spacefill::{SpaceFill, SpaceFillList};

pub(crate) trait Representation: Component + Eq + Hash + Clone + std::fmt::Debug {
    fn spawn(
        &self,
        commands: &mut Commands,
        parent: Entity,
        children: &Children,
        q_atoms: &Query<(&Element, &AtomPosition)>,
        q_bonds: &Query<(&BondElements, &BondPositions)>,
        atom_mesh: &AtomMesh,
        bond_mesh: &BondMesh,
        element_mats: &ElementMaterials,
    ) {
        for &child in children.iter() {
            if let Ok((elem, pos)) = q_atoms.get(child) {
                self.spawn_atom(commands, parent, elem, pos, atom_mesh, element_mats)
            }
            if let Ok((elem, pos)) = q_bonds.get(child) {
                self.spawn_bond(commands, parent, elem, pos, bond_mesh, element_mats)
            }
        }
    }

    fn spawn_atom(
        &self,
        commands: &mut Commands,
        parent: Entity,
        elem: &Element,
        pos: &AtomPosition,
        atom_mesh: &AtomMesh,
        element_mats: &ElementMaterials,
    );

    fn spawn_bond(
        &self,
        commands: &mut Commands,
        parent: Entity,
        elem: &BondElements,
        pos: &BondPositions,
        bond_mesh: &BondMesh,
        element_mats: &ElementMaterials,
    );
}

pub(crate) trait RepresentationList: Component + std::fmt::Debug {
    type Rep: Representation;
    fn get_set(&self) -> &HashSet<Self::Rep>;
    fn get_set_mut(&mut self) -> &mut HashSet<Self::Rep>;

    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, Self::Rep, RandomState> {
        self.get_set().difference(other.get_set())
    }

    fn insert(&mut self, value: Self::Rep) -> bool {
        self.get_set_mut().insert(value)
    }

    fn contains(&self, value: &Self::Rep) -> bool {
        self.get_set().contains(value)
    }
}

fn rep_system<RL>(
    mut commands: Commands,
    q_parent: Query<(Entity, &Children, &RL), Changed<RL>>,
    q_atoms: Query<(&Element, &AtomPosition)>,
    q_bonds: Query<(&BondElements, &BondPositions)>,
    q_reps: Query<(Entity, &RL::Rep)>,
    atom_mesh: Res<AtomMesh>,
    bond_mesh: Res<BondMesh>,
    element_mats: Res<ElementMaterials>,
) where
    RL: RepresentationList,
{
    for (parent, children, reps) in q_parent.iter() {
        //TODO: Remove/amortize this allocation
        let mut existing_reps = HashSet::new();
        for &child in children.iter() {
            // Remove reps missing from parent and keep track of the reps that exist
            if let Ok(rep) = q_reps.get_component(child) {
                if reps.contains(rep) {
                    existing_reps.insert(rep.clone());
                } else {
                    commands.entity(child).despawn_recursive();
                }
            }
        }
        // Add reps missing from children
        for rep in reps.get_set().difference(&existing_reps) {
            rep.spawn(
                &mut commands,
                parent,
                &children,
                &q_atoms,
                &q_bonds,
                atom_mesh.as_ref(),
                bond_mesh.as_ref(),
                element_mats.as_ref(),
            );
        }
    }
}

pub(crate) struct RepresentationPlugin;

impl Plugin for RepresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rep_system::<SpaceFillList>)
            .init_resource::<AtomMesh>()
            .init_resource::<BondMesh>();
    }
}

pub(crate) struct AtomMesh(Handle<Mesh>);

impl FromWorld for AtomMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self(meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 3,
        })))
    }
}

pub(crate) struct BondMesh(Handle<Mesh>);

impl FromWorld for BondMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self(meshes.add(Mesh::from(shape::Capsule {
            ..Default::default()
        })))
    }
}
