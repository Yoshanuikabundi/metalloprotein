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
//! or its atoms are updated. Changes to a representation are detected by giving
//! the representation entity a copy of itself as a `PreviousRep` component. When
//! the ECS detects a change to the representation, the representation systems
//! check that the representation has actually changed before replacing spawning
//! in new meshes.

use bevy_egui::egui::Ui;

use crate::camera::CamControlEvent;
use crate::chemicals::{AtomPosition, BondIndices, Element};
use crate::error_handler;
use crate::prelude::*;
use crate::ElementMaterials;

macro_rules! representations {
    (@default $default_mod:ident, $default_struc:ident; $($mod:ident, $struc:ident);*;) => {
        /// The representation created when a new structure is spawned
        pub type DefaultRepresentation = $default_struc;

        impl Default for RepresentationEnum {
            fn default() -> Self {
                Self::from($default_struc::default())
            }
        }

        representations! {$default_mod, $default_struc; $($mod, $struc);*;}
    };

    ($($mod:ident, $struc:ident);+;) => {
        $(
            pub mod $mod;
            pub use $mod::$struc;
        )+

        /// Helper type for creating queries for entities that have any representation
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        pub type QueryAnyRep<'w, 's, 'c> = Query<'w, 's, (Entity, $(Option<&'c $struc>),+), Or<($(With<$struc>),+)>>;
        pub type QueryAnyRepMut<'w, 's, 'c> = Query<'w, 's, (Entity, $(Option<&'c mut $struc>),+), Or<($(With<$struc>),+)>>;

        /// Convenience type for working with multiple representations in one system
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        #[derive(Debug, Clone)]
        pub enum RepresentationEnum {
            $($struc($struc)),+,
        }

        impl RepresentationEnum {
            pub fn representation(&self) -> &dyn Representation {
                match self {
                    $(RepresentationEnum::$struc(inner) => inner),+
                }
            }

            pub fn representation_mut(&mut self) -> &mut dyn Representation {
                match self {
                    $(RepresentationEnum::$struc(inner) => inner),+
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(RepresentationEnum::$struc(_) => $struc::name()),+,
                }
            }
        }

        $(
            impl From<$struc> for RepresentationEnum {
                fn from(rep: $struc) -> Self {
                    RepresentationEnum::$struc(rep)
                }
            }
        )+

        impl PartialEq for RepresentationEnum {
            fn eq(&self, other: &RepresentationEnum) -> bool {
                match (self, other) {
                    $((RepresentationEnum::$struc(a), RepresentationEnum::$struc(b)) => a == b),+,
                    _ => false
                }
            }
        }


        /// Convenience type for working with multiple representations in one system
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        #[derive(Debug)]
        pub enum RepresentationEnumRef<'a> {
            $($struc(&'a $struc)),+,
        }

        impl<'a> RepresentationEnumRef<'a> {
            pub fn representation(&self) -> &dyn Representation {
                match self {
                    $(&RepresentationEnumRef::$struc(inner) => inner),+
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(RepresentationEnumRef::$struc(_) => $struc::name()),+,
                }
            }
        }

        $(
            impl<'a> From<&'a $struc> for RepresentationEnumRef<'a> {
                fn from(rep: &'a $struc) -> Self {
                    RepresentationEnumRef::$struc(rep)
                }
            }
        )+

        pub struct RepIter<'a> {
            leftovers: ($(Option<&'a $struc>),+),
            entity: Entity,
        }

        impl<'a> From<(Entity, $(Option<&'a $struc>),+)> for RepIter<'a> {
            fn from(tuple: (Entity, $(Option<&'a $struc>),+)) -> Self {
                let (entity, $($mod),+) = tuple;
                Self {
                    leftovers: ($($mod),+),
                    entity,
                }
            }
        }

        impl<'a> Iterator for RepIter<'a> {
            type Item = (Entity, RepresentationEnumRef<'a>);

            fn next(&mut self) -> Option<Self::Item> {
                let ($(ref mut $mod),+) = &mut self.leftovers;

                $(
                    if let Some($mod) = $mod.take() {
                        return Some((self.entity, $mod.into()));
                    }
                )+

                None
            }
        }

        /// Convenience type for working with multiple mutable representations in one system
        ///
        /// Creating systems that are generic over representations is preferred,
        /// but sometimes this is imposssible.
        #[derive(Debug)]
        pub enum RepresentationEnumMut<'a> {
            $($struc(Mut<'a, $struc>)),+,
        }

        impl<'a> RepresentationEnumMut<'a> {
            pub fn representation(&mut self) -> &mut dyn Representation {
                match self {
                    $(RepresentationEnumMut::$struc(ref mut inner) => &mut **inner),+
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(RepresentationEnumMut::$struc(_) => $struc::name()),+,
                }
            }
        }

        $(
            impl<'a> From<Mut<'a, $struc>> for RepresentationEnumMut<'a> {
                fn from(rep: Mut<'a, $struc>) -> Self {
                    RepresentationEnumMut::$struc(rep)
                }
            }
        )+

        pub struct RepIterMut<'a> {
            leftovers: ($(Option<Mut<'a, $struc>>),+),
            entity: Entity,
        }

        impl<'a> From<(Entity, $(Option<Mut<'a, $struc>>),+)> for RepIterMut<'a> {
            fn from(tuple: (Entity, $(Option<Mut<'a, $struc>>),+)) -> Self {
                let (entity, $($mod),+) = tuple;
                Self {
                    leftovers: ($($mod),+),
                    entity,
                }
            }
        }

        impl<'a> Iterator for RepIterMut<'a> {
            type Item = (Entity, RepresentationEnumMut<'a>);

            fn next(&mut self) -> Option<Self::Item> {
                let ($(ref mut $mod),+) = &mut self.leftovers;

                $(
                    if let Some($mod) = $mod.take() {
                        return Some((self.entity, $mod.into()));
                    }
                )+

                None
            }
        }

        pub const ALL_REPRESENTATIONS: [RepresentationEnum; 3] = [$(
            RepresentationEnum::$struc($struc::new())
        ),+];

        /// Plugin for chemical representations
        pub struct RepresentationPlugin;

        impl Plugin for RepresentationPlugin {
            fn build(&self, app: &mut App) {
                app
                    .init_resource::<AtomMesh>()
                    $(.add_system(new_reps::<$struc>.chain(spawn_reps).chain(error_handler).label("representations")))+
                    $(.add_system(update_reps::<$struc>.chain(spawn_reps).chain(error_handler).label("representations")))+
                    ;
            }
        }
    };
}

representations! {
    @default ball_and_stick, BallAndStick;
    licorice, Licorice;
    spacefill, SpaceFill;
}

#[derive(Debug, Bundle, Default)]
pub struct RepresentationBundle<R: Representation + std::fmt::Debug + Default + Component> {
    rep: R,
    transform: Transform,
    global_transform: GlobalTransform,
    computed_visibility: ComputedVisibility,
    visibility: Visibility,
}

pub type DefaultRepresentationBundle = RepresentationBundle<DefaultRepresentation>;

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

/// Add this to a RepresentationBundle to recenter the view once the representation is done loading
#[derive(Component)]
pub struct RecenterWhenDone;

pub trait Representation: std::fmt::Debug {
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

    /// egui interface for updating or creating a rep
    fn ui(&mut self, _ui: &mut Ui) {}

    ///
    fn into_bundle(self) -> RepresentationBundle<Self>
    where
        Self: Sized + Default + Component + std::fmt::Debug,
    {
        RepresentationBundle {
            rep: self,
            ..Default::default()
        }
    }

    /// Human-readable name of the representation
    fn name() -> &'static str
    where
        Self: Sized;
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct PreviousRep<R: Representation + std::fmt::Debug + Component + Clone + Eq> {
    rep: R,
}

impl<R> From<R> for PreviousRep<R>
where
    R: Representation + std::fmt::Debug + Component + Clone + Eq,
{
    fn from(rep: R) -> Self {
        Self { rep }
    }
}

impl<R> PartialEq<R> for PreviousRep<R>
where
    R: Representation + std::fmt::Debug + Component + Clone + Eq,
{
    fn eq(&self, other: &R) -> bool {
        self.rep == *other
    }
}

fn new_reps<'w, 's, 'a, 'b, R>(
    q_rep: Query<'w, 's, (Entity, &'a Parent, &'b R), Added<R>>,
) -> Vec<(Entity, Entity, R)>
where
    R: Representation + Component + Clone,
{
    q_rep
        .iter()
        .map(|(entity, parent, rep)| (entity, parent.0, rep.clone()))
        .collect()
}

fn spawn_reps<R>(
    In(reps): In<Vec<(Entity, Entity, R)>>,
    mut commands: Commands,
    q_parent: Query<&Children, With<Representable>>,
    q_atoms: Query<(&Element, &AtomPosition)>,
    q_bonds: Query<&BondIndices>,
    mut meshes: ResMut<Assets<Mesh>>,
    element_mats: Res<ElementMaterials>,
    atom_mesh: Res<AtomMesh>,
    q_recenter: Query<&RecenterWhenDone>,
    mut events: EventWriter<CamControlEvent>,
) -> Result<()>
where
    R: Representation + Component + std::fmt::Debug + Eq + Clone,
{
    for (rep_entity, parent, rep) in reps {
        commands
            .entity(rep_entity)
            .insert(PreviousRep::from(rep.clone()));
        wprintln!("Spawning new bond meshes for {rep:?} in {parent:?}");
        // Bonds, atoms are stored in the rep's siblings
        if let Ok(siblings) = q_parent.get(parent) {
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
        };
        if q_recenter.get(rep_entity).is_ok() {
            wprintln!("Recentering after spawning rep with RecenterWhenDone");
            events.send(CamControlEvent::ReCenter);
            commands.entity(rep_entity).remove::<RecenterWhenDone>();
        }
    }
    Ok(())
}

fn update_reps<R>(
    mut commands: Commands,
    mut q_rep: Query<(Entity, &Parent, &R, Option<&mut PreviousRep<R>>), Changed<R>>,
) -> Vec<(Entity, Entity, R)>
where
    R: Representation + Component + Clone + Eq,
{
    q_rep
        .iter_mut()
        .filter(|(_, _, rep, prev)| {
            if let Some(prev) = prev {
                prev.as_ref() != *rep
            } else {
                false
            }
        })
        .map(|(rep_entity, parent, rep, _)| {
            println!("Despawning children of {rep:?} entity {rep_entity:?}");
            commands.entity(rep_entity).despawn_descendants();
            commands.entity(rep_entity).remove::<PreviousRep<R>>();
            (rep_entity, parent.0, rep.clone())
        })
        .collect()
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
