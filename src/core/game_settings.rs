use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    #[serde(skip)]
    pub map: i32,
    pub player_speed: f32,
    pub crewmate_vision: f32,
    pub impostor_vision: f32,
    pub kill_cooldown: f32,
    pub common_tasks: i32,
    pub long_tasks: i32,
    pub short_tasks: i32,
    pub emergency_meeting: i32,
    pub emergency_cooldown: i32,
    #[serde(skip)]
    pub impostors: i32,
    pub kill_distance: i32,
    pub discussion_time: i32,
    pub voting_time: i32,
    pub confirm_eject: bool,
    pub visual_tasks: bool,
    pub anonymous_voting: bool,
    pub task_bar_updates: i32,
}
