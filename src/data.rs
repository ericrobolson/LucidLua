extern crate alloc;

use crate::{lua_core::*, Type};
use alloc::{ffi::CString, string::String};
use core::ffi::CStr;

#[derive(Clone, Debug, PartialEq)]
pub enum DataErr {
    StackUnderflow,
}

/// A piece of data.
#[derive(Clone, PartialEq, Debug)]
pub enum Data {
    Nil,
    Bool(bool),
    Number(LuaNum),
    String(String),
    Function(LuaFn),
    // TODO: implement this
    LightUserData,
    // TODO: implement this
    Table,
    // TODO: implement this
    UserData,
    // TODO: implement this
    Thread,
}
impl Data {
    /// Gets the type for the given position in the stack.
    pub fn m_type(&self) -> Type {
        match self {
            Data::Nil => Type::Nil,
            Data::Bool(_) => Type::Bool,
            Data::LightUserData => Type::LightUserData,
            Data::Number(_) => Type::Number,
            Data::String(_) => Type::String,
            Data::Table => Type::Table,
            Data::Function(_) => Type::Function,
            Data::UserData => Type::UserData,
            Data::Thread => Type::Thread,
        }
    }

    /// Attempts to pop some data off the stack.
    pub(crate) fn pop(state: State) -> Result<Self, DataErr> {
        let mtype = Type::get_type(state, -1);
        let has_data = unsafe { lua_gettop(state) > 0 };

        if !has_data {
            return Err(DataErr::StackUnderflow);
        }

        let data = match mtype {
            Some(ty) => match ty {
                Type::Nil => Data::Nil,
                Type::Bool => {
                    let value = unsafe { lua_toboolean(state, -1) };
                    Data::Bool(value > 0)
                }
                Type::Number => {
                    let value = unsafe { lua_tonumber(state, -1) };
                    Data::Number(value)
                }
                Type::String => {
                    let chars = unsafe { lua_tostring(state, -1) };
                    let cstr = unsafe { CStr::from_ptr(chars) };
                    let cstring = CString::from(cstr);

                    Data::String(String::from(cstring.to_str().unwrap()))
                }
                Type::LightUserData => todo!(),
                Type::Table => todo!(),
                Type::Function => todo!(),
                Type::UserData => todo!(),
                Type::Thread => todo!(),
            },
            None => Self::Nil,
        };

        unsafe {
            if has_data {
                lua_pop(state, 1);
            }
        }

        Ok(data)
    }

    /// Pushes the given data onto the stack.
    pub(crate) fn push(&self, state: State) {
        unsafe {
            match self {
                Data::Nil => lua_pushnil(state),
                Data::Bool(boolean) => match boolean {
                    true => lua_pushboolean(state, 1),
                    false => lua_pushboolean(state, 0),
                },
                Data::LightUserData => todo!(),
                Data::Number(num) => lua_pushnumber(state, *num),
                Data::String(s) => {
                    let s = CString::new(s.clone()).unwrap();
                    lua_pushstring(state, s.as_c_str().as_ptr() as *const u8);
                }
                Data::Table => todo!(),
                Data::Function(f) => lua_pushcfunction(state, *f),
                Data::UserData => todo!(),
                Data::Thread => todo!(),
            }
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::Nil
    }
}
impl<'a> From<&'a str> for Data {
    fn from(s: &'a str) -> Self {
        Data::String(s.into())
    }
}
impl From<LuaNum> for Data {
    fn from(n: LuaNum) -> Self {
        Data::Number(n)
    }
}

impl From<i64> for Data {
    fn from(n: i64) -> Self {
        Data::Number(n as LuaNum)
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_core::luaL_newstate;

    use super::*;

    #[test]
    fn default_returns_nil() {
        assert_eq!(Data::Nil, Data::default())
    }

    #[test]
    fn from_f64_returns_number() {
        let n = 34.0;
        let d: Data = n.into();
        assert_eq!(Data::Number(n), d)
    }

    #[test]
    fn from_i64_returns_number() {
        let n = 34;
        let d: Data = n.into();
        assert_eq!(Data::Number(n as LuaNum), d)
    }

    #[test]
    fn from_str_returns_string() {
        let n = "hello";
        let d: Data = n.into();
        assert_eq!(Data::String(n.into()), d)
    }

    #[test]
    fn pop_nil_value_returns_underflow() {
        let lua = unsafe { luaL_newstate() };

        let result = Data::pop(lua);
        let expected = Err(DataErr::StackUnderflow);
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_string() {
        let data = Data::String("Hello from Rust!".into());
        let lua = unsafe { luaL_newstate() };
        data.clone().push(lua);

        let result = Data::pop(lua);
        let expected = Ok(data.clone());
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_num_positive() {
        let data = Data::Number(123.44);
        let lua = unsafe { luaL_newstate() };
        data.clone().push(lua);

        let result = Data::pop(lua);
        let expected = Ok(data.clone());
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_num_negative() {
        let data = Data::Number(-234568.93);
        let lua = unsafe { luaL_newstate() };
        data.clone().push(lua);

        let result = Data::pop(lua);
        let expected = Ok(data.clone());
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_bool_true() {
        let data = Data::Bool(true);
        let lua = unsafe { luaL_newstate() };
        data.push(lua);

        let result = Data::pop(lua);
        let expected = Ok(Data::Bool(true));
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_bool_false() {
        let data = Data::Bool(false);
        let lua = unsafe { luaL_newstate() };
        data.push(lua);

        let result = Data::pop(lua);
        let expected = Ok(Data::Bool(false));
        assert_eq!(expected, result)
    }

    #[test]
    fn push_pop_nil() {
        let data = Data::Nil;
        let lua = unsafe { luaL_newstate() };
        data.push(lua);

        let result = Data::pop(lua);
        let expected = Ok(Data::Nil);
        assert_eq!(expected, result)
    }

    #[test]
    fn data_mtype_nil() {
        let data = Data::Nil;
        let expected = Type::Nil;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_bool() {
        let data = Data::Bool(true);
        let expected = Type::Bool;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_lightuserdata() {
        let data = Data::LightUserData;
        let expected = Type::LightUserData;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_number() {
        let data = Data::Number(0.0);
        let expected = Type::Number;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_string() {
        let data = Data::String("Hello".into());
        let expected = Type::String;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_table() {
        let data = Data::Table;
        let expected = Type::Table;
        assert_eq!(expected, data.m_type());
    }

    fn dummy_fn(_: State) -> Int {
        1
    }

    #[test]
    fn data_mtype_function() {
        let data = Data::Function(dummy_fn);
        let expected = Type::Function;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_userdata() {
        let data = Data::UserData;
        let expected = Type::UserData;
        assert_eq!(expected, data.m_type());
    }

    #[test]
    fn data_mtype_thread() {
        let data = Data::Thread;
        let expected = Type::Thread;
        assert_eq!(expected, data.m_type());
    }
}
