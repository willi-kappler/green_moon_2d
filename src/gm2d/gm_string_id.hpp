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
#include <format>

namespace gm2d {
struct GMStringId {
    uint32_t value;

    bool operator==(const GMStringId& other) const = default;

    explicit operator uint32_t() const {
        return value;
    }
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

template <>
struct std::formatter<gm2d::GMStringId> : std::formatter<std::string_view> {
    auto format(gm2d::GMStringId s, format_context& ctx) const {
        std::string name = std::format("{}", s.value);
        return std::formatter<std::string_view>::format(name, ctx);
    }
};

#endif // FILE_GM_STRING_ID_HPP_INCLUDED
