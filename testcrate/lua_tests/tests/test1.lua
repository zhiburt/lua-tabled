local tabled = require("tabled")

local m = {
    ["string"] = "zxczxczxc",
    ["int"] = 21313,
    ["float"] = 6.14,
    ["char"] = 'c',
    ["nil"] = nil,
    ["error"] = pcall(function() error("some error") end),
    ["function"] = function() return "hi" end,
    ["table"] = { key1 = 1, key2 = 2 },
    ["array"] = { 1, 2 },
    ["thread"] = coroutine.create(function()
        print("hi")
    end)
}

tabled.print(m)
