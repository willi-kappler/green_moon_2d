/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the bitmap text
*/

#ifndef FILE_GM_BITMAPTEXT_HPP_INCLUDED
#define FILE_GM_BITMAPTEXT_HPP_INCLUDED

// STD includes:
#include <string>
#include <stdfloat>
#include <memory>
#include <cstdint>

// Local includes:
#include "gm_bitmapfont.hpp"
#include "gm_context.hpp"
#include "gm_math.hpp"
#include "gm_object.hpp"

namespace gm2d {
class GMBitmapText: public GMObject {
    public:
        GMBitmapText(GMStringId, std::string, std::shared_ptr<GMBitmapFont>);

        void gm_handle_message(const GMObjectMessage &, GMContext &) override;
        void gm_update(GMContext &) override;
        void gm_draw(GMContext &) override;

        void gm_set_horizontal(bool);

    private:
        std::string bm_text;
        std::shared_ptr<GMBitmapFont> bm_font;
        bool bm_horizontal;
        bool bm_sine_effect;
        bool bm_shake_effect;
        bool bm_rotation_effect;

};
}

#endif // FILE_GM_BITMAPTEXT_HPP_INCLUDED
