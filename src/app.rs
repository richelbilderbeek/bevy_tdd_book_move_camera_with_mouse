//use bevy::input::mouse::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
    }

    app.add_systems(Startup, (add_camera, add_player));
    app.add_systems(Update, (respond_to_mouse_button_press, respond_to_mouse_move, respond_to_mouse_wheel_turn));

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default()
    );
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                size: Vec3::new(64.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

fn respond_to_mouse_button_press(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut transform = query.single_mut();
    if input.pressed(MouseButton::Left) {
        transform.rotate_z(0.1);
    }
    if input.pressed(MouseButton::Right) {
        transform.rotate_z(-0.1);
    }
}

fn respond_to_mouse_move(
    mut query: Query<&mut Transform, With<Camera>>,
    mut mouse_motion_event: EventReader<bevy::input::mouse::MouseMotion>,
) {
    for event in mouse_motion_event.read() {
        let mut transform = query.single_mut();
        transform.translation.x += event.delta.x / 20.0;
        transform.translation.y -= event.delta.y / 20.0;
    }
}

fn respond_to_mouse_wheel_turn(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut mouse_wheel_event: EventReader<bevy::input::mouse::MouseWheel>,
) {
    for event in mouse_wheel_event.read() {
        let mut projection = query.single_mut();
        // Do something
        projection.size *= 1.0 + ((event.x + event.y) / 10.0);
    }
}

#[cfg(test)]
fn count_n_cameras(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Camera>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn get_camera_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<(&OrthographicProjection, &Camera)>();
    let (projection, _) = query.single(app.world());
    projection.size
}
#[cfg(test)]
fn get_player_size(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn get_camera_rotation(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<(&Transform, &Camera)>();
    let (transform, _) = query.single(app.world());
    transform.rotation.z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_size() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_size(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_create_app_has_a_camera() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_cameras(&mut app), 1);
    }

    #[test]
    fn test_camera_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_camera_is_at_regular_zoom() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_scale(&mut app), 1.0);
    }

    #[test]
    fn test_camera_is_not_rotated_at_start() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_responds_to_mouse_move() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));

        // Move the mouse
        app.world_mut().send_event(bevy::input::mouse::MouseMotion {
            delta: Vec2::new(100.0, 100.0),
        });

        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_camera_responds_to_left_mouse_button_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.update();

        // Position must have changed now
        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_responds_to_right_mouse_button_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_camera_rotation(&mut app), 0.0);

        // Press the left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);

        app.update();

        // Position must have changed now
        assert_ne!(get_camera_rotation(&mut app), 0.0);
    }

    #[test]
    fn test_camera_responds_to_mouse_wheel_turn() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_camera_scale(&mut app), 1.0);

        // Scroll the mouse
        app.world_mut().send_event(bevy::input::mouse::MouseWheel {
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            x: 10.0,
            y: 10.0,
            window: Entity::PLACEHOLDER,
        });
        app.update();

        // Moved now
        assert_ne!(get_camera_scale(&mut app), 1.0);
    }
}
