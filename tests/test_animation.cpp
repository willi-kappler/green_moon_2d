/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file contains the tests for the animation class.

    Run only configuration tests:
    xmake run -w ./ gm2d_test [animation]
*/

// STD includes:
#include <thread>

// External includes:
#include <snitch/snitch.hpp>

// Local includes:
#include "gm2d/gm_animation.hpp"

using namespace gm2d;

TEST_CASE("Create animation.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::vector<std::pair<uint16_t, uint16_t>> frames{{5, 100}, {10, 100}, {8, 300}};
    GMAnimation a2(frames);
    REQUIRE(a2.gm_finished());
    REQUIRE(a2.gm_get_frame_index() == 5);
}

TEST_CASE("Change animation repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::FIXED);
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    a1.gm_change_repetition(GMRepetition::FORWARD);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    a1.gm_change_repetition(GMRepetition::BACKWARD);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    a1.gm_change_repetition(GMRepetition::FORWARD_LOOP);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    a1.gm_change_repetition(GMRepetition::BACKWARD_LOOP);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    a1.gm_change_repetition(GMRepetition::PINGPONG_F);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    a1.gm_change_repetition(GMRepetition::PINGPONG_B);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);
}

TEST_CASE("Test update with fixed repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::FIXED);
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    auto const sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);
}

TEST_CASE("Test update with forward repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::FORWARD);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    auto const sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);
}

TEST_CASE("Test update with backward repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::BACKWARD);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    auto sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);
}

TEST_CASE("Test update with forward loop repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::FORWARD_LOOP);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    auto sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);
}

TEST_CASE("Test update with backward loop repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::BACKWARD_LOOP);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    auto sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);
}

TEST_CASE("Test update with pingpong forward repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::PINGPONG_F);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    auto sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);
}

TEST_CASE("Test update with pingpong backward repetition.", "[animation]") {
    GMAnimation a1{{5, 100}, {10, 100}, {8, 300}};
    a1.gm_change_repetition(GMRepetition::PINGPONG_B);
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    auto sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 8);

    sleep_time = std::chrono::milliseconds(300);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 10);

    sleep_time = std::chrono::milliseconds(100);
    std::this_thread::sleep_for(sleep_time);
    a1.gm_update();
    REQUIRE(!a1.gm_finished());
    REQUIRE(a1.gm_get_frame_index() == 5);
}
