#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

mod camera;
use camera::{MetalloproteinCameraPlugin, PickableBundle};

mod elements;
use elements::ElementMaterials;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(MetalloproteinCameraPlugin)
        .insert_resource(Args::parse())
        .init_resource::<AtomMesh>()
        .init_resource::<ElementMaterials>()
        .add_startup_system(setup)
        .add_startup_system(read_file.chain(error_handler))
        .run();
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        panic!("{}", err);
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to chemical structure file
    #[clap(short, long)]
    structure: Option<PathBuf>,
}

fn read_file(
    mut commands: Commands,
    args: Res<Args>,
    atom_mesh: Res<AtomMesh>,
    elem_materials: Res<ElementMaterials>,
) -> Result<()> {
    let args = args.as_ref();
    if let Some(path) = &args.structure {
        let mut trajectory = chemfiles::Trajectory::open(path, 'r')?;
        let mut frame = chemfiles::Frame::new();

        trajectory.read(&mut frame)?;

        for (i, &[x, y, z]) in frame.positions().iter().enumerate() {
            let atom = frame.atom(i);
            let r = if atom.vdw_radius() == 0.0 {
                1.0
            } else {
                atom.vdw_radius()
            };
            let n = atom.atomic_number();

            commands
                .spawn_bundle(PbrBundle {
                    mesh: atom_mesh.0.clone(),
                    material: elem_materials.0[n as usize].clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32)
                        .with_scale(Vec3::splat(r as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default());
        }
    };
    Ok(())
}

struct AtomMesh(Handle<Mesh>);

impl FromWorld for AtomMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self(meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 3,
        })))
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut atom_mesh: ResMut<AtomMesh>) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
