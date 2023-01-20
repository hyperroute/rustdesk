pub const APP_NAME: &'static str = "nebula_desk";

pub fn args() -> Vec<String> {
    std::env::args().filter(|arg| !arg.eq(APP_NAME)).collect()
}
