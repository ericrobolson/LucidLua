use crate::{
    lua_core::{luaL_checknumber, Int, LuaNum, State},
    Data,
};

/// A simple representation of Lua's stack.
pub struct Stack(State);
impl Stack {
    /// Creates a new stack.
    pub fn new(state: State) -> Self {
        Self(state)
    }

    /// Checks for a number on the stack.
    pub fn check_num(&self, stack_position: Int) -> LuaNum {
        unsafe { luaL_checknumber(self.0, stack_position) }
    }

    /// Pushes a value onto the stack.
    pub fn push(&self, data: Data) {
        data.push(self.0)
    }
}
