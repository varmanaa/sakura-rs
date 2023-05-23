pub fn humanize(mut milliseconds: u128) -> String {
    let days = milliseconds / 86_400_000;
    milliseconds = milliseconds % 86_400_000;
    let hours = milliseconds / 3_600_000;
    milliseconds = milliseconds % 3_600_000;
    let minutes = milliseconds / 60_000;
    milliseconds = milliseconds % 60_000;
    let seconds = milliseconds / 1_000;
    milliseconds = milliseconds % 1_000;

    let parts =
        vec![(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s"), (milliseconds, "ms")];
    let duration: String = parts
        .into_iter()
        .filter_map(|(value, unit)| {
            match unit {
                _ if value > 0 => Some(format!("{value}{unit}")),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    duration
}
