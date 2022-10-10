use llua::*;

const CODE: &'static str = "
function my_func(a,b)
    return a * b
end
";

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;
    m.interpret(CODE)?;

    let result = m.call("my_func", [3.into(), 4.into()])?;

    assert_eq!([Data::Number(12.0)], result);

    println!("3 x 4 = {:?}", result[0]);

    Ok(())
}
