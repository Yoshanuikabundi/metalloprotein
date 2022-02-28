use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
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
    pub flipped: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            flipped: false,
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
    CheckUpsideDown,
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
    if keyboard.pressed(recenter_button) {
        events.send(CamControlEvent::ReCenter);
    }
    if mouse_buttons.just_released(orbit_button) || mouse_buttons.just_pressed(orbit_button) {
        events.send(CamControlEvent::CheckUpsideDown);
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
    mut cameras: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    mut events: EventReader<CamControlEvent>,
    changed_meshes: Query<&GlobalTransform, (With<Handle<Mesh>>, Changed<GlobalTransform>)>,
) {
    for (mut pan_orbit, mut transform, proj) in cameras.iter_mut() {
        let mut any = false;
        for event in events.iter() {
            any = true;
            match *event {
                CamControlEvent::Orbit(delta) => {
                    let delta_x = if pan_orbit.flipped { -delta.x } else { delta.x };
                    let yaw = Quat::from_rotation_y(-delta_x);
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
                    let xyz: Vec3 = changed_meshes
                        .iter()
                        .map(|transform| &transform.translation)
                        .sum();
                    let n = changed_meshes.iter().count() as f32;

                    let radius = changed_meshes
                        .iter()
                        .map(|transform| transform.translation.length())
                        .max_by(f32_cmp)
                        .unwrap();

                    pan_orbit.focus = xyz / n;
                    pan_orbit.radius = f32::max(radius, 0.05);
                }
                CamControlEvent::CheckUpsideDown => {
                    // only check for upside down when orbiting started or ended this frame
                    // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
                    let up = transform.rotation * Vec3::Y;
                    pan_orbit.flipped = up.y <= 0.0;
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
