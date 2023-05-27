local ttest = require("table_test")
local t = require("tabled")

local expected = ttest.create_table({
    "+-------------+",
    "| Hello World |",
    "+-------------+",
})
local table = t.tostring("Hello World")

return ttest.assert_table(table, expected)
