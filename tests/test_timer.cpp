/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the timer class.

    Run only configuration tests:
    xmake run -w ./ gm2d_test [timer]
*/

// STD includes:
#include <thread>

// External includes:
#include <snitch/snitch.hpp>

// Local includes:
#include "gm2d/gm_timer.hpp"

using namespace gm2d;

TEST_CASE("Create Timer", "[timer]" ) {
    GMTimer t1{};
    REQUIRE(t1.gm_finished());

    t1 = GMTimer(1000); // 1000 ms = 1s
    REQUIRE(!t1.gm_finished());

    auto const sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());
}

TEST_CASE("Set timer duration", "[timer]" ) {
    GMTimer t1{};
    REQUIRE(t1.gm_finished());

    t1.gm_set_duration(1000);
    REQUIRE(!t1.gm_finished());

    auto const sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());
}

TEST_CASE("Restart timer", "[timer]" ) {
    GMTimer t1{1000};
    REQUIRE(!t1.gm_finished());

    auto sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());

    t1.gm_restart();
    REQUIRE(!t1.gm_finished());

    sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());
}

TEST_CASE("Set timer duration restart", "[timer]" ) {
    GMTimer t1{1000};
    REQUIRE(!t1.gm_finished());

    auto sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());

    t1.gm_set_duration_restart(500);
    REQUIRE(!t1.gm_finished());

    sleep_time = std::chrono::milliseconds(500);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());
}

TEST_CASE("Set active", "[timer]" ) {
    GMTimer t1{1000};
    REQUIRE(!t1.gm_finished());

    auto sleep_time = std::chrono::milliseconds(1000);
    std::this_thread::sleep_for(sleep_time);
    REQUIRE(t1.gm_finished());

    t1.gm_set_active(false);
    REQUIRE(!t1.gm_finished());
    t1.gm_set_active(true);
    REQUIRE(t1.gm_finished());
}
