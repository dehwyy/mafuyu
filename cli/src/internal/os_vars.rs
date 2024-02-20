fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub struct Executable;
impl Executable {
    pub fn get_pnpm() -> String {
        match is_windows() {
            true => "pnpm.cmd".to_string(),
            false => "pnpm".to_string(),
        }
    }
}