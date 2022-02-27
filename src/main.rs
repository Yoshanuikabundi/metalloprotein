#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod camera;
use camera::MetalloproteinCameraPlugin;

pub mod elements;
use elements::ElementMaterials;

pub mod chemicals;
use chemicals::spawn_frame;

pub mod representations;
use representations::{Licorice, RepresentableBundle, RepresentationList, RepresentationPlugin};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MetalloproteinCameraPlugin)
        .init_resource::<ElementMaterials>()
        .add_startup_system(setup)
        .add_system(read_file.chain(error_handler).after("representations"))
        .add_plugin(RepresentationPlugin)
        .add_system(animate_light_direction)
        .add_system(report_tick.before("representations"))
        .run();
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        panic!("{}", err);
    }
}

fn report_tick(time: Res<Time>) {
    let time = time.time_since_startup();
    println!("Tick! The time is {time:?}");
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to chemical structure file
    #[clap(short, long)]
    structure: Option<PathBuf>,
}

fn setup(mut commands: Commands) {
    // light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
        ..Default::default()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });

    let args = Args::parse();
    if let Some(path) = &args.structure {
        commands
            .spawn_bundle((
                StructureFile::from(path),
                Transform::default(),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .insert_bundle(RepresentableBundle::default());
    };
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.5));
    }
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
    mut query: Query<
        (Entity, &StructureFile, &mut RepresentationList<Licorice>),
        Added<StructureFile>,
    >,
) -> Result<()> {
    for (entity, file, mut reps) in query.iter_mut() {
        let StructureFile(path) = file;
        let mut trajectory = chemfiles::Trajectory::open(path, 'r')?;
        let mut frame = chemfiles::Frame::new();

        trajectory.read(&mut frame)?;

        spawn_frame(&mut commands, &frame, entity);

        reps.insert_default();
        println!("Spawned frame from {path:?} with reps {reps:?}")
    }
    Ok(())
}
