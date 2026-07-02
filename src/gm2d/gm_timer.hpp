/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the timer class
*/

#ifndef FILE_GM_TIMER_HPP_INCLUDED
#define FILE_GM_TIMER_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <chrono>

namespace gm2d {
class GMTimer {
    public:
        // Constructor:
        GMTimer();
        GMTimer(uint32_t);

        // Methods:
        bool gm_finished();
        void gm_set_duration(uint32_t);
        void gm_set_duration_restart(uint32_t);
        void gm_restart();
        void gm_set_active(bool);

    private:
        uint32_t duration;
        std::chrono::time_point<std::chrono::steady_clock> start_time;
        bool active;
};
}

#endif // FILE_GM_TIMER_HPP_INCLUDED
