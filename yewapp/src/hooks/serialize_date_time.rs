pub fn serialize_dt<S>(
    dt: &Option<chrono::DateTime<chrono::Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(dt) => chrono::serde::ts_seconds::serialize(dt, serializer),
        None => serializer.serialize_none(),
        _ => unreachable!(),
    }
}
