use crate::chemicals::AtomPosition;
use crate::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use std::cmp::max_by;
use std::cmp::Ordering;
use std::f32::consts::PI;

#[derive(Default)]
pub struct MetalloproteinCameraPlugin;

impl Plugin for MetalloproteinCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_event::<CamControlEvent>()
            .add_system(camera_control)
            .add_system(camera_input_map);
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            ..Default::default()
        })
        .insert(PanOrbitCamera::default());
}

fn f32_cmp(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(b).unwrap_or(if a.is_nan() {
        Ordering::Greater
    } else {
        Ordering::Less
    })
}

#[derive(Debug)]
pub enum CamControlEvent {
    Orbit(Vec2),
    Roll(Vec2),
    Pan(Vec2),
    Zoom(f32),
    ReCenter,
}

fn camera_input_map(
    mut events: EventWriter<CamControlEvent>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    windows: Res<Windows>,
) {
    let orbit_button = MouseButton::Left;
    let roll_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;
    let recenter_button = KeyCode::Equals;

    let mouse_rotate_sensitivity = Vec2::new(PI * 2.0, PI); // Radians per window length
    let mouse_roll_sensitivity = Vec2::new(PI / 2.0, PI / 4.0); // Radians per window length
    let mouse_translate_sensitivity = Vec2::splat(1.0);
    let mouse_wheel_zoom_sensitivity = 1.0;
    let pixels_per_line = 53.0;

    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);

    let cursor_delta: Vec2 = mouse_motion.iter().map(|v| &v.delta).sum();

    if mouse_buttons.pressed(orbit_button) {
        let delta = mouse_rotate_sensitivity * cursor_delta / window;
        events.send(CamControlEvent::Orbit(delta));
    }
    if mouse_buttons.pressed(roll_button) {
        let delta = mouse_roll_sensitivity * cursor_delta / window;
        events.send(CamControlEvent::Roll(delta));
    }
    if mouse_buttons.pressed(pan_button) {
        let delta = mouse_translate_sensitivity * cursor_delta / window;
        events.send(CamControlEvent::Pan(delta));
    }
    if keyboard.just_pressed(recenter_button) {
        events.send(CamControlEvent::ReCenter);
    }

    let mut zoom = 0.0;
    for event in mouse_wheel.iter() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        zoom += scroll_amount * mouse_wheel_zoom_sensitivity;
    }
    if zoom.abs() > 0.0 {
        events.send(CamControlEvent::Zoom(zoom));
    }
}

fn camera_control(
    mut q_cameras: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    mut events: EventReader<CamControlEvent>,
    q_meshes: Query<&GlobalTransform, With<Handle<Mesh>>>,
    q_atoms: Query<&AtomPosition>,
) {
    for (mut pan_orbit, mut transform, proj) in q_cameras.iter_mut() {
        let mut any = false;
        for event in events.iter() {
            any = true;
            // wprintln!("Processing {event:?}");
            match *event {
                CamControlEvent::Orbit(delta) => {
                    let yaw = Quat::from_rotation_y(-delta.x);
                    let pitch = Quat::from_rotation_x(-delta.y);
                    transform.rotation = transform.rotation * yaw; // rotate around local y axis
                    transform.rotation = transform.rotation * pitch; // rotate around local x axis
                }
                CamControlEvent::Roll(delta) => {
                    let delta = max_by(delta.x, -delta.y, |a, b| f32_cmp(&a.abs(), &b.abs()))
                        * std::f32::consts::PI
                        * 2.0;
                    let roll = Quat::from_rotation_z(-delta);
                    transform.rotation = transform.rotation * roll; // rotate around local z axis
                }
                CamControlEvent::Pan(delta) => {
                    // make panning distance independent of resolution and FOV
                    let delta = delta * Vec2::new(proj.fov * proj.aspect_ratio, proj.fov);
                    // translate by local axes
                    let right = transform.rotation * Vec3::X * -delta.x;
                    let up = transform.rotation * Vec3::Y * delta.y;
                    // make panning proportional to distance away from focus point
                    let translation = (right + up) * pan_orbit.radius;
                    pan_orbit.focus += translation;
                }
                CamControlEvent::Zoom(scroll) => {
                    pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
                    // dont allow zoom to reach zero or you get stuck
                    pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
                }
                CamControlEvent::ReCenter => {
                    wprintln!("Got recenter command");
                    if let Some((focus, radius)) =
                        compute_new_center(|| q_meshes.iter().map(|trf| &trf.translation))
                    {
                        pan_orbit.focus = focus;
                        pan_orbit.radius = radius;
                    } else if let Some((focus, radius)) =
                        compute_new_center(|| q_atoms.iter().map(|pos| &pos.0))
                    {
                        pan_orbit.focus = focus;
                        pan_orbit.radius = radius;
                    }
                }
            }
        }

        if any {
            // Put the camera in the right place
            //
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

/// Compute the center and radius of a sphere enclosing all points
///
/// Points are given by a function that returns an
/// iterator over some points. It is a logic error
/// for the set of points to be different on subsequent
/// calls of the function, though they may be in
/// a different order.
///
/// Returns None if there are no points in the iterator.
fn compute_new_center<'a, I: Iterator<Item = &'a Vec3>>(
    points: impl Fn() -> I,
) -> Option<(Vec3, f32)> {
    let n = points().count();
    if n > 0 {
        let xyz: Vec3 = points().sum();
        let focus = xyz / n as f32;

        let radius = points()
            .map(|pos| (*pos - focus).length())
            .max_by(f32_cmp)
            .unwrap();

        Some((focus, f32::max(radius * 3.0, 0.05)))
    } else {
        None
    }
}
