/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the animation class
*/


// Local includes:
#include "gm_animation.hpp"

namespace gm2d {
GMAnimation::GMAnimation():
    current_frame(0),
    repetition(GMRepetition::FIXED),
    frames(),
    active(false)
{}

GMAnimation::GMAnimation(const std::vector<std::pair<uint16_t, uint16_t>> &data):
    current_frame(0),
    repetition(GMRepetition::FIXED),
    frames(data),
    active(false)
{}

GMAnimation::GMAnimation(std::vector<std::pair<uint16_t, uint16_t>> &&data):
    current_frame(0),
    repetition(GMRepetition::FIXED),
    frames(data),
    active(false)
{}

GMAnimation::GMAnimation(std::initializer_list<std::pair<uint16_t, uint16_t>> data):
    current_frame(0),
    repetition(GMRepetition::FIXED),
    frames(data),
    active(false)
{}

void GMAnimation::gm_update() {
    // if self.active and self.timer.finished():

    const size_t last_frame = frames.size() - 1;

    switch(repetition) {
        case GMRepetition::FIXED:
            // Nothing to do...
        break;
        case GMRepetition::FORWARD:
            if (current_frame < last_frame) {
                current_frame++;
                gm_set_timer_duration();
            } else {
                active = false;
            }
        break;
        case GMRepetition::BACKWARD:
            if (current_frame > 0) {
                current_frame--;
                gm_set_timer_duration();
            } else {
                active = false;
            }
        break;
        case GMRepetition::FORWARD_LOOP:
            if (current_frame < last_frame) {
                current_frame++;
            } else {
                current_frame = 0;
            }

            gm_set_timer_duration();
        break;
        case GMRepetition::BACKWARD_LOOP:
            if (current_frame > 0) {
                current_frame--;
            } else {
                current_frame = last_frame;
            }

            gm_set_timer_duration();
        break;
        case GMRepetition::PINGPONG_F:
            if (current_frame < last_frame) {
                current_frame++;
            } else {
                current_frame--;
                repetition = GMRepetition::PINGPONG_B;
            }

            gm_set_timer_duration();
        break;
        case GMRepetition::PINGPONG_B:
            if (current_frame > 0) {
                current_frame--;
            } else {
                current_frame++;
                repetition = GMRepetition::PINGPONG_F;
            }

            gm_set_timer_duration();
        break;
    }
}

bool GMAnimation::gm_finished() {
    switch (repetition) {
        case GMRepetition::FIXED:
            return true;
        break;
        case GMRepetition::FORWARD:
            return current_frame == frames.size() - 1;
        break;
        case GMRepetition::BACKWARD:
            return current_frame == 0;
        break;
        default:
            return false;
        break;
    }
}

void GMAnimation::gm_change_repetition(GMRepetition rep) {
    const size_t last_frame = frames.size() - 1;

    switch (rep) {
        case GMRepetition::FIXED:
            current_frame = 0;
        break;
        case GMRepetition::FORWARD:
            current_frame = 0;
        break;
        case GMRepetition::BACKWARD:
            current_frame = last_frame;
        break;
        case GMRepetition::FORWARD_LOOP:
            current_frame = 0;
        break;
        case GMRepetition::BACKWARD_LOOP:
            current_frame = last_frame;
        break;
        case GMRepetition::PINGPONG_F:
            current_frame = 0;
        break;
        case GMRepetition::PINGPONG_B:
            current_frame = last_frame;
        break;
    }

    repetition = rep;
    active = true;
    gm_set_timer_duration();
}

uint16_t GMAnimation::gm_get_frame_index() {
    return frames[current_frame].first;
}

void GMAnimation::gm_set_timer_duration() {
    [[maybe_unused]] uint16_t new_duration = frames[current_frame].second;
    // timer.set_duration_restart(new_duration);
}

}
