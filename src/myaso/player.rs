use bevy::prelude::*;

const LOCAL_ID: u32 = 0;
const LOCAL_NAME: &str = "player_local";
const PLAYER_SPEED: f32 = 100.0;
const SMOOTH_CAMERA_SPEED: f32 = 0.1;
// component to indicate Player with ID
#[derive(Component)]
pub struct Player
{
    // ID of current player
    pub id: u32,
    // Name of current player
    pub name: String,
}

// main player plugin struct
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) {
        app
            // spawning player
            .add_startup_system(spawn_player)
            .add_system(player_move)
            .add_system(player_camera_follow);
    }
}

// Bundle to create PLAYER
// myaso bratsk blya player))
#[derive(Bundle)]
struct PlayerBundle
{
    info: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            info: Player { id: LOCAL_ID, name: LOCAL_NAME.to_string() }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    // constucting own player in single -- always id == 0
    commands.spawn(
    // some player info
    (PlayerBundle {
            ..Default::default()
        },
    // drawing
    SpriteBundle {
        texture: asset_server.load("bro.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    }
    ));
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
)
{
    let mut transform = query.single_mut();
    let mut vector: Vec3 = Vec3::default();

    let sprint_mult = if keys.pressed(KeyCode::LShift) {2.0} else {1.0};
    let mult = sprint_mult * time.delta_seconds();


    if keys.pressed(KeyCode::W) {
        vector = vector + Vec3{y: PLAYER_SPEED * mult, ..Default::default()};
    }
    if keys.pressed(KeyCode::A) {
        vector = vector + Vec3{x: -PLAYER_SPEED * mult, ..Default::default()};
    }
    if keys.pressed(KeyCode::S) {
        vector = vector + Vec3{y: -PLAYER_SPEED * mult, ..Default::default()};
    }
    if keys.pressed(KeyCode::D) {
        vector = vector + Vec3{x: PLAYER_SPEED * mult, ..Default::default()};
    }
    
    transform.translation = transform.translation + vector;
}

fn player_camera_follow(
    mut set: ParamSet<(Query<&mut Transform, With<Player>>, Query<&mut Transform, With<Camera>>)>
)
{
    let mut p_query: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    for player in set.p0().iter()
    {
        p_query = player.translation;
    }
    for mut cam_pos in set.p1().iter_mut()
    {
        cam_pos.translation = cam_pos.translation.lerp(p_query, SMOOTH_CAMERA_SPEED);
    }
    
}