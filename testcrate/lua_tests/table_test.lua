function create_table(lines)
    return table.concat(lines, "\n")
end

function test_table(expected, value)
    local expected = string.gsub(expected, "\n", '\\n')
    local value = string.gsub(value, "\n", '\\n')
    return assert(value == expected, string.format("Assertion failed:\ngot\n%s\nexpected\n%s", value, expected))
end

return {
    assert_table = test_table,
    create_table = create_table,
}