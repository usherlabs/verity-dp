use verity_dp_ic::crypto::config::Config;
use std::cell::RefCell;

thread_local! {
    pub static CONFIG: RefCell<Config> = RefCell::default();
}
