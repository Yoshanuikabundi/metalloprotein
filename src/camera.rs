use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use std::cmp::max_by;
use std::cmp::Ordering;

#[derive(Default)]
pub struct MetalloproteinCameraPlugin;

impl Plugin for MetalloproteinCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .insert_resource(GeometryBounds::default())
            .add_event::<ControlEvent>()
            .add_system(pan_orbit_camera)
            // .add_system(camera_input_map)
            .add_system(update_cog);
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
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

#[derive(Default, Debug, Component)]
struct GeometryBounds {
    cog: Vec3,
    radius: f32,
}

fn f32_cmp(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(b).unwrap_or(if a.is_nan() {
        Ordering::Greater
    } else {
        Ordering::Less
    })
}

fn update_cog(
    mut bounds: ResMut<GeometryBounds>,
    query: Query<&GlobalTransform, (With<Handle<Mesh>>, Changed<GlobalTransform>)>,
) {
    if !query.is_empty() {
        let xyz: Vec3 = query.iter().map(|transform| &transform.translation).sum();
        let n = query.iter().count() as f32;
        bounds.cog = xyz / n;

        bounds.radius = query
            .iter()
            .map(|transform| transform.translation.length())
            .max_by(f32_cmp)
            .unwrap();
    }
}

pub enum ControlEvent {
    Orbit(Vec2),
    Roll(Vec2),
    Pan(Vec2),
    Zoom(f32),
    ReCenter,
}

fn camera_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mouse_rotate_sensitivity = Vec2::splat(0.006);
    let mouse_translate_sensitivity = Vec2::splat(0.008);
    let mouse_wheel_zoom_sensitivity = 0.15;
    let pixels_per_line = 53.0;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::Pan(
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

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    keyboard: Res<Input<KeyCode>>,
    geom_bounds: Res<GeometryBounds>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;
    let roll_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;
    let recenter_button = KeyCode::Equals;

    let mut pan = Vec2::ZERO;
    let mut orbit_move = Vec2::ZERO;
    let mut roll_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut check_upside_down = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            orbit_move += ev.delta;
        }
    }
    if input_mouse.pressed(roll_button) {
        for ev in ev_motion.iter() {
            roll_move += ev.delta;
        }
    }
    if input_mouse.pressed(pan_button) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    let recenter = keyboard.pressed(recenter_button);
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        check_upside_down = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if check_upside_down {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if orbit_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = orbit_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = orbit_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = transform.rotation * yaw; // rotate around local y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        }
        if roll_move.x.abs() > 0.0 || roll_move.y.abs() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let (delta_x, delta_y) = (roll_move.x / window.x, roll_move.y / window.y);
            let delta = max_by(delta_x, delta_y, |a, b| f32_cmp(&a.abs(), &b.abs()))
                * std::f32::consts::PI
                * 2.0;
            let roll = Quat::from_rotation_z(-delta);
            transform.rotation = transform.rotation * roll; // rotate around local z axis
        }
        if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        }
        if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }
        if recenter {
            any = true;
            pan_orbit.focus = geom_bounds.cog;
            pan_orbit.radius = f32::max(geom_bounds.radius, 0.05);
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

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
