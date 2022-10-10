use llua::*;

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;
    m.set_global("answer", 42.0.into())?;
    m.interpret("print(answer)")?;

    m.set_global("answer", "Foo".into())?;
    m.interpret("print(answer)")?;

    Ok(())
}
