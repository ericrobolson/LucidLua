# RLua
A Rusty wrapper for Lua. Will clone and compile Lua, then link to it.

Right now a basic version is working, but many data types haven't been implemented.

# Usage
```
use rlua::*;

const CODE: &'static str = "
function my_func()
    print(\"Hello from lua!\")
end
";

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;
    m.interpret(CODE)?;
    let result: [Data; 0] = m.call("my_func", [])?;
    let expected: [Data; 0] = [];
    assert_eq!(expected, result);

    Ok(())
}
```

# Examples
- `cargo run --example global_get` will run an example showing how to get a global
- `cargo run --example global_set` will run an example showing how to set a global
- `cargo run --example interpret` will run an example showing how to interpret code
- `cargo run --example lua_calls_rust` will run an example showing how to call Rust code from Lua
- `cargo run --example rust_calls_lua_args` will run an example showing how to call Lua code from Rust with arguments
- `cargo run --example rust_calls_lua` will run an example showing how to call Lua code from Rust with no arguments

# Roadmap
- [ ] Implement rest of data
- [ ] Remove todos + panics
