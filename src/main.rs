#![doc = include_str!("../README.md")]

use bevy::app::AppExit;
use bevy::prelude::*;
use eyre::Report;
use std::path::{Path, PathBuf};

pub mod camera;
use camera::MetalloproteinCameraPlugin;

pub mod elements;
use elements::ElementMaterials;

pub mod chemicals;

pub mod representations;
use representations::{RepresentableBundle, RepresentationPlugin};

type Result<T, E = Report> = std::result::Result<T, E>;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MetalloproteinCameraPlugin)
        .add_event::<LoadFile>()
        .init_resource::<ElementMaterials>()
        .add_startup_system(setup)
        .add_system(read_file.chain(error_handler).before("representations"))
        .add_plugin(RepresentationPlugin)
        .add_system(animate_light_direction)
        .run();
}

fn error_handler(In(result): In<Result<()>>, mut exit: EventWriter<AppExit>) {
    if let Err(err) = result {
        eprintln!("{:?}", err);
        exit.send(AppExit);
    }
}

#[cfg(not(target_family = "wasm"))]
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to chemical structure file
    #[clap(short, long)]
    structure: Option<PathBuf>,
}

#[macro_export]
macro_rules! wprintln {
    ($($args:expr),+) => {
        #[cfg(target_family = "wasm")]
        web_sys::console::log_1(&format!($($args),+).into());
        #[cfg(not(target_family = "wasm"))]
        println!($($args),+);
    };
}

struct LoadFile {
    path: PathBuf,
}

fn setup(mut commands: Commands, mut ev_loadfile: EventWriter<LoadFile>) {
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

    #[cfg(not(target_family = "wasm"))]
    {
        use clap::Parser;
        let args = Args::parse();
        if let Some(path) = args.structure {
            ev_loadfile.send(LoadFile { path });
        };
    }

    wprintln!("Hello, world! This is metalloprotein.");
    #[cfg(target_family = "wasm")]
    {
        wprintln!("Spawning a hydrogen molecule");
        let parent = commands
            .spawn_bundle((
                Transform::default(),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .insert_bundle(RepresentableBundle::default())
            .id();
        let atom_a = chemicals::spawn_atom(&mut commands, parent, 1, Vec3::new(2.0, 2.0, 0.0));
        let atom_b = chemicals::spawn_atom(&mut commands, parent, 1, Vec3::new(3.0, 2.0, 0.0));
        chemicals::spawn_bond(&mut commands, parent, atom_a, atom_b);
        wprintln!("Spawned!");
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.5));
    }
    // wprintln!("tick");
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

fn read_file(mut commands: Commands, mut ev_loadfile: EventReader<LoadFile>) -> Result<()> {
    for LoadFile { path } in ev_loadfile.iter() {
        let entity = commands
            .spawn_bundle((
                StructureFile::from(path),
                Transform::default(),
                GlobalTransform::default(),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .id();

        #[cfg(target_family = "wasm")]
        {
            wprintln!("Opening PDB {path:?}!");

            let (pdb, _errors) = pdbtbx::open(
                path.to_str().ok_or(Report::msg("Path not valid unicode"))?,
                pdbtbx::StrictnessLevel::Loose,
            )
            .map_err(|v| v.first().cloned().unwrap())?;

            wprintln!("PDB opened!");

            chemicals::spawn_pdb(&mut commands, entity, pdb)?;

            wprintln!("PDB spawned!");
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let mut trajectory = chemfiles::Trajectory::open(path, 'r')?;
            let mut frame = chemfiles::Frame::new();

            trajectory.read(&mut frame)?;

            chemicals::spawn_frame(&mut commands, &frame, entity)?;

            wprintln!("Frame spawned!");
        }

        commands
            .entity(entity)
            .insert_bundle(RepresentableBundle::default());
    }
    Ok(())
}
