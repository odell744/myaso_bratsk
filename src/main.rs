use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod myaso; // main module of game )

#[derive(Component)]
pub struct Player;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor { 
                title: "Myaso Bratsk".to_string(), 
                ..Default::default() 
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(startup_system)
        .add_system(camera_system)
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        texture: asset_server.load("bro.png"),
        ..Default::default()
    }, Player)); // here is PLAYER
}




fn camera_system(
    keys: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, With<Camera>)>
)
{
    let mut transform = camera_query.single_mut().0;
    let mut vector: Vec3 = Vec3::default();


    if keys.pressed(KeyCode::W) {
        vector = vector + Vec3{y: 1.0, ..Default::default()};
    }
    if keys.pressed(KeyCode::A) {
        vector = vector + Vec3{x: -1.0, ..Default::default()};
    }
    if keys.pressed(KeyCode::S) {
        vector = vector + Vec3{y: -1.0, ..Default::default()};
    }
    if keys.pressed(KeyCode::D) {
        vector = vector + Vec3{x: 1.0, ..Default::default()};
    }
    
    transform.translation = transform.translation + vector;
}

