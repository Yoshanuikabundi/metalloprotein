use bevy::prelude::*;
use chemfiles::Frame;

fn spawn_atoms(commands: &mut Commands, frame: &Frame, parent: Entity) {
    for (i, &xyz) in frame.positions().iter().enumerate() {
        let atom = frame.atom(i);

        let child = commands
            .spawn_bundle(AtomBundle {
                element: Element::new(atom.atomic_number() as i32),
                atom_position: AtomPosition::new(xyz),
                index: AtomIndex(i),
                ..Default::default()
            })
            .id();

        commands.entity(parent).push_children(&[child]);
    }
}

fn spawn_bonds(commands: &mut Commands, frame: &Frame, parent: Entity) {
    let top = frame.topology();
    for [i, j] in top.bonds() {
        let atom_a = frame.positions()[i];
        let atom_b = frame.positions()[j];

        let element_a = top.atom(i).atomic_number();
        let element_b = top.atom(j).atomic_number();

        let child = commands
            .spawn_bundle(BondBundle {
                elements: BondElements(
                    Element::new(element_a as i32),
                    Element::new(element_b as i32),
                ),
                indices: BondIndices::new(i, j),
                positions: BondPositions::new(atom_a, atom_b),
                ..Default::default()
            })
            .id();

        commands.entity(parent).push_children(&[child]);
    }
}

pub(crate) fn spawn_frame(commands: &mut Commands, frame: &Frame, parent: Entity) {
    spawn_atoms(commands, frame, parent);
    spawn_bonds(commands, frame, parent);
}

#[derive(Component, Default, Debug)]
/// Entities with this component are atoms
pub(crate) struct Element {
    pub(crate) atomic_number: i32,
}

#[derive(Component, Default, Debug)]
pub(crate) struct AtomIndex(usize);

#[derive(Component, Default, Debug)]
pub(crate) struct AtomPosition(pub f32, pub f32, pub f32);

impl AtomPosition {
    fn new(xyz: [f64; 3]) -> Self {
        let [x, y, z] = xyz;
        Self(x as f32, y as f32, z as f32)
    }
}

impl Element {
    fn new(atomic_number: i32) -> Self {
        Self { atomic_number }
    }
}

#[derive(Component, Default, Debug)]
struct BondIndices(AtomIndex, AtomIndex);

impl BondIndices {
    fn new(i: usize, j: usize) -> Self {
        Self(AtomIndex(i), AtomIndex(j))
    }
}

#[derive(Component, Default, Debug)]
pub(crate) struct BondPositions(pub Vec3, pub Vec3);

impl BondPositions {
    fn new(a: [f64; 3], b: [f64; 3]) -> Self {
        let a = Vec3::new(a[0] as f32, a[1] as f32, a[2] as f32);
        let b = Vec3::new(b[0] as f32, b[1] as f32, b[2] as f32);
        Self(a, b)
    }
}

#[derive(Component, Default, Debug)]
pub(crate) struct BondElements(pub Element, pub Element);

#[derive(Bundle, Default)]
struct BondBundle {
    indices: BondIndices,
    positions: BondPositions,
    elements: BondElements,
}

#[derive(Bundle, Default)]
struct AtomBundle {
    atom_position: AtomPosition,
    element: Element,
    index: AtomIndex,
}
