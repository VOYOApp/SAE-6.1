/// Struct containing constant definitions for the application.
pub struct AppDefines;

impl AppDefines {
    /// ENVIRONMENT
    /// The target frames per second for 2D physics calculations.
    /// 60 = fine, 30 = cheaper.
    pub const TARGET_FPS_2D_PHYSICS: f32 = 30.0;
    /// The width of the arena.
    pub const ARENA_WIDTH: f32 = 1200.0;
    /// The height of the arena.
    pub const ARENA_HEIGHT: f32 = 1000.0;
    /// The probability of an obstacle appearing in the arena.
    pub const OBSTACLE_PROBABILITY: f64 = 0.3;


    /// BOT Gameplay
    /// The available game modes.
    // BOT Gameplay
    pub const GAME_MODES: [&'static str; 1] = ["FreeForAll"];
    /// The rate of fire for bots in ticks.
    pub const BOT_RATE_OF_FIRE: i32 = 100;
    /// The penalty time for infractions in ticks.
    pub const PENALTY_TIME: i64 = 1000;
    /// The delay before a connection times out in seconds.
    pub const CONNECTION_TIMEOUT_DELAY: i32 = 10;
    /// The duration messages are displayed in ticks.
    pub const MESSAGE_DURATION: i32 = 1000;
    /// The maximum length of a message in characters.
    pub const MESSAGE_LENGTH: i32 = 40;
    /// The score limit for the game.
    pub const SCORE_LIMIT: i32 = -1;


    /// USER command keywords
    /// Command to set the user's name. Argument: string.
    pub const SET_NAME: &'static str = "NAME";
    /// Command to set the user's color. Arguments: 3 integers for RGB.
    pub const SET_COLOR: &'static str = "COL";
    /// Command to quit. No arguments.
    pub const QUIT: &'static str = "EXIT";
    /// Command to indicate the user is alive. No arguments.
    pub const ALIVE: &'static str = "LIVE";
    /// Command to send a message. Argument: string (a short message).
    pub const MESSAGE: &'static str = "MSG";

    /// Command to query the closest bot. No arguments.
    pub const QUERY_CLOSEST_BOT: &'static str = "CBOT";
    /// Command to query the closest projectile. No arguments.
    pub const QUERY_CLOSEST_PROJECTILE: &'static str = "CPROJ";
    /// Command to query a bot by name. Argument: string (name of the player).
    pub const QUERY_BY_NAME: &'static str = "NBOT";
    /// Command to query the list of names. No arguments.
    pub const QUERY_NAME_LIST: &'static str = "NLIST";
    /// Command to query the orientation. No arguments.
    pub const QUERY_ORIENTATION: &'static str = "ORIENT";
    /// Command to query messages from a user. Argument: string (name of the player).
    pub const QUERY_MESSAGES_FROM_USER: &'static str = "USRMSG";

    /// Command for an empty reply. No arguments.
    pub const EMPTY_REPLY: &'static str = "EMPTY";

    /// Separator for commands.
    pub const COMMAND_SEP: &'static str = "#";
    /// Separator for arguments.
    pub const ARGUMENT_SEP: &'static str = "=";
}
