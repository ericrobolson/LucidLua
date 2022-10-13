extern crate alloc;
use crate::alloc::string::ToString;
use crate::{lua_core::*, Data, Error, Library, LibraryErr, Stack};
use alloc::ffi::CString;
use core::ffi::CStr;

/// Abstraction for a Lua runtime.
pub struct Lua {
    lua: *const LuaState,
}
impl Lua {
    /// Creates a new instance of Lua.
    pub fn new() -> Self {
        let lua = unsafe { luaL_newstate() };

        Self { lua }
    }

    /// Returns a handle to the stack.
    pub fn stack(&self) -> Stack {
        Stack::new(self.lua)
    }

    /// Activates the given libraries.
    pub fn activate<'a>(&mut self, libaries: &'a [Library]) -> Result<&mut Self, LibraryErr> {
        for lib in libaries {
            lib.enable(self.lua)?;
        }

        Ok(self)
    }

    /// Attempts to call the given Lua function.
    pub fn call<'a, const RETURN_VALUES: usize, const ARGS: usize>(
        &mut self,
        function_name: &'a str,
        args: [Data; ARGS],
    ) -> Result<[Data; RETURN_VALUES], Error> {
        let function_name = CString::new(function_name).unwrap();
        self.call_no_alloc(&function_name, args)
    }

    /// Attempts to call the given Lua function.
    /// May perform allocations for retrieving the data.
    fn call_no_alloc<'a, const RETURN_VALUES: usize, const ARGS: usize>(
        &mut self,
        function_name: &'a CString,
        args: [Data; ARGS],
    ) -> Result<[Data; RETURN_VALUES], Error> {
        const NIL: Data = Data::Nil;
        let mut data: [Data; RETURN_VALUES] = [NIL; RETURN_VALUES];
        unsafe {
            // Put function on stack
            lua_getglobal(self.lua, map_cstr(&function_name));

            // Put args on stack
            for arg in args {
                arg.push(self.lua);
            }

            // Call function
            if lua_pcall(self.lua, ARGS as Int, RETURN_VALUES as Int, 0) == LUA_OK {
                // Get data from stack
                for return_value in data.iter_mut() {
                    *return_value = Data::pop(self.lua)?;
                }

                // Remove function from stack
                lua_pop(self.lua, lua_gettop(self.lua));
            } else {
                return Err(self.get_error());
            }
        }

        // Since we popped stuff off the stack, but it in logical order.
        data.reverse();

        Ok(data)
    }

    /// Returns the error from Lua
    fn get_error(&self) -> Error {
        let chars = unsafe { lua_tostring(self.lua, lua_gettop(self.lua)) };
        let cstr = unsafe { CStr::from_ptr(chars) };
        let cstring = CString::from(cstr);
        let error = cstring.to_str().unwrap().to_string();

        Error::Runtime(error)
    }

    /// Attempts to retrieve the given global.
    pub fn get_global<'a>(&self, global_name: &'a str) -> Result<Data, Error> {
        let global_name = CString::new(global_name).unwrap();
        self.get_global_no_alloc(&global_name)
    }

    /// Attempts to retrieve the given global.
    /// May perform an allocation for retrieving the data.
    fn get_global_no_alloc<'a>(&self, global_name: &'a CString) -> Result<Data, Error> {
        unsafe {
            lua_getglobal(self.lua, map_cstr(global_name));
        }

        let data = Data::pop(self.lua)?;
        Ok(data)
    }

    /// Interprets the given code.
    pub fn interpret<'a>(&mut self, code: &'a str) -> Result<(), Error> {
        let code = CString::new(code).unwrap();
        self.interpret_noalloc(&code)
    }

    /// Interprets the given code.
    /// No allocations are performed.
    fn interpret_noalloc<'a>(&mut self, code: &CString) -> Result<(), Error> {
        unsafe {
            self.map_code(luaL_loadstring(self.lua, map_cstr(code)))?;
            self.map_code(lua_pcall(self.lua, 0, 0, 0))?;

            // If executed successfully remove from the stack
            lua_pop(self.lua, lua_gettop(self.lua));
        };

        Ok(())
    }

    /// Sets the given global variable.
    pub fn set_global<'a>(&mut self, global_name: &'a str, data: Data) -> Result<(), Error> {
        let global_name = CString::new(global_name).unwrap();
        self.set_global_noalloc(&global_name, data)
    }

    /// Sets the given global variable.
    /// Does not perform any allocations.
    fn set_global_noalloc(&mut self, global_name: &CString, data: Data) -> Result<(), Error> {
        unsafe {
            data.push(self.lua);
            lua_setglobal(self.lua, map_cstr(global_name));
        }
        Ok(())
    }

    fn map_code(&self, result_code: ResultCode) -> Result<(), Error> {
        match result_code {
            LUA_OK => Ok(()),
            _ => Err(self.get_error()),
        }
    }
}
impl Drop for Lua {
    fn drop(&mut self) {
        unsafe { lua_close(self.lua) };
    }
}

