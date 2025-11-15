use bevy::prelude::*;

#[derive(Default, Component)]
struct Player {
    position: f32,
    health: u8,
}

#[derive(Default, Component)]
struct GameState {
    prompt: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_player)
        .add_systems(Update, setup_prompt)
        .add_systems(Update, player_physics)
        .run();
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_coords = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn(Camera2d::default()
        );

    commands.spawn((
        Sprite::from_image(asset_server.load("wewuz.png")),
        Transform::default()
            .with_translation(spawn_coords)
            .with_scale(Vec3::new(1.0, 1.0, 3.0)),
        Player {
            position: 1.0,
            health: 100,
        },
    ));
}

const MENU_BOX: Color = Color::srgb(2.0, 3.0, 4.0);

fn setup_prompt(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(MENU_BOX, Vec2::ONE),
        Transform::default()
            .with_translation(Vec3::new(1.0, 1.0, 1.0))
            .with_scale(Vec3::new(40.0, 40.0, 8.0)),
    ));
}

fn player_physics(keys: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut Player)>) {

    for (mut transform, mut player) in &mut query {
            if keys.pressed(KeyCode::KeyW) {
                transform.translation.y += 1.0;
                player.position = transform.translation.y;
            } 

            if keys.pressed(KeyCode::KeyD) {
                transform.translation.x += 1.0;
                player.position = transform.translation.x;
            } 

            if keys.pressed(KeyCode::KeyA) {
                transform.translation.x -= 1.0;
                player.position = transform.translation.x;
            } 

            if keys.pressed(KeyCode::KeyS) {
                transform.translation.y -= 1.0;
                player.position = transform.translation.y;
            }
    }
}
