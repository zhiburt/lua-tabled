# lua-tabled
![Build Status]

[Build Status]: https://github.com/zhiburt/lua-tabled/workflows/CI/badge.svg

A pretty table library written in [Rust] for Lua using [tabled].

Thanks to [mlua] library, it supports Lua 5.4/5.3/5.2/5.1 (including LuaJIT) without any effort.
Thanks to [lua-ryaml] for an example of a project layout.

[Rust]: https://www.rust-lang.org
[tabled]: https://github.com/zhiburt/tabled
[mlua]: https://github.com/khvzak/mlua
[lua-ryaml]: https://github.com/khvzak/

## Usage

```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local output = tabled.tostring(data)
print(output)

-- Prints:
-- +-------+-------+
-- | key   | Hello |
-- +-------+-------+
-- | value | World |
-- +-------+-------+
```

### Interface

#### `tabled.tostring`

Takes a lua object and returns a pretty table as a string.

```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local output = tabled.tostring(data)
print(output)

-- Prints:
-- +-------+-------+
-- | key   | Hello |
-- +-------+-------+
-- | value | World |
-- +-------+-------+
```

#### `tabled.print`

Takes a lua object and prints a pretty table into `STDOUT`.

```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
tabled.print(data)

-- Prints:
-- +-------+-------+
-- | key   | Hello |
-- +-------+-------+
-- | value | World |
-- +-------+-------+
```

#### `tabled.table`

Takes a lua object and return a `tabled.table` object, which can be used to configure a pretty table.

```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local t = tabled.table(data)
```

#### `tabled.table.tostring`

Returns a pretty table as a string. 

```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local t = tabled.table(data)
local output = t:tostring()
print(output)

-- Prints:
-- +-------+-------+
-- | key   | Hello |
-- +-------+-------+
-- | value | World |
-- +-------+-------+
```

#### `tabled.table.theme`

Sets a theme for a table.
Available options are:

- `ASCII`
- `UTF8`
- `UTF8-ROUNDED`


```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local t = tabled.table(data)
t:theme("UTF8")
local output = t:tostring()
print(output)

-- Prints:
-- ┌───────┬───────┐
-- │ key   │ Hello │
-- ├───────┼───────┤
-- │ value │ World │
-- └───────┴───────┘
```

#### `tabled.table.orientation`

Sets an orientation for a table.
Available options are:

- `VERTICAL`
- `HORIZONTAL`


```lua
local tabled = require("tabled")

local data = { key = "Hello", value = "World" };
local t = tabled.table(data)
t:orientation("HORIZONTAL")
local output = t:tostring()
print(output)

-- Prints:
-- +-------+-------+
-- | key   | value |
-- +-------+-------+
-- | Hello | World |
-- +-------+-------+
```
