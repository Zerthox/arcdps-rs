use arcdps::{
    extras::{ExtrasAddonInfo, UserInfoIter, UserRole},
    Agent, CombatEvent, StateChange,
};
use log::info;

arcdps::export! {
    name: "Example Plugin",
    sig: 0x12345678, // change this to a random number
    init,
    combat,
    extras_init,
    extras_squad_update,
}

fn init() -> Result<(), String> {
    info!("plugin has been started");
    // for info level target "window" is the same as not specifying target
    info!(target: "window", "only window logging");
    info!(target: "file", "only file logging");
    info!(target: "both", "logging to file and window");
    Ok(())
}

fn combat(
    event: Option<CombatEvent>,
    src: Option<Agent>,
    _dst: Option<Agent>,
    _skill_name: Option<&str>,
    _id: u64,
    _revision: u64,
) {
    if let (Some(event), Some(src)) = (event, src) {
        if let StateChange::EnterCombat = event.is_statechange {
            info!(
                "{} ({}) has entered combat",
                src.name.unwrap_or("unknown agent"),
                src.id
            );
        }
    }
}

fn extras_init(extras_info: ExtrasAddonInfo, account_name: Option<&str>) {
    info!(
        "extras version {} on account {}",
        extras_info.string_version.unwrap_or("unknown"),
        account_name.unwrap_or("unknown")
    );
}

fn extras_squad_update(users: UserInfoIter) {
    for user in users {
        if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
            info!(
                "{} can place markers",
                user.account_name.unwrap_or("unknown user")
            );
        }
    }
}
