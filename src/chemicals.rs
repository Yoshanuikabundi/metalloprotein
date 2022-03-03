use crate::Result;
use bevy::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

pub(crate) fn spawn_atom(
    commands: &mut Commands,
    parent: Entity,
    atomic_number: i32,
    xyz: Vec3,
) -> Entity {
    let child = commands
        .spawn_bundle(AtomBundle {
            element: Element::new(atomic_number),
            atom_position: AtomPosition(xyz),
            ..Default::default()
        })
        .id();

    commands.entity(parent).push_children(&[child]);

    child
}

pub(crate) fn spawn_bond(
    commands: &mut Commands,
    parent: Entity,
    atom_a: Entity,
    atom_b: Entity,
) -> Entity {
    let child = commands
        .spawn_bundle(BondBundle {
            indices: BondIndices::new(atom_a, atom_b),
        })
        .id();

    commands.entity(parent).push_children(&[child]);

    child
}

#[derive(Error, Debug)]
pub(crate) enum SpawnStructureErr {
    #[error("Atom index {0} in bond is not defined")]
    UndefinedAtomInBond(i64),
}

#[cfg(not(target_family = "wasm"))]
pub(crate) fn spawn_frame(
    commands: &mut Commands,
    frame: &chemfiles::Frame,
    parent: Entity,
) -> Result<(), SpawnStructureErr> {
    let top = frame.topology();
    let mut index_map: Vec<Option<Entity>> = vec![None; top.size()];

    for (i, &xyz) in frame.positions().iter().enumerate() {
        let atom = frame.atom(i);
        let [x, y, z] = xyz;
        index_map[i] = Some(spawn_atom(
            commands,
            parent,
            atom.atomic_number() as i32,
            Vec3::new(x as f32, y as f32, z as f32),
        ));
    }

    for [i, j] in top.bonds() {
        let atom_a = index_map[i].ok_or(SpawnStructureErr::UndefinedAtomInBond(i as i64))?;
        let atom_b = index_map[j].ok_or(SpawnStructureErr::UndefinedAtomInBond(j as i64))?;
        spawn_bond(commands, parent, atom_a, atom_b);
    }

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn spawn_pdb(
    commands: &mut Commands,
    parent: Entity,
    pdb: pdbtbx::PDB,
) -> Result<(), SpawnStructureErr> {
    let mut index_map: HashMap<usize, Entity> = HashMap::with_capacity(pdb.atom_count());

    for atom in pdb.atoms() {
        let (x, y, z) = atom.pos();
        let atomic_number = atom.atomic_number().unwrap_or(0) as i32;
        let serial = atom.serial_number();

        index_map.insert(
            serial,
            spawn_atom(
                commands,
                parent,
                atomic_number,
                Vec3::new(x as f32, y as f32, z as f32),
            ),
        );
    }

    for (atom_a, atom_b, _bondtype) in pdb.bonds() {
        let (i, j) = (atom_a.serial_number(), atom_b.serial_number());
        let atom_a = index_map[&i];
        let atom_b = index_map[&j];
        spawn_bond(commands, parent, atom_a, atom_b);
    }

    Ok(())
}

#[derive(Component, Default, Debug)]
/// Entities with this component are atoms
pub(crate) struct Element {
    pub(crate) atomic_number: i32,
}

#[derive(Component, Default, Debug)]
pub(crate) struct AtomPosition(pub Vec3);

impl From<[f64; 3]> for AtomPosition {
    fn from(xyz: [f64; 3]) -> Self {
        let [x, y, z] = xyz;
        Self(Vec3::new(x as f32, y as f32, z as f32))
    }
}

impl Element {
    fn new(atomic_number: i32) -> Self {
        Self { atomic_number }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.atomic_number {
            n @ 0..=118 => crate::elements::ELEMENTNAMES[n as usize],
            _ => crate::elements::ELEMENTNAMES[0],
        };
        write!(f, "{name}",)
    }
}

#[derive(Component, Debug)]
pub(crate) struct BondIndices(pub Entity, pub Entity);

impl BondIndices {
    fn new(i: Entity, j: Entity) -> Self {
        Self(i, j)
    }
}

#[derive(Bundle)]
struct BondBundle {
    indices: BondIndices,
}

#[derive(Bundle, Default)]
struct AtomBundle {
    atom_position: AtomPosition,
    element: Element,
}
