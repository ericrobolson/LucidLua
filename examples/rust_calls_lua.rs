use llua::*;

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
