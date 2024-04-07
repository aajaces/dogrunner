use bevy::{ prelude::*, window::{ WindowMode, WindowResized } };
use bevy_editor_pls::prelude::*;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Fullscreen Test".into(),
            mode: WindowMode::Fullscreen,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: Transform,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last { indices.first } else { atlas.index + 1 };
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    });
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut windows: Query<&mut Window>
) {
    let window = windows.single_mut();
    let window_width = window.width();
    let window_height = window.height();

    // DOG SPRITE & ANIMATION
    let dog_texture = asset_server.load("images/dog_idle_strip8.png");
    let dog_layout = TextureAtlasLayout::from_grid(Vec2::new(60.0, 60.0), 8, 1, None, None);
    let dog_atlas_layout = texture_atlas_layouts.add(dog_layout);
    let animation_indices = AnimationIndices { first: 1, last: 7 };

    // GROUND TILESET
    let ground_texture = asset_server.load("images/ground.png");
    let ground_layout = TextureAtlasLayout::from_grid(Vec2::new(288.0, 24.0), 1, 1, None, None);
    let ground_atlas_layout = texture_atlas_layouts.add(ground_layout);

    // SPAWN CAMERA
    commands.spawn(Camera2dBundle::default());

    // SPAWN DOG
    commands.spawn((
        SpriteSheetBundle {
            texture: dog_texture,
            atlas: TextureAtlas {
                layout: dog_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // SPAWN GROUND
    let ground_height = 24.0;
    let num_tiles = (window_width / 288.0).ceil() as i32;
    for i in 0..num_tiles {
        let x = ((i as f32) - (num_tiles as f32) / 2.0) * 288.0;
        let y = -window_height / 2.0 + ground_height / 2.0;
        commands.spawn((
            SpriteSheetBundle {
                texture: ground_texture.clone(),
                atlas: TextureAtlas {
                    layout: ground_atlas_layout.clone(),
                    index: 0,
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
        ));
    }
}

fn window_resize_system(resize_event: Res<Events<WindowResized>>) {
    for _ in resize_event.iter_current_update_events() {
        println!("Window resized!");
    }
}
