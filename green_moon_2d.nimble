# Package
version       = "0.1.0"
author        = "Willi Kappler"
description   = "2D game library"
license       = "MIT"
srcDir        = "src"

# Dependencies
requires "nim >= 2.0.0"
requires "naylib"

# Tasks
task testAll, "Run all test cases in tests/":
    #exec "testament --print --verbose c /"
    exec "testament c /"
    # to run only one test file: testament p "tests/test_xxx.nim"

task checkAll, "run 'nim check' on all source files":
    cd "src/"
    exec "nim check green_moon_2d.nim"

    cd "green_moon_2d/"
    #exec "nim check xxx.nim"

task cleanTests, "Clean log files and binaries in tests/ folder":
    cd "tests/"
    # Delete all log files
    exec "rm -f *.log"
    # Delete all executable files
    exec "find . -type f -perm /u=x -delete"

task genDoc, "Generate documentation":
    exec "nim doc --project src/green_moon_2d.nim"

