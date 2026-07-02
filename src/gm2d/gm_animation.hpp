/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the animation class
*/

#ifndef FILE_GM_ANIMATION_HPP_INCLUDED
#define FILE_GM_ANIMATION_HPP_INCLUDED

// STD includes:
#include <cstdint>
//#include <stdfloat>
#include <utility>
#include <array>
//#include <optional>
#include <vector>
#include <initializer_list>

// Local includes:
#include "gm_math.hpp"
#include "gm_timer.hpp"

namespace gm2d {
class GMAnimation {
    public:
        // Constructor:
        GMAnimation();
        GMAnimation(const std::vector<std::pair<uint16_t, uint16_t>> &);
        GMAnimation(std::vector<std::pair<uint16_t, uint16_t>> &&);
        GMAnimation(std::initializer_list<std::pair<uint16_t, uint16_t>>);

        // Methods
        void gm_update();
        bool gm_finished();
        void gm_change_repetition(GMRepetition);
        uint16_t gm_get_frame_index();
        void gm_set_timer_duration();
        void gm_set_active(bool);

    private:
        size_t current_frame;
        GMRepetition repetition;
        std::vector<std::pair<uint16_t, uint16_t>> frames;
        GMTimer timer;
        bool active;
};
}

#endif // FILE_GM_ANIMATION_HPP_INCLUDED
