use crate::chemicals::{AtomPosition, BondIndices, Element};
use crate::error_handler;
use crate::prelude::*;
use crate::ElementMaterials;
use std::collections::HashSet;
use std::hash::Hash;

macro_rules! representations {
    ($default_mod:ident, $default_struc:ident; $($mod:ident, $struc:ident);*;) => {
        pub mod $default_mod;
        pub use $default_mod::$default_struc;
        $(
            pub mod $mod;
            pub use $mod::$struc;
        )*

        pub type DefaultRepresentation = $default_struc;

        pub type BorrowAllRepListsTuple<'a> = (
            &'a RepresentationList<$default_struc>,
            $(&'a RepresentationList<$struc>),*
        );

        pub type BorrowAllRepListsTupleWithEntity<'a> = (
            Entity,
            &'a RepresentationList<$default_struc>,
            $(&'a RepresentationList<$struc>),*
        );

        pub struct BorrowAllRepLists<'a>(
            &'a RepresentationList<$default_struc>,
            $(&'a RepresentationList<$struc>),*
        );

        impl<'a> BorrowAllRepLists<'a> {
            pub fn for_each_iter<F>(&self, mut f: F)  where F: FnMut(&mut dyn Iterator<Item=&dyn std::fmt::Debug>) -> (){
                let Self($default_mod, $($mod),*) = self;
                f(&mut $default_mod.iter().map(|v| {
                    let v: &dyn std::fmt::Debug = &*v;
                    v
                }));
                $(f(&mut $mod.iter().map(|v| {
                    let v: &dyn std::fmt::Debug = &*v;
                    v
                })));*
            }
        }

        impl<'a> From<BorrowAllRepListsTuple<'a>> for BorrowAllRepLists<'a> {
            fn from(tuple: BorrowAllRepListsTuple<'a>) -> Self {
                let ($default_mod, $($mod),*) = tuple;
                Self($default_mod, $($mod),*)
            }
        }

        impl<'a> From<BorrowAllRepListsTupleWithEntity<'a>> for BorrowAllRepLists<'a> {
            fn from(tuple: BorrowAllRepListsTupleWithEntity<'a>) -> Self {
                let (_, $default_mod, $($mod),*) = tuple;
                Self($default_mod, $($mod),*)
            }
        }

        impl Default for RepresentationList<DefaultRepresentation> {
            fn default() -> Self {
                Self(HashSet::from([DefaultRepresentation::default()]))
            }
        }

        $(impl Default for RepresentationList<$struc> {
            fn default() -> Self {
                Self(HashSet::default())
            }
        })*

        /// Bundle for entities that can be represented
        ///
        /// Contains `RepresentationList`s for all representations
        #[derive(Bundle, Default)]
        pub struct RepresentableBundle {
            $default_mod: RepresentationList<$default_struc>,
            $($mod: RepresentationList<$struc>),*
        }

        pub struct RepresentationPlugin;

        impl Plugin for RepresentationPlugin {
            fn build(&self, app: &mut App) {
                app
                    .init_resource::<AtomMesh>()
                    .add_system(rep_system::<$default_struc>.chain(error_handler).label("representations"))
                    $(.add_system(rep_system::<$struc>.chain(error_handler).label("representations")))*;
            }
        }
    };
}

representations! {
    ball_and_stick, BallAndStick;
    licorice, Licorice;
    spacefill, SpaceFill;
}

pub trait Representation: Component + Eq + Hash + Clone + std::fmt::Debug {
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
        _children: &Children,
        _q_atoms: &Query<(&Element, &AtomPosition)>,
        _q_bonds: &Query<&BondIndices>,
        _atom_mesh: &AtomMesh,
        _meshes: &mut Assets<Mesh>,
        _element_mats: &ElementMaterials,
    ) {
        ()
    }
}

#[derive(Debug, Component)]
pub struct RepresentationList<R>(HashSet<R>)
where
    R: Representation + Default;

impl<R: Representation + Default> RepresentationList<R> {
    pub fn insert(&mut self, value: R) -> bool {
        self.0.insert(value)
    }

    pub fn contains(&self, value: &R) -> bool {
        self.0.contains(value)
    }

    pub fn iter(&self) -> <&HashSet<R> as IntoIterator>::IntoIter {
        (&self).into_iter()
    }
}

impl<R: Representation + Default> IntoIterator for RepresentationList<R> {
    type Item = R;

    type IntoIter = <HashSet<R> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, R: Representation + Default> IntoIterator for &'a RepresentationList<R> {
    type Item = &'a R;

    type IntoIter = <&'a HashSet<R> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

fn rep_system<R>(
    mut commands: Commands,
    q_parent: Query<(Entity, &Children, &RepresentationList<R>), Changed<RepresentationList<R>>>,
    q_atoms: Query<(&Element, &AtomPosition)>,
    q_bonds: Query<&BondIndices>,
    q_reps: Query<(Entity, &R)>,
    atom_mesh: Res<AtomMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    element_mats: Res<ElementMaterials>,
    mut events: EventWriter<crate::camera::CamControlEvent>,
) -> Result<()>
where
    R: Representation + Default,
{
    for (parent, children, reps) in q_parent.iter() {
        // wprintln!(
        //     "Entity {parent:?} has changed {} representations: {reps:?}",
        //     std::any::type_name::<R>()
        // );
        //TODO: Remove/amortize this allocation
        let mut existing_reps = HashSet::new();
        for &child in children.iter() {
            // Remove reps missing from parent and keep track of the reps that exist
            if let Ok(rep) = q_reps.get_component(child) {
                if reps.contains(rep) {
                    existing_reps.insert(rep.clone());
                } else {
                    // wprintln!("Deleting rep {rep:?} from {child:?}");
                    commands.entity(child).despawn_recursive();
                }
            }
        }
        // Add reps missing from children
        for rep in reps.0.difference(&existing_reps) {
            for &child in children.iter() {
                if let Ok(idcs) = q_bonds.get(child) {
                    let (elem_a, pos_a) = q_atoms.get(idcs.0)?;
                    let (elem_b, pos_b) = q_atoms.get(idcs.1)?;
                    // wprintln!("Drawing {rep:?} for bond between {elem_a} at {pos_a:?} and {elem_b} at {pos_b:?}");
                    rep.spawn_bond(
                        &mut commands,
                        parent,
                        (elem_a, elem_b),
                        (pos_a, pos_b),
                        &mut meshes,
                        &element_mats,
                    )
                }
                if let Ok((elem, pos)) = q_atoms.get(child) {
                    // wprintln!("Drawing {rep:?} for {elem} atom at {pos:?}");
                    rep.spawn_atom(&mut commands, parent, elem, pos, &atom_mesh, &element_mats)
                }
            }
            rep.spawn_others(
                &mut commands,
                parent,
                &children,
                &q_atoms,
                &q_bonds,
                atom_mesh.as_ref(),
                meshes.as_mut(),
                element_mats.as_ref(),
            );
            events.send(crate::camera::CamControlEvent::ReCenter);
        }
    }
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
