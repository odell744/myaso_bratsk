use bevy::prelude::*;

const LOCAL_ID: u32 = 0;
const LOCAL_NAME: &str = "player_local";

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
            .add_system(player_move);
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

fn spawn_player(mut commands: Commands)
{
    // constucting own player in single -- always id == 0
    commands.spawn(
        PlayerBundle {
            ..Default::default()
        }
    );
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>
)
{

}