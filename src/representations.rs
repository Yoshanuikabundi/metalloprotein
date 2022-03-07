//! Represent chemicals as 3D models
//!
//! A representation is a type that implements the [`Representation`] trait.
//! This trait prescribes methods that construct a representation's 3D model. A
//! representation is (a) a child of a structure entity with (b) a component that
//! implements Representation. Typically, this looks something like the
//! following:
//!
//!                        Structure entity
//!                               |
//!           ----------------------------------------
//!           |             |             |          |
//!     Representable    Children    Transform   Visibility
//!                         |
//!                 --------------------------------------------------...
//!                 |             |             |                |
//!           AtomPosition  AtomPosition  AtomPosition  Representation entity
//!                                                              |
//!                                    ------------------------------------
//!                                    |                |        |        |
//!                          impl Representation    Children   Transform  Visibility
//!                                                     |
//!                                              ---------------
//!                                              |       |     |
//!                                         Transform  Mesh  Visibility
//!
//! Representations are responsible for spawning in any meshes or materials that
//! they require. Entities added by representations should be children of the
//! Representation entity so that they can respond to changes in the parent's
//! Transform and Visibility components, and so that they can be cleaned up.
//!
//! The various systems in this module are responsible for keeping track of
//! which atoms and bonds are visible in each representation, and for cleaning
//! up the representation when the component is removed. They do this by
//! calling the [`Representation::spawn_bond`], [`Representation::spawn_atom`],
//! and [`Representation::spawn_others`] methods when a representation is added
//! or its atoms are updated.

use crate::chemicals::{AtomPosition, BondIndices, Element};
use crate::error_handler;
use crate::prelude::*;
use crate::ElementMaterials;

macro_rules! representations {
    (@default $default_mod:ident, $default_struc:ident; $($mod:ident, $struc:ident);*;) => {
        /// The representation created when a new structure is spawned
        pub type DefaultRepresentation = $default_struc;

        #[derive(Debug, Bundle, Default)]
        pub struct DefaultRepresentationBundle{
            transform: Transform,
            global_transform: GlobalTransform,
            computed_visibility: ComputedVisibility,
            visibility: Visibility,
            $default_mod: $default_struc,
        }

        representations! {$default_mod, $default_struc; $($mod, $struc);*;}
    };

    ($($mod:ident, $struc:ident);*;) => {
        $(
            pub mod $mod;
            pub use $mod::$struc;
        )*

        /// Helper type for creating queries for entities that have any representation
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        pub type QueryAnyRep<'w, 's, 'c> = Query<'w, 's, (Entity, $(Option<&'c $struc>),*), Or<($(With<$struc>),*)>>;

        /// Convenience type for working with multiple representations in one system
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        #[derive(Debug)]
        pub enum RepresentationEnum<'a> {
            $($struc(&'a $struc)),*,
        }

        $(
            impl<'a> From<&'a $struc> for RepresentationEnum<'a> {
                fn from(rep: &'a $struc) -> Self {
                    RepresentationEnum::$struc(rep)
                }
            }
        )*

        pub struct RepIter<'a> {
            leftovers: ($(Option<&'a $struc>),*),
            entity: Entity,
        }

        impl<'a> From<(Entity, $(Option<&'a $struc>),*)> for RepIter<'a> {
            fn from(tuple: (Entity, $(Option<&'a $struc>),*)) -> Self {
                let (entity, $($mod),*) = tuple;
                Self {
                    leftovers: ($($mod),*),
                    entity,
                }
            }
        }

        impl<'a> Iterator for RepIter<'a> {
            type Item = (Entity, RepresentationEnum<'a>);

            fn next(&mut self) -> Option<Self::Item> {
                let ($(ref mut $mod),*) = &mut self.leftovers;

                $(
                    if let Some($mod) = $mod.take() {
                        return Some((self.entity, $mod.into()));
                    }
                )*

                None
            }
        }

        /// Plugin for chemical representations
        pub struct RepresentationPlugin;

        impl Plugin for RepresentationPlugin {
            fn build(&self, app: &mut App) {
                app
                    .init_resource::<AtomMesh>()
                    $(.add_system(spawn_reps::<$struc>.chain(error_handler).label("representations")))*
                    ;
            }
        }
    };
}

representations! {
    @default licorice, Licorice;
    ball_and_stick, BallAndStick;
    spacefill, SpaceFill;
}

#[derive(Debug, Component, Default)]
pub struct Representable;

#[derive(Debug, Bundle, Default)]
pub struct RepresentableBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    computed_visibility: ComputedVisibility,
    visibility: Visibility,
    representable: Representable,
}

pub trait Representation: Component + Default {
    fn spawn_atom(
        &self,
        _commands: &mut Commands,
        _parent: Entity,
        _elem: &Element,
        _pos: &AtomPosition,
        _atom_mesh: &AtomMesh,
        _element_mats: &ElementMaterials,
    ) {
        ()
    }

    fn spawn_bond(
        &self,
        _commands: &mut Commands,
        _parent: Entity,
        _elem: (&Element, &Element),
        _pos: (&AtomPosition, &AtomPosition),
        _meshes: &mut Assets<Mesh>,
        _element_mats: &ElementMaterials,
    ) {
        ()
    }

    fn spawn_others(
        &self,
        _commands: &mut Commands,
        _parent: Entity,
        _siblings: &Children,
        _q_atoms: &Query<(&Element, &AtomPosition)>,
        _q_bonds: &Query<&BondIndices>,
        _atom_mesh: &AtomMesh,
        _meshes: &mut Assets<Mesh>,
        _element_mats: &ElementMaterials,
    ) {
        ()
    }
}

fn spawn_reps<R>(
    mut commands: Commands,
    q_rep: Query<(Entity, &Parent, &R), Added<R>>,
    q_parent: Query<&Children, With<Representable>>,
    q_atoms: Query<(&Element, &AtomPosition)>,
    q_bonds: Query<&BondIndices>,
    mut meshes: ResMut<Assets<Mesh>>,
    element_mats: Res<ElementMaterials>,
    atom_mesh: Res<AtomMesh>,
    mut ev_camera: EventWriter<crate::camera::CamControlEvent>,
) -> Result<()>
where
    R: Representation,
{
    let mut new_reps = false;
    for (rep_entity, parent, rep) in q_rep.iter() {
        if let Ok(siblings) = q_parent.get(parent.0) {
            for &sibling in siblings.iter() {
                if let Ok(idcs) = q_bonds.get(sibling) {
                    let (elem_a, pos_a) = q_atoms.get(idcs.0)?;
                    let (elem_b, pos_b) = q_atoms.get(idcs.1)?;
                    rep.spawn_bond(
                        &mut commands,
                        rep_entity,
                        (elem_a, elem_b),
                        (pos_a, pos_b),
                        &mut meshes,
                        &element_mats,
                    )
                }
                if let Ok((elem, pos)) = q_atoms.get(sibling) {
                    rep.spawn_atom(
                        &mut commands,
                        rep_entity,
                        elem,
                        pos,
                        &atom_mesh,
                        &element_mats,
                    )
                }
            }

            rep.spawn_others(
                &mut commands,
                rep_entity,
                &siblings,
                &q_atoms,
                &q_bonds,
                atom_mesh.as_ref(),
                meshes.as_mut(),
                element_mats.as_ref(),
            );
            new_reps = true;
        }
    }
    if new_reps {
        ev_camera.send(crate::camera::CamControlEvent::ReCenter);
    };
    Ok(())
}

pub struct AtomMesh(Handle<Mesh>);

impl FromWorld for AtomMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self(meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 3,
        })))
    }
}
