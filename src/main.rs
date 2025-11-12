use bevy::{picking::input::spawn_mouse_pointer, prelude::*};

#[derive(Default, Component)]
struct Player {
    coordinates: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_player)
        .add_systems(Update, player_physics)
        .run();
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut spawn_coords = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("wewuz.png")),
        Transform::from_translation(spawn_coords),
    ));
}

fn player_physics(keys: Res<ButtonInput<KeyCode>>, mut query: Query<(&Sprite, &mut Transform)>) {
    for (sprite, mut transform) in &mut query {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += 1.0;
        } else if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.0;
        } else if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.0;
        } else if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.0;
        }
    }
}
