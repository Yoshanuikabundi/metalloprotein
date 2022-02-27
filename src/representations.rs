use crate::chemicals::{AtomPosition, BondElements, BondPositions, Element};
use crate::ElementMaterials;
use bevy::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

macro_rules! representation {
    ($($mod:ident, $struc:ident);+) => {
        $(
            pub mod $mod;
            pub use $mod::$struc;
        )+

        /// Bundle for entities that can be represented
        ///
        /// Contains `RepresentationList`s for all representations
        #[derive(Bundle, Default)]
        pub(crate) struct RepresentableBundle {
            $($mod: RepresentationList<$struc>),+
        }

        pub(crate) struct RepresentationPlugin;

        impl Plugin for RepresentationPlugin {
            fn build(&self, app: &mut App) {
                app
                    .init_resource::<AtomMesh>()
                    $(.add_system(rep_system::<$struc>.label("representations")))+;
            }
        }
    };
}

representation! {
    spacefill, SpaceFill;
    ball_and_stick, BallAndStick;
    licorice, Licorice
}

pub(crate) trait Representation: Component + Eq + Hash + Clone + std::fmt::Debug {
    fn spawn(
        &self,
        commands: &mut Commands,
        parent: Entity,
        children: &Children,
        q_atoms: &Query<(&Element, &AtomPosition)>,
        q_bonds: &Query<(&BondElements, &BondPositions)>,
        atom_mesh: &AtomMesh,
        meshes: &mut Assets<Mesh>,
        element_mats: &ElementMaterials,
    ) {
        for &child in children.iter() {
            if let Ok((elem, pos)) = q_atoms.get(child) {
                self.spawn_atom(commands, parent, elem, pos, atom_mesh, element_mats)
            }
            if let Ok((elem, pos)) = q_bonds.get(child) {
                self.spawn_bond(commands, parent, elem, pos, meshes, element_mats)
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
        meshes: &mut Assets<Mesh>,
        element_mats: &ElementMaterials,
    );
}

#[derive(Default, Debug, Component)]
pub(crate) struct RepresentationList<R>(HashSet<R>)
where
    R: Representation + Default;

impl<R: Representation + Default> RepresentationList<R> {
    pub fn insert(&mut self, value: R) -> bool {
        self.0.insert(value)
    }

    pub fn contains(&self, value: &R) -> bool {
        self.0.contains(value)
    }

    /// Add a default representation to the list
    pub fn insert_default(&mut self) {
        self.insert(R::default());
    }
}

fn rep_system<R>(
    mut commands: Commands,
    q_parent: Query<(Entity, &Children, &RepresentationList<R>), Changed<RepresentationList<R>>>,
    q_atoms: Query<(&Element, &AtomPosition)>,
    q_bonds: Query<(&BondElements, &BondPositions)>,
    q_reps: Query<(Entity, &R)>,
    atom_mesh: Res<AtomMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    element_mats: Res<ElementMaterials>,
    mut events: EventWriter<crate::camera::ControlEvent>,
) where
    R: Representation + Default,
{
    for (parent, children, reps) in q_parent.iter() {
        println!("Updating representations of {parent:?}");
        //TODO: Remove/amortize this allocation
        let mut existing_reps = HashSet::new();
        for &child in children.iter() {
            // Remove reps missing from parent and keep track of the reps that exist
            if let Ok(rep) = q_reps.get_component(child) {
                if reps.contains(rep) {
                    existing_reps.insert(rep.clone());
                    println!("Recorded existing rep {rep:?}")
                } else {
                    commands.entity(child).despawn_recursive();
                    println!("Removed rep {rep:?}");
                }
            }
        }
        // Add reps missing from children
        for rep in reps.0.difference(&existing_reps) {
            rep.spawn(
                &mut commands,
                parent,
                &children,
                &q_atoms,
                &q_bonds,
                atom_mesh.as_ref(),
                meshes.as_mut(),
                element_mats.as_ref(),
            );
            println!("Spawned rep {rep:?}");
            // events.send(crate::camera::ControlEvent::ReCenter);
        }
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
