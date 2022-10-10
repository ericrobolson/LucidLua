use crate::lua_core::*;

type ResultCode = Int;

/// Various errors that may occur during opening of a library.
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum LibraryErr {}

/// Various libraries that may be enabled for Lua.
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Library {
    Basic,
    Coroutine,
    Package,
    String,
    Utf8,
    Table,
    Math,
    Io,
    Os,
    Debug,
}
impl Library {
    /// Returns all the libraries.
    pub fn all() -> &'static [Self] {
        &[
            Library::Basic,
            Library::Coroutine,
            Library::Package,
            Library::String,
            Library::Utf8,
            Library::Table,
            Library::Math,
            Library::Io,
            Library::Os,
            Library::Debug,
        ]
    }

    /// Turns on the given library.
    pub(crate) fn enable(&self, state: State) -> Result<(), LibraryErr> {
        let result = unsafe {
            match self {
                Library::Basic => luaopen_base(state),
                Library::Coroutine => luaopen_coroutine(state),
                Library::Package => luaopen_package(state),
                Library::String => luaopen_string(state),
                Library::Utf8 => luaopen_utf8(state),
                Library::Table => luaopen_table(state),
                Library::Math => luaopen_math(state),
                Library::Io => luaopen_io(state),
                Library::Os => luaopen_os(state),
                Library::Debug => luaopen_debug(state),
            }
        };

        open_result_to_result(result)
    }
}

fn open_result_to_result(i: ResultCode) -> Result<(), LibraryErr> {
    match i {
        1 => Ok(()),
        // TODO: need to convert to an actual error
        _ => todo!("openresult '{:?}'", i),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_core::luaL_newstate;

    #[test]
    fn enable_all_libs_no_errors() {
        let s = unsafe { luaL_newstate() };

        for lib in Library::all() {
            let result = lib.enable(s);
            let expected: Result<(), LibraryErr> = Ok(());
            assert_eq!((*lib, expected), (*lib, result));
        }
    }

    #[test]
    fn luaopen_base_returns_ok() {
        let result = unsafe { luaopen_base(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_coroutine_returns_ok() {
        let result = unsafe { luaopen_coroutine(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_debug_returns_ok() {
        let result = unsafe { luaopen_debug(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_io_returns_ok() {
        let result = unsafe { luaopen_io(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_math_returns_ok() {
        let result = unsafe { luaopen_math(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_package_returns_ok() {
        let result = unsafe { luaopen_package(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_os_returns_ok() {
        let result = unsafe { luaopen_os(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_string_returns_ok() {
        let result = unsafe { luaopen_string(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_table_returns_ok() {
        let result = unsafe { luaopen_table(luaL_newstate()) };

        assert_eq!(1, result);
    }

    #[test]
    fn luaopen_utf8_returns_ok() {
        let result = unsafe { luaopen_utf8(luaL_newstate()) };

        assert_eq!(1, result);
    }
}
