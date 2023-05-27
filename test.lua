#!/bin/usr/lua

local function read_dir(path)
    local command = [[ls -pa ]] .. path .. [[ | grep -v /]]
    local dir = {}
    for file in io.popen(command):lines() do
        dir[#dir + 1] = file
    end

    return dir
end

local lib_path = os.getenv("PATH_LIB")
if not lib_path then
    lib_path = "./target/debug"
end

local tests_path = os.getenv("PATH_TEST")
if not tests_path then
    tests_path = "./testcrate/lua_tests/tests"
end

local dynlib_path = lib_path .. "/lib?.so"

package.cpath = package.cpath .. ";" .. dynlib_path
package.path = package.path .. ";./testcrate/lua_tests/?.lua"

local files = read_dir(tests_path)
if not files then
    error("fail to collect test files")
end

local failed = false;
for _, file in ipairs(files) do
    local path = tests_path .. '/' .. file

    local f = loadfile(path)
    if not f then
        error("unexpected lua file")
    end

    local ok, message = pcall(f)
    if not ok then
        failed = true
    end

    print(file, ok, message)
end

if failed then
    os.exit(-1)
end
