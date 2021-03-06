use super::{
    aucaptureoffsets::AUCaptureOffsets, auprocess::AUProcess, game_settings::GameSettings,
    process::Process,
};

const ENABLE_UNCONTROLLABLE_PARAMS: bool = false;

struct Offsets {
    game_options_offset: u32,
    game_settings_relative_address: [u32; 2],
    map: u32,
    player_speed: u32,
    crewmate_vision: u32,
    impostor_vision: u32,
    kill_cooldown: u32,
    common_tasks: u32,
    long_tasks: u32,
    short_tasks: u32,
    emergency_meeting: u32,
    emergency_cooldown: u32,
    impostors: u32,
    kill_distance: u32,
    discussion_time: u32,
    voting_time: u32,
    confirm_eject: u32,
    visual_tasks: u32,
    anonymous_voting: u32,
    task_bar_updates: u32,
}

impl Offsets {
    pub fn new(game_options_offset: u32) -> Self {
        Self {
            game_options_offset,
            game_settings_relative_address: [0x5C, 0x04],
            map: 0x10,
            player_speed: 0x14,    // float
            crewmate_vision: 0x18, // float
            impostor_vision: 0x1c, // float
            kill_cooldown: 0x20,   // float
            common_tasks: 0x24,
            long_tasks: 0x28,
            short_tasks: 0x2c,
            emergency_meeting: 0x30,
            emergency_cooldown: 0x34,
            impostors: 0x38,
            kill_distance: 0x40,
            discussion_time: 0x44,
            voting_time: 0x48,
            confirm_eject: 0x4c,    // bool
            visual_tasks: 0x4d,     // bool
            anonymous_voting: 0x4e, // bool
            task_bar_updates: 0x50,
        }
    }
}

fn base_addr(process: &Process, offsets: &Offsets) -> u32 {
    let game_options = process.read_u32(
        process
            .base_addr_of_module_name("GameAssembly.dll")
            .unwrap()
            + offsets.game_options_offset,
    );
    // let game_options = process.read_u32(game_options_ref);
    let mut addr = game_options;
    for relative in offsets.game_settings_relative_address.iter() {
        addr = process.read_u32(addr + relative);
    }
    println!(
        "{:x} {:x} {:x} {:x} {:?}",
        process
            .base_addr_of_module_name("GameAssembly.dll")
            .unwrap(),
        offsets.game_options_offset,
        game_options,
        addr,
        offsets.game_settings_relative_address
    );
    addr
}

impl GameSettings {
    fn read(process: &Process, offsets: &Offsets) -> Self {
        let base_addr = base_addr(process, offsets);
        Self {
            map: process.read_i32(base_addr + offsets.map),
            player_speed: process.read_f32(base_addr + offsets.player_speed),
            crewmate_vision: process.read_f32(base_addr + offsets.crewmate_vision),
            impostor_vision: process.read_f32(base_addr + offsets.impostor_vision),
            kill_cooldown: process.read_f32(base_addr + offsets.kill_cooldown),
            common_tasks: process.read_i32(base_addr + offsets.common_tasks),
            long_tasks: process.read_i32(base_addr + offsets.long_tasks),
            short_tasks: process.read_i32(base_addr + offsets.short_tasks),
            emergency_meeting: process.read_i32(base_addr + offsets.emergency_meeting),
            emergency_cooldown: process.read_i32(base_addr + offsets.emergency_cooldown),
            impostors: process.read_i32(base_addr + offsets.impostors),
            kill_distance: process.read_i32(base_addr + offsets.kill_distance),
            discussion_time: process.read_i32(base_addr + offsets.discussion_time),
            voting_time: process.read_i32(base_addr + offsets.voting_time),
            confirm_eject: process.read_u8(base_addr + offsets.confirm_eject) != 0,
            visual_tasks: process.read_u8(base_addr + offsets.visual_tasks) != 0,
            anonymous_voting: process.read_u8(base_addr + offsets.anonymous_voting) != 0,
            task_bar_updates: process.read_i32(base_addr + offsets.task_bar_updates),
        }
    }

    fn write(&self, process: &Process, offsets: &Offsets) {
        let base_addr = base_addr(process, offsets);
        if ENABLE_UNCONTROLLABLE_PARAMS {
            process.write_i32(base_addr + offsets.map, self.map);
        }
        process.write_f32(base_addr + offsets.player_speed, self.player_speed);
        process.write_f32(base_addr + offsets.crewmate_vision, self.crewmate_vision);
        process.write_f32(base_addr + offsets.impostor_vision, self.impostor_vision);
        process.write_f32(base_addr + offsets.kill_cooldown, self.kill_cooldown);
        process.write_i32(base_addr + offsets.common_tasks, self.common_tasks);
        process.write_i32(base_addr + offsets.long_tasks, self.long_tasks);
        process.write_i32(base_addr + offsets.short_tasks, self.short_tasks);
        process.write_i32(
            base_addr + offsets.emergency_meeting,
            self.emergency_meeting,
        );
        process.write_i32(
            base_addr + offsets.emergency_cooldown,
            self.emergency_cooldown,
        );
        if ENABLE_UNCONTROLLABLE_PARAMS {
            process.write_i32(base_addr + offsets.impostors, self.impostors);
        }
        process.write_i32(base_addr + offsets.kill_distance, self.kill_distance);
        process.write_i32(base_addr + offsets.discussion_time, self.discussion_time);
        process.write_i32(base_addr + offsets.voting_time, self.voting_time);
        process.write_u8(base_addr + offsets.confirm_eject, self.confirm_eject as u8);
        process.write_u8(base_addr + offsets.visual_tasks, self.visual_tasks as u8);
        process.write_u8(
            base_addr + offsets.anonymous_voting,
            self.anonymous_voting as u8,
        );
        process.write_i32(base_addr + offsets.task_bar_updates, self.task_bar_updates);
    }
}

pub struct AUProcessReadWrite<'a> {
    au_process: &'a AUProcess,
    offsets: Offsets,
}

impl<'a> AUProcessReadWrite<'a> {
    pub fn new(
        au_capture_offsets: &'a AUCaptureOffsets,
        au_process: &'a AUProcess,
    ) -> Option<Self> {
        Some(Self {
            au_process,
            offsets: Offsets::new(au_capture_offsets.game_options_offset(&au_process.dll_hash())?),
        })
    }

    pub fn game_settings(&self) -> GameSettings {
        GameSettings::read(self.au_process.process(), &self.offsets)
    }

    pub fn set_game_settings(&self, value: GameSettings) {
        value.write(self.au_process.process(), &self.offsets);
    }
}
