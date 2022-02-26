#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};

mod camera;
use camera::MetalloproteinCameraPlugin;

mod elements;
use elements::ElementMaterials;

mod chemicals;
use chemicals::spawn_frame;

mod representations;
use representations::{spacefill_rep, SpaceFill};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(MetalloproteinCameraPlugin)
        .init_resource::<AtomMesh>()
        .init_resource::<ElementMaterials>()
        .add_startup_system(setup)
        .add_system(read_file.chain(error_handler))
        .add_system(spacefill_rep)
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

fn setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let args = Args::parse();
    if let Some(path) = &args.structure {
        commands.spawn_bundle((
            StructureFile::from(path),
            Transform::default(),
            GlobalTransform::default(),
            SpaceFill,
        ));
    };
}

#[derive(Default, Component)]
struct StructureFile(PathBuf);

impl<T> From<T> for StructureFile
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        Self(PathBuf::from(path.as_ref()))
    }
}

fn read_file(
    mut commands: Commands,
    query: Query<(Entity, &StructureFile), Added<StructureFile>>,
) -> Result<()> {
    for (entity, file) in query.iter() {
        let StructureFile(path) = file;
        let mut trajectory = chemfiles::Trajectory::open(path, 'r')?;
        let mut frame = chemfiles::Frame::new();

        trajectory.read(&mut frame)?;

        spawn_frame(&mut commands, &frame, entity);
    }
    Ok(())
}
