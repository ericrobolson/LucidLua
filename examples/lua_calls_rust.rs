use rlua::*;

pub fn multiplication(state: State) -> Int {
    let stack = Stack::new(state);
    let a = stack.check_num(1);
    let b = stack.check_num(2);

    let n = a * b;

    stack.push(n.into());

    let num_return_values = 1;

    num_return_values
}

fn main() -> Result<(), Error> {
    let mut m = Lua::new();
    m.activate(Library::all())?;

    m.set_global("mul", Data::Function(multiplication))?;
    m.interpret("print(mul(3,4))")?;

    Ok(())
}
