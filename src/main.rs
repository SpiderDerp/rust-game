use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_retrograde::prelude::*;

// Create a stage label that will be used for our game logic stage
#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

const PLAYER_SPRITESHEET: &str = "spritesheet.png";

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "SpiderDerp's Website".into(),
            ..Default::default()
        })
        .add_plugins(RetroPlugins::default())
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Load our sprites
    let texture_handle = asset_server.load(PLAYER_SPRITESHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1000.0, 1000.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn the camera with a fixed height of 80 in-game pixels and a width determined by the
    // window aspect.
    commands.spawn_bundle(RetroCameraBundle::fixed_height(80.0));

    // Spawn a character
    let _player = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(0.05)),
            ..Default::default()
        })
        // Add our player marker component so we can move it
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = 1;
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let speed: f32 = 40.0 * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x += -speed;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += speed;

    }

    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += speed;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y += -speed;
    }
}