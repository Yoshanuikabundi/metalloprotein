use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController},
    LookAngles, LookTransform, LookTransformPlugin,
};
use std::cmp::Ordering;

#[derive(Default)]
pub struct MetalloproteinCameraPlugin;

impl Plugin for MetalloproteinCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin)
            .add_startup_system(setup_camera)
            .insert_resource(GeometryBounds::default())
            .add_event::<ControlEvent>()
            .add_system(control_system)
            .add_system(camera_input_map)
            .add_system(update_cog);
    }
}

static EYEOFFSET: [f32; 3] = [-2.0, 5.0, 5.0];

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::from(EYEOFFSET),
        Vec3::new(0., 0., 0.),
    ));
}

#[derive(Default, Debug, Component)]
struct GeometryBounds(Vec3, f32);

fn update_cog(
    mut cog: ResMut<GeometryBounds>,
    query: Query<&GlobalTransform, (With<Handle<Mesh>>, Changed<GlobalTransform>)>,
) {
    if !query.is_empty() {
        let xyz: Vec3 = query.iter().map(|transform| &transform.translation).sum();
        let n = query.iter().count() as f32;
        cog.0 = xyz / n;

        cog.1 = query
            .iter()
            .map(|transform| transform.translation.length())
            .max_by(|a, b| {
                a.partial_cmp(b).unwrap_or(if a.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                })
            })
            .unwrap();

        println!("{cog:?}")
    }
}

pub enum ControlEvent {
    Orbit(Vec2),
    TranslateTarget(Vec2),
    Zoom(f32),
    ReCenter,
}

fn camera_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    controllers: Query<&OrbitCameraController>,
) {
    // Can only control one camera at a time.
    let controller = controllers.single();

    let OrbitCameraController {
        enabled,
        mouse_rotate_sensitivity,
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    if !enabled {
        return;
    }

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::TranslateTarget(
            mouse_translate_sensitivity * cursor_delta,
        ));
    }

    if keyboard.pressed(KeyCode::Equals) {
        events.send(ControlEvent::ReCenter);
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.iter() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
    }
    events.send(ControlEvent::Zoom(scalar));
}

fn control_system(
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&OrbitCameraController, &mut LookTransform, &Transform)>,
    cog: Res<GeometryBounds>,
) {
    // Can only control one camera at a time.
    let (controller, mut transform, scene_transform) = cameras.single_mut();

    if controller.enabled {
        let mut look_angles = LookAngles::from_vector(-transform.look_direction().unwrap());
        let mut radius_scalar = 1.0;

        for event in events.iter() {
            match event {
                ControlEvent::Orbit(delta) => {
                    look_angles.add_yaw(-delta.x);
                    look_angles.add_pitch(delta.y);
                    println!(
                        "{delta:?}, {look_angles:?}, {:?}",
                        transform.look_direction()
                    );
                }
                ControlEvent::TranslateTarget(delta) => {
                    let right_dir = scene_transform.rotation * -Vec3::X;
                    let up_dir = scene_transform.rotation * Vec3::Y;
                    transform.target += delta.x * right_dir + delta.y * up_dir;
                }
                ControlEvent::Zoom(scalar) => {
                    radius_scalar *= scalar;
                }
                ControlEvent::ReCenter => {
                    transform.target = cog.0;
                    transform.eye = cog.0 + (Vec3::from(EYEOFFSET).normalize_or_zero() * cog.1);
                }
            }
        }

        look_angles.assert_not_looking_up();

        let new_radius = (radius_scalar * transform.radius())
            .min(1000000.0)
            .max(0.001);
        transform.eye = transform.target + new_radius * look_angles.unit_vector();
    } else {
        events.iter(); // Drop the events.
    }
}
