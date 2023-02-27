use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod myaso; // main module of game )

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
        .add_plugin(myaso::player::PlayerPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());
}