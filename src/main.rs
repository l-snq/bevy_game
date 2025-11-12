use bevy::{ecs::world::EntityFetcher, prelude::*};

#[derive(Default, Component)]
struct Player;

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
    let mut spawn_coords = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("wewuz.png")),
        Transform::from_translation(spawn_coords),
    ));
}

const MENU_BOX: Color = Color::srgb(2.0, 3.0, 4.0);

fn setup_prompt(mut commands: Commands) {
    let player_id = commands.spawn((
        Sprite::from_color(MENU_BOX, Vec2::ONE),
        Transform::from_translation(Vec3::new(1.0, 1.0, 1.0)),
        Player {},
    ));
}

// figure out how to query by making Struct Markers (such as struct PLayer)
fn query_entities(mut commands: Commands, mut query: Query<(Entity, &mut Transform)>) {
    for (e, transform) in query.iter() {
        if let Ok(entity) = commands.get_entity(e) {
            println!("entity IDs: {:?}", entity.id());
        }
    }
}

fn player_physics(keys: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&Sprite, &mut Transform)>) {
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
