// https://www.lua.org/manual/5.4/manual.html

use core::ffi::c_char;

/// The state of the Lua interpreter.
#[allow(improper_ctypes)]
pub struct LuaState;

pub type Int = i32;
pub type ResultCode = Int;

/// Representation of a Lua function.
pub type LuaFn = fn(State) -> Int;
/// Representation of a Lua integer.
pub type LuaInt = i64;
/// Representation of a Lua number.
pub type LuaNum = f64;
/// Handle to the state of the Lua interpreter.
pub type State = *const LuaState;

#[cfg(windows)]
pub type SizeT = i64;

#[cfg(not(windows))]
pub type SizeT = Int;

pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: i32 = 0;
pub const LUA_TBOOLEAN: i32 = 1;
pub const LUA_TLIGHTUSERDATA: i32 = 2;
pub const LUA_TNUMBER: i32 = 3;
pub const LUA_TSTRING: i32 = 4;
pub const LUA_TTABLE: i32 = 5;
pub const LUA_TFUNCTION: i32 = 6;
pub const LUA_TUSERDATA: i32 = 7;
pub const LUA_TTHREAD: i32 = 8;

pub const LUA_OK: Int = 0;
pub const LUA_YIELD: Int = 1;
pub const LUA_ERRRUN: Int = 2;
pub const LUA_ERRSYNTAX: Int = 3;
pub const LUA_ERRMEM: Int = 4;
pub const LUA_ERRERR: Int = 5;

extern "C" {
    pub fn lua_close(state: State);
    pub fn lua_getglobal(state: State, name: *const u8) -> Int;
    pub fn lua_gettop(state: State) -> Int;
    pub fn lua_isinteger(state: State, index: Int) -> Int;
    pub fn lua_isstring(state: State, index: Int) -> Int;
    pub fn lua_pcallk(
        state: State,
        nargs: Int,
        nresults: Int,
        msgh: Int,
        ctx: Int,
        function: Int,
    ) -> ResultCode;
    pub fn lua_pushboolean(state: State, boolean: LuaInt);
    pub fn lua_pushcclosure(state: State, f: LuaFn, position: Int);
    pub fn lua_pushnil(state: State);
    pub fn lua_pushnumber(state: State, n: LuaNum);
    pub fn lua_pushstring(state: State, s: *const u8) -> *const u8;
    pub fn lua_setglobal(state: State, name: *const u8);
    pub fn lua_settop(state: State, index: Int);
    pub fn lua_toboolean(state: State, idx: Int) -> Int;
    pub fn lua_tonumberx(state: State, index: Int, isnum: *const Int) -> LuaNum;
    pub fn lua_tolstring(state: State, index: Int, len: *const SizeT) -> *const c_char;
    pub fn lua_type(state: State, index: Int) -> Int;
    pub fn luaL_checknumber(state: State, stack: Int) -> LuaNum;
    pub fn luaL_dofile();
    pub fn luaL_loadstring(state: State, string: *const u8) -> ResultCode;
    pub fn luaL_newstate() -> State;
    pub fn luaopen_base(state: State) -> ResultCode;
    pub fn luaopen_coroutine(state: State) -> ResultCode;
    pub fn luaopen_debug(state: State) -> ResultCode;
    pub fn luaopen_io(state: State) -> ResultCode;
    pub fn luaopen_math(state: State) -> ResultCode;
    pub fn luaopen_os(state: State) -> ResultCode;
    pub fn luaopen_package(state: State) -> ResultCode;
    pub fn luaopen_string(state: State) -> ResultCode;
    pub fn luaopen_table(state: State) -> ResultCode;
    pub fn luaopen_utf8(state: State) -> ResultCode;
}

pub unsafe fn lua_pcall(state: State, nargs: Int, nresults: Int, msgh: Int) -> ResultCode {
    lua_pcallk(state, nargs, nresults, msgh, 0, 0)
}

pub unsafe fn lua_pop(state: State, n: Int) {
    lua_settop(state, -(n) - 1)
}

pub unsafe fn lua_pushcfunction(state: State, f: LuaFn) {
    lua_pushcclosure(state, f, 0)
}

pub unsafe fn lua_tonumber(state: State, index: Int) -> LuaNum {
    lua_tonumberx(state, index, core::ptr::null())
}

pub unsafe fn lua_tostring(state: State, idx: Int) -> *const c_char {
    lua_tolstring(state, idx, core::ptr::null())
}

#[cfg(test)]
#[allow(non_camel_case_types, non_snake_case)]
mod lua_tests {
    extern crate alloc;
    use alloc::ffi::CString;

    use super::*;

    #[test]
    fn lua_close_closes() {
        let l = unsafe { luaL_newstate() };
        unsafe { lua_close(l) };
        assert_eq!(true, true);
    }

    #[test]
    fn luaL_newstate_new_state_returns_non_null() {
        let s = unsafe { luaL_newstate() };
        assert_ne!(std::ptr::null(), s);
    }

    #[test]
    fn luaL_loadstring_returns_ok() {
        let string = CString::new("print('Hello')").unwrap();
        let state = unsafe { luaL_newstate() };

        let s = string.as_ptr() as *const u8;

        let result = unsafe { luaL_loadstring(state, s) };
        let expected = LUA_OK;
        assert_eq!(expected, result);
    }
}
