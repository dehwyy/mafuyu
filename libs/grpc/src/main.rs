fn main() -> std::io::Result<()> {

    {
        let protos = vec!(
            "api/general.proto", "api/auth.proto", "api/integrations.proto",
            "api/oauth2.proto", "api/passport.proto", "api/user.proto",
            "api/tokens.proto", "api/cdn.proto", "api.proto"
        );

        let cwd = std::env::current_dir().unwrap();
        let cwd = cwd.to_string_lossy();
        let out_dir = format!("{}/src/pkg", cwd);
        std::fs::create_dir_all(&out_dir)?;

        std::env::set_var("OUT_DIR", &out_dir);

        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional")
            .compile(
                &protos.iter().map(|proto| format!("{0}", proto)).collect::<Vec<_>>(),
                &vec!("protos").iter().map(|path| format!("{0}/{1}", cwd, path)).collect::<Vec<_>>(),
            ).expect("cannot compile rust protos!");
    }

    Ok(())
}
