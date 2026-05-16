/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the object class
*/

#ifndef FILE_GM_OBJECT_HPP_INCLUDED
#define FILE_GM_OBJECT_HPP_INCLUDED

// STD includes:
#include <string>
#include <cstdint>
#include <string_view>

// Local includes:
#include "gm_message.hpp"

class GMObject {
    public:
        // Constructor:
        GMObject(std::string_view, bool, int16_t);
        virtual ~GMObject() = default;

        [[nodiscard]] virtual GMHandleResult handle_message(const GMMessage &);


        const std::string obj_name;
        bool obj_active;
        int16_t obj_update_order;
        // groups

};

class GMGFXObject: public GMObject {
    public:
        // Constructor:
        GMGFXObject(std::string_view, bool, int16_t);

        [[nodiscard]] GMHandleResult handle_message(const GMMessage &) override;


        bool gfx_visible;
        int16_t gfx_draw_order;
};

class GMObjectManager {

};

#endif // FILE_GM_OBJECT_HPP_INCLUDED
