package = "lua-tabled"
version = "0.1.0-1"

source = {
    url = "git+https://github.com/khvzak/lua-tabled",
    tag = "0.1.0",
}

description = {
    summary = "A pretty print table library written in Rust",
    detailed = [[
        The Lua module written in Rust that provides pretty table support for Lua.
    ]],
    homepage = "https://github.com/zhiburt/lua-tabled",
    license = "MIT"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",
    modules = {
        "tabled"
    },
}