/// Maps a cstring to a poitner
fn map_cstr<'a>(c: &CString) -> *const u8 {
    c.as_ptr() as *const u8
}

#[cfg(test)]
mod tests {
    use core::f64::INFINITY;

    use crate::lua_core::{luaL_checknumber, Int, State};

    use super::*;

    #[test]
    fn call_with_single_return_value_two_args() {
        const CODE: &'static str = "
function my_func(a,b)
    return a * b
end
";

        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret(CODE).unwrap();

        let result = m.call("my_func", [3.into(), 4.into()]).unwrap();

        assert_eq!([Data::Number(12.0)], result);
    }

    #[test]
    fn call_with_multiple_return_values_two_args() {
        const CODE: &'static str = "
function swapper(a,b)
    return b,a
end
";

        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret(CODE).unwrap();

        let result = m.call("swapper", [3.into(), 4.into()]).unwrap();

        assert_eq!([Data::Number(4.0), 3.0.into()], result);
    }

    #[test]
    fn call_div_zero() {
        const CODE: &'static str = "
function div(a,b)
    return a / b
end
";

        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret(CODE).unwrap();

        let result = m.call("div", [3.into(), 0.into()]).unwrap();

        assert_eq!([Data::Number(INFINITY)], result);
    }

    #[test]
    fn call_calls_code_correctly() {
        let code = "
function my_function()
    print(\"Hello from Function in Lua\")
end
";

        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret(code).unwrap();
        let result = m.call("my_function", []);
        let expected = Ok([]);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_global_returns_nil() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret("myGlobal = nil").unwrap();
        let result = m.get_global("myGlobal");
        let expected = Ok(Data::Nil);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_global_returns_bool_true() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret("myGlobal = true").unwrap();
        let result = m.get_global("myGlobal");
        let expected = Ok(Data::Bool(true));
        assert_eq!(expected, result);
    }

    #[test]
    fn get_global_returns_bool_false() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret("myGlobal = false").unwrap();
        let result = m.get_global("myGlobal");
        let expected = Ok(Data::Bool(false));
        assert_eq!(expected, result);
    }

    #[test]
    fn get_global_returns_number() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret("myGlobal = 123.44").unwrap();
        let result = m.get_global("myGlobal");
        let expected = Ok(Data::Number(123.44));
        assert_eq!(expected, result);
    }

    #[test]
    fn get_global_returns_string() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.interpret("myGlobal = 'Hello from Lua!'").unwrap();
        let result = m.get_global("myGlobal");
        let expected = Ok(Data::String("Hello from Lua!".into()));
        assert_eq!(expected, result);
    }

    #[test]
    fn interpret_calls_fn() {
        pub fn multiplication(state: State) -> Int {
            let a = unsafe { luaL_checknumber(state, 1) };
            let b = unsafe { luaL_checknumber(state, 2) };

            Data::Number(a * b).push(state);

            let num_return_values = 1;

            num_return_values
        }

        let data = Data::Function(multiplication);
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        m.set_global("mul", data).unwrap();

        m.interpret("print(mul(7,8))").unwrap();
    }

    #[test]
    fn activate_all_libs_no_errors() {
        let mut m = Lua::new();
        let result = m.activate(Library::all());
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn interpret_noalloc_print() {
        let code = CString::new("print('Hello, World')").unwrap();
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        let result = m.interpret_noalloc(&code);

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn interpret_print() {
        let mut m = Lua::new();
        m.activate(Library::all()).unwrap();
        let result = m.interpret("print('Hello, World')");

        assert_eq!(Ok(()), result);
    }
}
