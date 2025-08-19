use std::sync::LazyLock;

pub static POOL: LazyLock<ritual::Pool> =
    LazyLock::new(|| ritual::Pool::new(ritual::PoolOptions::default()));
