fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub struct Executable;
impl Executable {
    pub fn get_pnpm() -> String {
        if is_windows() {
            "pnpm.cmd".to_string()
        } else {
            "pnpm".to_string()
        }
    }
}