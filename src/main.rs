#![doc = include_str!("../README.md")]

use bevy::app::AppExit;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

pub mod camera;
use camera::MetalloproteinCameraPlugin;

pub mod elements;
use elements::ElementMaterials;

pub mod chemicals;

pub mod gui;
use gui::MetalloproteinGuiPlugin;

pub mod representations;
use representations::{
    DefaultRepresentationBundle, RecenterWhenDone, RepresentableBundle, RepresentationPlugin,
};

pub mod prelude {
    pub use crate::wprintln;
    pub use bevy::prelude::*;
    pub use eyre::Report;

    pub type Result<T, E = Report> = std::result::Result<T, E>;
}
use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MetalloproteinCameraPlugin)
        .add_plugin(MetalloproteinGuiPlugin)
        .add_event::<LoadFile>()
        .init_resource::<ElementMaterials>()
        .add_startup_system(setup)
        .add_system(read_file.chain(error_handler))
        .add_plugin(RepresentationPlugin)
        .add_system(animate_light_direction);

    #[cfg(debug_assertions)]
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.run();
}

fn error_handler(In(result): In<Result<()>>, mut exit: EventWriter<AppExit>) {
    if let Err(err) = result {
        eprintln!("{:?}", err);
        exit.send(AppExit);
    }
}

#[macro_export]
macro_rules! wprintln {
    ($($args:expr),+) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!($($args),+).into());
        #[cfg(not(target_arch = "wasm32"))]
        println!($($args),+);
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to chemical structure file
    #[clap(short, long)]
    structure: Option<PathBuf>,
}

struct LoadFile {
    path: PathBuf,
}

fn setup(mut commands: Commands, mut ev_loadfile: EventWriter<LoadFile>) {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

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

    #[cfg(not(target_arch = "wasm32"))]
    {
        use clap::Parser;
        let args = Args::parse();
        if let Some(path) = args.structure {
            ev_loadfile.send(LoadFile { path });
        };
    }

    // wprintln!("Hello, world! This is metalloprotein.");
    #[cfg(target_arch = "wasm32")]
    {
        // wprintln!("Spawning a hydrogen molecule");
        let parent = commands.spawn_bundle(RepresentableBundle::default()).id();
        let atom_a = chemicals::spawn_atom(&mut commands, parent, 1, Vec3::new(2.0, 2.0, 0.0));
        let atom_b = chemicals::spawn_atom(&mut commands, parent, 1, Vec3::new(3.0, 2.0, 0.0));
        chemicals::spawn_bond(&mut commands, parent, atom_a, atom_b);
        // wprintln!("Spawned!");
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

#[derive(Default, Component, Debug)]
struct StructureFile(PathBuf);

impl<T> From<T> for StructureFile
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        Self(PathBuf::from(path.as_ref()))
    }
}

impl Display for StructureFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.file_name().map(std::ffi::OsStr::to_str) {
            Some(Some(s)) => write!(f, "{s}"),
            Some(None) => write!(f, "File"),
            None => write!(f, "File"),
        }
    }
}

fn read_file(mut commands: Commands, mut ev_loadfile: EventReader<LoadFile>) -> Result<()> {
    for LoadFile { path } in ev_loadfile.iter() {
        let entity = commands.spawn_bundle((StructureFile::from(path),)).id();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut trajectory = chemfiles::Trajectory::open(path, 'r')?;
            let mut frame = chemfiles::Frame::new();

            trajectory.read(&mut frame)?;

            chemicals::spawn_frame(&mut commands, &frame, entity)?;

            // wprintln!("Frame spawned!");
        }

        commands
            .entity(entity)
            .insert_bundle(RepresentableBundle::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(DefaultRepresentationBundle::default())
                    .insert(RecenterWhenDone);
            });
    }
    Ok(())
}
