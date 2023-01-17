pub fn create_hash() -> String {
    gloo_console::log! {"Creating hash"};

    rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
}
