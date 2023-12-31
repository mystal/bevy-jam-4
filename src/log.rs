use bevy::log::{Level, LogPlugin};

pub fn log_plugin() -> LogPlugin {
    // Configure logging.
    let mut plugin = LogPlugin::default();
    if cfg!(feature = "verbose_logs") {
        plugin.filter.push_str(",info,bevy_jam_4=trace");
        plugin.level = Level::TRACE;
    } else if cfg!(debug_assertions) {
        plugin.filter.push_str(",info,bevy_jam_4=debug");
        plugin.level = Level::DEBUG;
    }
    plugin
}
