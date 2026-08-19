/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the resource manager
*/

#ifndef FILE_GM_RESOURCES_HPP_INCLUDED
#define FILE_GM_RESOURCES_HPP_INCLUDED

// STD includes:
//#include <cstdint>
//#include <string_view>
#include <vector>
#include <memory>
#include <filesystem>
//#include <utility>

// Local includes:
#include "gm_texture.hpp"
#include "gm_bitmapfont.hpp"

namespace gm2d {
class GMResourceManager {
    public:
        GMResourceManager();

        void gm_load_resources(std::filesystem::path);

    private:
};
}

#endif // FILE_GM_RESOURCES_HPP_INCLUDED
