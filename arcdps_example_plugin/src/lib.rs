//! Example plugin showcasing ArcDPS and Unofficial Extras callbacks.

#![allow(unused_variables)]

use arcdps::{
    extras::{
        message::Message, user::UserInfoIter, Control, ExtrasAddonInfo, KeybindChange,
        SquadMessage, UserRole,
    },
    imgui, Agent, Event, Language, StateChange,
};

arcdps::export! {
    name: "Example Plugin",
    sig: 0x12345678, // change this to a random number
    init,
    release,
    update_url,
    imgui,
    options_end,
    options_windows,
    wnd_nofilter,
    wnd_filter,
    combat,
    combat_local,
    extras_init,
    extras_squad_update,
    extras_language_changed,
    extras_keybind_changed,
    extras_squad_chat_message,
    extras_chat_message,
}

/// Plugin load.
fn init() -> Result<(), Option<String>> {
    log::info!("plugin has been started");
    // for info level target "window" is the same as not specifying target
    log:: info!(target: "window", "only window logging");
    log:: info!(target: "file", "only file logging");
    log:: info!(target: "both", "logging to file and window");
    Ok(())
}

/// Plugin unload.
fn release() {
    log::info!("plugin has stopped")
}

/// Plugin update URL.
fn update_url() -> Option<String> {
    None
}

// Standalone UI creation.
fn imgui(ui: &imgui::Ui, not_character_select_or_loading: bool) {
    imgui::Window::new("My Window").build(ui, || {
        ui.text("Hello World");
    });
}

/// Plugin settings UI creation.
fn options_end(ui: &imgui::Ui) {
    let mut enabled = true;
    ui.checkbox("Some setting", &mut enabled);
}

/// Modify Arc's window checkboxes.
fn options_windows(ui: &imgui::Ui, window_name: Option<&str>) -> bool {
    if window_name.is_none() {
        // add our own window checkbox
        let mut enabled = true;
        ui.checkbox("My Window", &mut enabled);
    }
    false
}

/// All key events.
fn wnd_nofilter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    true
}

/// Key event filtered by Arc's modifiers.
fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    if key_down && !prev_key_down {
        log::info!("{key} pressed with arc modifiers");
    }
    true
}

/// Area combat event.
/// Comes with a delay and filtering.
fn combat(
    event: Option<&Event>,
    src: Option<&Agent>,
    dst: Option<&Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    if let (Some(event), Some(src)) = (event, src) {
        if let StateChange::EnterCombat = event.get_statechange() {
            log::info!(
                "{} ({}) has entered combat",
                src.name().unwrap_or("unknown agent"),
                src.id
            );
        }
    }
}

/// Combat event from chat combat log.
fn combat_local(
    event: Option<&Event>,
    src: Option<&Agent>,
    dst: Option<&Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
}

/// Unofficial extras load.
fn extras_init(extras_info: ExtrasAddonInfo, account_name: Option<&str>) {
    log::info!(
        "extras version {} on account {}",
        extras_info.string_version.unwrap_or("unknown"),
        account_name.unwrap_or("unknown")
    );
    if !extras_info.version().supports_squad_chat_message() {
        log::warn!("Chat message not supported")
    }
    if !extras_info.version().supports_chat_message2() {
        log::warn!("Chat message 2 not supported")
    }
}

/// Unofficial extras squad update.
fn extras_squad_update(users: UserInfoIter) {
    for user in users {
        if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
            log::info!(
                "{} can place markers",
                user.account_name().unwrap_or("unknown user")
            );
        }
    }
}

/// Unofficial extras client language change.
fn extras_language_changed(language: Language) {
    log::info!("language changed to {language:?}")
}

/// Unofficial extras client keybind change.
fn extras_keybind_changed(changed: KeybindChange) {
    if let Control::Movement_MoveForward
    | Control::Movement_MoveBackward
    | Control::Movement_StrafeLeft
    | Control::Movement_StrafeRight
    | Control::Movement_TurnLeft
    | Control::Movement_TurnRight = changed.control
    {
        log::info!("movement key changed");
    }
}

/// Unofficial extras squad chat message.
fn extras_squad_chat_message(message: &SquadMessage) {
    if message.is_broadcast() {
        log::info!("broadcast from {}", message.account_name());
    } else {
        log::info!(
            "message from {} in {:?}",
            message.account_name(),
            message.channel_type
        )
    }
}

/// Unofficial extras chat message.
fn extras_chat_message(message: Message) {
    match message {
        Message::Squad(message) => {
            if message.is_broadcast() {
                log::info!("broadcast from {}", message.account_name());
            } else {
                log::info!(
                    "message from {} in {:?}",
                    message.account_name(),
                    message.channel_type
                )
            }
        }
        Message::Npc(message) => {
            log::info!("message from NPC {}", message.character_name())
        }
    }
}
