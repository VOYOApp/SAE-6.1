pub struct AppDefines;

impl AppDefines {
    // ENVIRONMENT
    // calculation per sec (or step length): 60 = fine , 30 = cheaper
    pub const TARGET_FPS_2D_PHYSICS: f32 = 30.0;
    pub const ARENA_WIDTH: f32 = 100.0;
    pub const ARENA_HEIGHT: f32 = 80.0;
    pub const OBSTACLE_PROBABILITY: f64 = 0.3;

    // BOT Gameplay
    pub const GAME_MODES: [&'static str; 1] = ["FreeForAll"];
    pub const BOT_RATE_OF_FIRE: i32 = 100; //ticks
    pub const PENALTY_TIME: i64 = 1000; //ticks
    pub const CONNECTION_TIMEOUT_DELAY: i32 = 10; // 10 seconds
    pub const MESSAGE_DURATION: i32 = 1000; //ticks
    pub const MESSAGE_LENGTH: i32 = 40; //characters
    pub const SCORE_LIMIT: i32 = -1; //points

    // USER command keywords
    pub const SET_NAME: &'static str = "NAME"; // arg: string
    // args: 3 integers for RGB
    pub const SET_COLOR: &'static str = "COL";
    pub const QUIT: &'static str = "EXIT"; // no arguments
    pub const ALIVE: &'static str = "LIVE"; // no arguments
    pub const MESSAGE: &'static str = "MSG"; // arg string : a short message

    pub const QUERY_CLOSEST_BOT: &'static str = "CBOT"; // no arguments
    pub const QUERY_CLOSEST_PROJECTILE: &'static str = "CPROJ"; // no arguments
    pub const QUERY_BY_NAME: &'static str = "NBOT"; // arg string : name of the player
    pub const QUERY_NAME_LIST: &'static str = "NLIST"; // no arguments
    pub const QUERY_ORIENTATION: &'static str = "ORIENT"; // no arguments
    pub const QUERY_MESSAGES_FROM_USER: &'static str = "USRMSG"; // arg string : name of the player

    pub const EMPTY_REPLY: &'static str = "EMPTY"; // no arguments

    pub const COMMAND_SEP: &'static str = "#";
    pub const ARGUMENT_SEP: &'static str = "=";
}
