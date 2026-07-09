use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

const PREFIX: &str = "rust_ui"; // Must NOT contain "/" or "-"

pub fn use_random_id() -> String {
    format!("_{PREFIX}_{}", generate_hash())
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn generate_hash() -> u64 {
    let mut hasher = DefaultHasher::new();
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    counter.hash(&mut hasher);
    hasher.finish()
}