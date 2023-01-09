use rand::{distributions::Alphanumeric, Rng};

pub fn create_hash() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
}
