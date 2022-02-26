use bevy::prelude::*;
use chemfiles::Frame;

fn spawn_atoms(commands: &mut Commands, frame: &Frame, parent: Entity) {
    for (i, &[x, y, z]) in frame.positions().iter().enumerate() {
        let atom = frame.atom(i);
        let r = atom.vdw_radius();

        let child = commands
            .spawn_bundle(AtomBundle {
                element: Element::new(atom.atomic_number() as i32),
                transform: Transform::from_xyz(x as f32, y as f32, z as f32)
                    .with_scale(Vec3::splat(r as f32)),
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

        let child = commands
            .spawn_bundle(BondBundle {
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

#[derive(Component, Default)]
/// Entities with this component are atoms
pub(crate) struct Element {
    pub(crate) atomic_number: i32,
}

#[derive(Component, Default)]
struct AtomIndex(usize);

impl Element {
    fn new(atomic_number: i32) -> Self {
        Self { atomic_number }
    }
}

#[derive(Component, Default)]
struct BondIndices(AtomIndex, AtomIndex);

impl BondIndices {
    fn new(i: usize, j: usize) -> Self {
        Self(AtomIndex(i), AtomIndex(j))
    }
}

#[derive(Component, Default)]
struct BondPositions([f32; 3], [f32; 3]);

impl BondPositions {
    fn new(a: [f64; 3], b: [f64; 3]) -> Self {
        let [x_a, y_a, z_a] = a;
        let [x_b, y_b, z_b] = b;
        Self(
            [x_a as f32, y_a as f32, z_a as f32],
            [x_b as f32, y_b as f32, z_b as f32],
        )
    }
}

#[derive(Bundle, Default)]
struct BondBundle {
    indices: BondIndices,
    positions: BondPositions,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Default)]
struct AtomBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    element: Element,
    index: AtomIndex,
}
