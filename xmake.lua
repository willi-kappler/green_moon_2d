set_project("green_moon_2d")
set_languages("c++23")
set_version("0.1.0")
set_optimize("fastest")
-- set_defaultmode("debug")
set_defaultmode("release")
add_rules("mode.debug", "mode.release")

-- set_optimize("aggressive") -- Becomes -Ofast on GCC/Clang
-- add_vectorexts("native")   -- Becomes -march=native on GCC/Clang

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

before_build(function (target)
    local compiler = target:tool("cxx")
    if compiler then
        if compiler:find("gcc") or compiler:find("g%+%+") then
            cprint("GCC detected: %s", compiler)
        elseif compiler:find("clang") then
            cprint("Clang detected: %s", compiler)
        elseif compiler:find("cl") then
            cprint("MSVC detected: %s", compiler)
        end
    end
end)

-- For all compilers:
set_warnings("all", "extra", "pedantic", "error")
add_cxxflags("-Ofast")
-- add_cxxflags("-Ofast", "-march=native")
add_cxxflags("-Walloca", "-Wcast-align=strict", "-Wimplicit-fallthrough=5")
add_cxxflags("-Wconversion", "-Wshadow", "-Wsign-conversion", "-Wdouble-promotion", "-Wformat=2")
add_cxxflags("-Wundef", "-Wcast-qual", "-Wnon-virtual-dtor", "-Wold-style-cast")
add_cxxflags("-Woverloaded-virtual", "-Wunused", "-Wuninitialized", "-Winit-self")
add_cxxflags("-Wredundant-decls", "-Wsuggest-override")



add_requires("taocpp-json 2025.03.11")
add_requires("snitch")
add_requires("spdlog", {configs = {header_only = false}})
add_requires("libsdl3")
add_requires("libsdl3_image")
-- add_requires("argparse")

target("green_moon_2d")
    set_kind("shared")

    if is_plat("windows") then
        add_rules("utils.symbols.export_all", {export_classes = true})
    end
    add_files("src/gm2d/*.cpp")
    add_packages("taocpp-json")
    add_packages("spdlog")
    add_packages("libsdl3")
    add_packages("libsdl3_image")
    -- For spdlog, so that every object file sees the global logger:
    add_defines("SPDLOG_COMPILED_LIB", {public = true})
    -- Tell xmake which headers to give to the user when installing it:
    add_headerfiles("src/(gm2d/*.hpp)")

target("gm2d_test")
    set_kind("binary")
    add_files("tests/*.cpp")
    add_packages("taocpp-json")
    add_packages("snitch")
    add_packages("spdlog")
    add_packages("libsdl3")
    add_packages("libsdl3_image")
    add_deps("green_moon_2d")
    add_includedirs("src")
    set_default(false) -- Don't build by default

-- Test package locally in xmake repo:
-- xmake l scripts/test.lua --shallow -vD green_moon_2d
-- xmake l scripts/test.lua --shallow -vD -k shared -m debug green_moon_2d
-- xmake l scripts/test.lua --shallow -vD --runtimes=MD green_moon_2d
