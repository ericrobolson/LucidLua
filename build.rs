use std::{fs::create_dir_all, path::PathBuf, process::Command};

extern crate cc;

fn main() {
    let lua_path: PathBuf = ".lua".into();

    // Clone Lua
    if !lua_path.exists() {
        create_dir_all(&lua_path).unwrap();
        let lua_repo = "https://github.com/ericrobolson/lua.git";

        let clone_output = Command::new("git")
            .arg("clone")
            .arg(lua_repo)
            .arg(&lua_path)
            .output()
            .unwrap();

        if !clone_output.status.success() {
            let error = String::from_utf8(clone_output.stderr.clone()).unwrap();

            panic!("{error}");
        }
    }

    // Build Lua
    const LUA_INT_LONGLONG: &'static str = "3";
    const LUA_FLOAT_DOUBLE: &'static str = "2";
    cc::Build::new()
        .file(".lua/onelua.c")
        .include(".lua/")
        .define("MAKE_LIB", "1")
        .define("LUA_INT_TYPE", LUA_INT_LONGLONG)
        .define("LUA_FLOAT_TYPE", LUA_FLOAT_DOUBLE)
        .compile("liblua");
}
