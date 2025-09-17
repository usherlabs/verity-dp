use std::cell::RefCell;
use verity_ic::crypto::config::Config;

thread_local! {
    pub static CONFIG: RefCell<Config> = RefCell::default();
}
