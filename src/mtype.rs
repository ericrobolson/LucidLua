use crate::lua_core::*;

/// The various types that can be represented in Lua.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    Nil,
    Bool,
    LightUserData,
    Number,
    String,
    Table,
    Function,
    UserData,
    Thread,
}
impl Type {
    /// Gets the type for the given position in the stack.
    pub(crate) fn get_type(state: State, index: Int) -> Option<Self> {
        let lua_type = unsafe { lua_type(state, index) };
        cast_type(lua_type)
    }
}

fn cast_type(lua_type: Int) -> Option<Type> {
    match lua_type {
        LUA_TNIL => Some(Type::Nil),
        LUA_TBOOLEAN => Some(Type::Bool),
        LUA_TLIGHTUSERDATA => Some(Type::LightUserData),
        LUA_TNUMBER => Some(Type::Number),
        LUA_TSTRING => Some(Type::String),
        LUA_TTABLE => Some(Type::Table),
        LUA_TFUNCTION => Some(Type::Function),
        LUA_TUSERDATA => Some(Type::UserData),
        LUA_TTHREAD => Some(Type::Thread),
        LUA_TNONE | _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_type_nil() {
        let ty = LUA_TNIL;
        let expected = Some(Type::Nil);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_bool() {
        let ty = LUA_TBOOLEAN;
        let expected = Some(Type::Bool);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_light_user_data() {
        let ty = LUA_TLIGHTUSERDATA;
        let expected = Some(Type::LightUserData);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_number() {
        let ty = LUA_TNUMBER;
        let expected = Some(Type::Number);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_string() {
        let ty = LUA_TSTRING;
        let expected = Some(Type::String);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_table() {
        let ty = LUA_TTABLE;
        let expected = Some(Type::Table);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_function() {
        let ty = LUA_TFUNCTION;
        let expected = Some(Type::Function);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_user_data() {
        let ty = LUA_TUSERDATA;
        let expected = Some(Type::UserData);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_thread() {
        let ty = LUA_TTHREAD;
        let expected = Some(Type::Thread);
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_none() {
        let ty = LUA_TNONE;
        let expected = None;
        assert_eq!(expected, cast_type(ty))
    }

    #[test]
    fn cast_type_invalid_returns_none() {
        let ty = 9999;
        let expected = None;
        assert_eq!(expected, cast_type(ty))
    }
}
