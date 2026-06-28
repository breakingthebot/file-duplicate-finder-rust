// Emits simple structured log lines to stderr for application diagnostics.
// Connects to: src/main.rs, src/services/
// Created: 2026-06-28

/// Writes a DEBUG level structured log entry.
pub fn log_debug(event: &str, fields: &[(&str, &str)]) {
    log("DEBUG", event, fields);
}

/// Writes an INFO level structured log entry.
pub fn log_info(event: &str, fields: &[(&str, &str)]) {
    log("INFO", event, fields);
}

/// Writes a WARNING level structured log entry.
pub fn log_warning(event: &str, fields: &[(&str, &str)]) {
    log("WARNING", event, fields);
}

/// Writes an ERROR level structured log entry.
pub fn log_error(event: &str, fields: &[(&str, &str)]) {
    log("ERROR", event, fields);
}

/// Formats and prints one structured log entry to stderr.
fn log(level: &str, event: &str, fields: &[(&str, &str)]) {
    let mut message = format!("level={level} event={event}");

    for (key, value) in fields {
        message.push(' ');
        message.push_str(key);
        message.push('=');
        message.push('"');
        message.push_str(&escape_value(value));
        message.push('"');
    }

    eprintln!("{message}");
}

/// Escapes double quotes so field values remain parseable in log output.
fn escape_value(value: &str) -> String {
    value.replace('"', "\\\"")
}
