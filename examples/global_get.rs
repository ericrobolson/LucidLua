use rlua::*;

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;
    m.interpret("message = 'Hello world from Lua!'")?;
    let data = m.get_global("message")?;

    println!("{:?}", data);

    Ok(())
}
