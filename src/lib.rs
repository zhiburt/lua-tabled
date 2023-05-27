use mlua::{prelude::LuaString, Function, Lua, Result, Table, ToLua, Value};
use tabled::{
    builder::Builder,
    settings::{style::RawStyle, Style},
};

#[mlua::lua_module]
fn tabled(lua: &Lua) -> Result<Table> {
    make_exports(lua)
}

fn make_exports(lua: &'_ Lua) -> Result<Table<'_>> {
    let print_fn = lua.create_function(mod_print_table)?;
    let tostring_fn = lua.create_function(mod_format_table)?;
    let table_fn = lua.create_function(mod_builder)?;

    let exports = lua.create_table()?;
    exports.set("print", print_fn)?;
    exports.set("tostring", tostring_fn)?;
    exports.set("table", table_fn)?;

    Ok(exports)
}

fn mod_print_table(lua: &'_ Lua, value: Value<'_>) -> Result<()> {
    let table = build_table(lua, value, false)?;
    println!("{table}");
    Ok(())
}

fn mod_format_table<'a>(lua: &'a Lua, value: Value<'_>) -> Result<Value<'a>> {
    let table = build_table(lua, value, false)?;
    let string = table.to_string();
    string.to_lua(lua)
}

fn mod_builder<'a>(lua: &'a Lua, value: Value<'_>) -> Result<Value<'a>> {
    let prop_orientation = lua
        .create_function(|_, arg: (_, _)| builder_prop::<LuaString>(arg.0, "orientation", arg.1))?;
    let prop_theme =
        lua.create_function(|_, arg: (_, _)| builder_prop::<LuaString>(arg.0, "theme", arg.1))?;
    let fn_tostring = lua.create_function(builder_fn_tostring)?;

    let properties = lua.create_table()?;
    properties.set("orientation", "VERTICAL")?;
    properties.set("theme", "ASCII")?;

    let exports = lua.create_table()?;
    exports.set("__data", value)?;
    exports.set("__properties", properties)?;
    exports.set("tostring", fn_tostring)?;
    exports.set("orientation", prop_orientation)?;
    exports.set("theme", prop_theme)?;

    exports.to_lua(lua)
}

fn builder_fn_tostring<'a>(lua: &'a Lua, table: Table<'a>) -> Result<Value<'a>> {
    let data = table.get("__data")?;
    let props: Table<'_> = table.get("__properties")?;
    let theme = props
        .get::<_, LuaString>("theme")?
        .to_string_lossy()
        .into_owned();
    let orientation = props
        .get::<_, LuaString>("orientation")?
        .to_string_lossy()
        .into_owned();
    let is_horizontal = orientation == "HORIZONTAL";
    let theme = peak_theme(&theme).unwrap_or(Style::ascii().into());

    let mut table = build_table(lua, data, is_horizontal)?;
    table.with(theme);

    let string = table.to_string();
    string.to_lua(lua)
}

fn builder_prop<'a, P>(table: Table<'a>, key: &'static str, value: P) -> Result<()>
where
    P: ToLua<'a>,
{
    let props: Table<'_> = table.get("__properties")?;
    props.set(key, value)?;
    Ok(())
}

fn build_table(lua: &'_ Lua, value: Value<'_>, horizontal: bool) -> Result<tabled::Table> {
    let mut b: Builder = Builder::new();

    match value {
        Value::Table(table) => {
            let iter = table.pairs::<Value, Value>();

            if horizontal {
                let mut header = vec![];
                let mut row = vec![];
                for pair in iter {
                    let (key, value) = pair?;

                    let key = value_to_string(lua, key)?;
                    let value = value_to_string(lua, value)?;
                    header.push(key);
                    row.push(value);
                }

                b.set_header(header);
                b.push_record(row);
            } else {
                for pair in iter {
                    let (key, value) = pair?;

                    let key = value_to_string(lua, key)?;
                    let value = value_to_string(lua, value)?;
                    b.push_record([key, value]);
                }
            }
        }
        value => {
            b.push_record([value_to_string(lua, value)?]);
        }
    }

    let table = b.build();

    Ok(table)
}

fn value_to_string(lua: &'_ Lua, value: Value<'_>) -> Result<String> {
    let globals = lua.globals();
    let tostring: Function = globals.get("tostring")?;
    tostring.call::<_, String>(value)
}

fn peak_theme(theme: &str) -> Option<RawStyle> {
    match theme {
        "ASCII" => Some(Style::ascii().into()),
        "UTF8" => Some(Style::modern().into()),
        "UTF8-ROUNDED" => Some(Style::rounded().into()),
        _ => None,
    }
}
