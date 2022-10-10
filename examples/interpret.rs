use llua::*;

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;

    m.interpret("print('Hello world!')")?;

    Ok(())
}
