pub enum Executable {
    Pnpm,
}

impl std::fmt::Display for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unix = cfg!(unix);

        let s = match self {
            Self::Pnpm => unix.then_some("pnpm").unwrap_or("pnpm.cmd"),
        };

        write!(f, "{}", s)
    }
}
