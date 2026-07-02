/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the timer class
*/


// Local includes:
#include "gm_timer.hpp"

namespace gm2d {
GMTimer::GMTimer():
    duration(0),
    start_time(std::chrono::steady_clock::now()),
    active(true)
{}

GMTimer::GMTimer(uint32_t d):
    duration(d),
    start_time(std::chrono::steady_clock::now()),
    active(true)
{}

bool GMTimer::gm_finished() {
    if (active) {
        auto now = std::chrono::steady_clock::now();
        auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(now - start_time);
        return elapsed.count() > duration;
    } else {
        return false;
    }
}

void GMTimer::gm_set_duration(uint32_t d) {
    duration = d;
}

void GMTimer::gm_set_duration_restart(uint32_t d) {
    duration = d;
    gm_restart();
}

void GMTimer::gm_restart() {
    start_time = std::chrono::steady_clock::now();
    active = true;
}

void GMTimer::gm_set_active(bool act) {
    active = act;
}
}
