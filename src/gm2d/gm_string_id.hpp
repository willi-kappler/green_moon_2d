/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines string id FNV-1a hashes at compile time.
*/

#ifndef FILE_GM_STRING_ID_HPP_INCLUDED
#define FILE_GM_STRING_ID_HPP_INCLUDED

#include <string_view>
#include <cstdint>

namespace gm2d {
struct GMStringId {
    uint32_t value;

    bool operator==(const GMStringId& other) const = default;
};

// The compile-time FNV-1a 32-bit hashing function
constexpr GMStringId GMID(std::string_view str) {
    uint32_t hash = 2166136261u; // FNV offset basis
    for (char c : str) {
        hash ^= static_cast<uint32_t>(c);
        hash *= 16777619u; // FNV prime
    }
    return GMStringId{ hash };
}
}

#endif // FILE_GM_STRING_ID_HPP_INCLUDED
