use log::Level;

#[allow(dead_code)]
pub fn log_message(level: Level, message: &str) {
    match level {
        Level::Info => log::info!("{}", message),
        Level::Error => log::error!("{}", message),
        Level::Warn => log::warn!("{}", message),
        _ => log::debug!("{}", message),
    }
}
