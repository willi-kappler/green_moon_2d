set_project("green_moon_2d")
    set_languages("c++23")
    set_version("0.2.0")
    set_optimize("fastest")
    -- set_defaultmode("debug")
    set_defaultmode("release")
    add_rules("mode.debug", "mode.release")

-- Clean configuration:
-- xmake f -c

-- Set configuration:
-- xmake f -m release
-- xmake f -m debug

-- Set clang:
-- xmake f --toolchain=clang --runtimes=c++_shared
-- xmake f --toolchain=clang --runtimes=stdc++_shared

-- Force rebuild:
-- rm -rf build/ .xmake ~/.xmake/
-- xmake -a -r

-- Just clean cache and update:
-- xrepo clean
-- xrepo update-repo

-- For all compilers:
set_warnings("all", "extra", "pedantic", "error")

-- For GCC and Clang only:
if is_kind("gcc", "clang") then
    -- For GCC only:
    if is_kind("gcc") then
        add_cxxflags("-Walloca", "-Wcast-align=strict", "-Wimplicit-fallthrough=5")
    elseif is_kind("clang") then
        -- Clang prefers the boolean flag without the numeric level
        add_cxxflags("-Wimplicit-fallthrough")
    end
--    add_cxxflags("-Wnull-dereference", "-Wswitch-enum")
    add_cxxflags("-Wconversion", "-Wshadow", "-Wsign-conversion", "-Wdouble-promotion", "-Wformat=2")
    add_cxxflags("-Wundef", "-Wcast-qual", "-Wnon-virtual-dtor", "-Wold-style-cast")
    add_cxxflags("-Woverloaded-virtual", "-Wunused", "-Wuninitialized", "-Winit-self")
    add_cxxflags("-Wredundant-decls", "-Wsuggest-override")
end

-- For MSVC only:
if is_kind("cl") then
    -- add_cxxflags("/Wall")
    add_cxxflags("/W4") -- Level 4 is the standard "Strict" for MSVC
    add_cxxflags("/w14242") -- 'identifier': conversion from 'type1' to 'type1', possible loss of data
    add_cxxflags("/w14265") -- 'class': class has virtual functions, but destructor is not virtual
    add_cxxflags("/w14287") -- 'operator': unsigned/negative constant mismatch
    add_cxxflags("/we4289") -- nonstandard extension used: 'variable': control variable declared in the for-loop is used outside the for-loop scope
    add_cxxflags("/w14296") -- 'operator': expression is always false
    add_cxxflags("/w14311") -- 'variable' : pointer truncation from 'type' to 'type'
    add_cxxflags("/wd4068") -- disable "unknown pragma" (useful if you use GCC pragmas)
    add_cxxflags("/utf-8")
end

add_requires("taocpp-json 2025.03.11")
add_requires("snitch")
add_requires("spdlog", {configs = {header_only = false}})
-- add_requires("argparse")

target("green_moon_2d")
    set_kind("shared")
    if is_plat("windows") then
        add_rules("utils.symbols.export_all", {export_classes = true})
    end
    add_files("src/gm2d/*.cpp")
    add_packages("taocpp-json")
    add_packages("spdlog")
    -- For spdlog, so that every object file sees the global logger:
    add_defines("SPDLOG_COMPILED_LIB", {public = true})
    -- Tell xmake which headers to give to the user when installing it:
    add_headerfiles("src/(gm2d/*.hpp)")

target("gm2d_test")
    set_kind("binary")
    add_files("tests/test_all.cpp")
    add_packages("taocpp-json")
    add_packages("snitch")
    add_packages("spdlog")
    add_deps("green_moon_2d")
    add_includedirs("src")
    set_default(false) -- Don't build by default

-- Test package locally in xmake repo:
-- xmake l scripts/test.lua --shallow -vD node_crunch2
-- xmake l scripts/test.lua --shallow -vD -k shared -m debug node_crunch2
-- xmake l scripts/test.lua --shallow -vD --runtimes=MD node_crunch2
