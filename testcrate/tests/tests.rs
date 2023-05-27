use std::env;
use std::path::PathBuf;

use mlua::{prelude::LuaString, Lua, Result};

macro_rules! lua_eval {
    ($script:expr) => {
        make_lua().and_then(|lua| {
            lua.load($script)
                .eval::<LuaString>()
                .map(|result| result.to_string_lossy().to_string())
        })
    };
}

macro_rules! lua_exec {
    ($script:expr) => {
        make_lua().and_then(|lua| lua.load($script).exec())
    };
}

macro_rules! lua_test {
    ($test_name:ident, $script:expr) => {
        #[test]
        fn $test_name() {
            lua_exec!($script).unwrap();
        }
    };
    ($test_name:ident, $script:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let result = lua_eval!($script).unwrap();
            let expected = $expected;
            assert_eq!(expected, result);
        }
    };
}

lua_test!(
    test_print,
    r#"
        local tabled = require("tabled")
        tabled.print(_G)
    "#
);

lua_test!(
    test_tostring,
    r#"
        local tabled = require("tabled")
        local data = {
            ["string"] = "zxczxczxc",
            ["int"] = 21313,
            ["float"] = 6.14,
            ["char"] = 'c',
            ["nil"] = nil,
            ["error"] = pcall(function() error("some error") end),
            ["function"] = function() return "hi" end,
            ["table"] = { key1 = 1, key2 = 2 },
            ["array"] = { 1, 2 },
            ["thread"] = coroutine.create(function() print("hi") end),
        }
        local table = tabled.tostring(data)
        print(table)
    "#
);

lua_test!(
    test_builder,
    r#"
        local tabled = require("tabled")
        local data = {
            "zxczxczxc",
            21313,
            6.14,
            'c',
            nil,
            "xxxxxxxxx",
        }
        local table = tabled.table(data):tostring()
        return table
    "#,
    "+---+-----------+\n\
     | 1 | zxczxczxc |\n\
     +---+-----------+\n\
     | 2 | 21313     |\n\
     +---+-----------+\n\
     | 3 | 6.14      |\n\
     +---+-----------+\n\
     | 4 | c         |\n\
     +---+-----------+\n\
     | 6 | xxxxxxxxx |\n\
     +---+-----------+"
);

lua_test!(
    test_builder_theme_utf8,
    r#"
        local tabled = require("tabled")
        local data = {
            "zxczxczxc",
            21313,
            6.14,
            'c',
            nil,
            "xxxxxxxxx",
        }
        local table = tabled.table(data)
        table:theme("UTF8")
        return table:tostring()
    "#,
    "┌───┬───────────┐\n\
     │ 1 │ zxczxczxc │\n\
     ├───┼───────────┤\n\
     │ 2 │ 21313     │\n\
     ├───┼───────────┤\n\
     │ 3 │ 6.14      │\n\
     ├───┼───────────┤\n\
     │ 4 │ c         │\n\
     ├───┼───────────┤\n\
     │ 6 │ xxxxxxxxx │\n\
     └───┴───────────┘"
);

lua_test!(
    test_builder_theme_rounded,
    r#"
        local tabled = require("tabled")
        local data = {
            "zxczxczxc",
            21313,
            6.14,
            'c',
            nil,
            "xxxxxxxxx",
        }
        local table = tabled.table(data)
        table:theme("UTF8-ROUNDED")
        return table:tostring()
    "#,
    "╭───┬───────────╮\n\
     │ 1 │ zxczxczxc │\n\
     ├───┼───────────┤\n\
     │ 2 │ 21313     │\n\
     │ 3 │ 6.14      │\n\
     │ 4 │ c         │\n\
     │ 6 │ xxxxxxxxx │\n\
     ╰───┴───────────╯"
);

lua_test!(
    test_builder_orientation,
    r#"
        local tabled = require("tabled")
        local data = {
            "zxczxczxc",
            21313,
            6.14,
            'c',
            nil,
            "xxxxxxxxx",
        }
        local table = tabled.table(data)
        table:theme("UTF8-ROUNDED")
        table:orientation("HORIZONTAL")
        return table:tostring()
    "#,
    "╭───────────┬───────┬──────┬───┬───────────╮\n\
     │ 1         │ 2     │ 3    │ 4 │ 6         │\n\
     ├───────────┼───────┼──────┼───┼───────────┤\n\
     │ zxczxczxc │ 21313 │ 6.14 │ c │ xxxxxxxxx │\n\
     ╰───────────┴───────┴──────┴───┴───────────╯"
);

lua_test!(
    test_builder_orientation_theme_ascii,
    r#"
        local tabled = require("tabled")
        local data = {
            "zxczxczxc",
            21313,
            6.14,
            'c',
            nil,
            "xxxxxxxxx",
        }
        local table = tabled.table(data)
        table:theme("ASCII")
        table:orientation("HORIZONTAL")
        return table:tostring()
    "#,
    "+-----------+-------+------+---+-----------+\n\
     | 1         | 2     | 3    | 4 | 6         |\n\
     +-----------+-------+------+---+-----------+\n\
     | zxczxczxc | 21313 | 6.14 | c | xxxxxxxxx |\n\
     +-----------+-------+------+---+-----------+"
);

fn make_lua() -> Result<Lua> {
    let cpath = get_libpath();
    let lua = unsafe { Lua::unsafe_new() };
    lua.load(&format!(r#"package.cpath = "{cpath}""#)).exec()?;
    Ok(lua)
}

fn get_libpath() -> String {
    let (dylib_path, dylib_ext, separator);
    if cfg!(target_os = "macos") {
        dylib_path = env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap();
        dylib_ext = "dylib";
        separator = ":";
    } else if cfg!(target_os = "linux") {
        dylib_path = env::var("LD_LIBRARY_PATH").unwrap();
        dylib_ext = "so";
        separator = ":";
    } else if cfg!(target_os = "windows") {
        dylib_path = env::var("PATH").unwrap();
        dylib_ext = "dll";
        separator = ";";
    } else {
        panic!("unknown target os");
    };

    let mut cpath = dylib_path
        .split(separator)
        .take(3)
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push(format!("lib?.{dylib_ext}"));
            path.to_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join(";");

    if cfg!(target_os = "windows") {
        cpath = cpath.replace('\\', "\\\\");
        cpath = cpath.replace("lib?.", "?.");
    }

    cpath
}
