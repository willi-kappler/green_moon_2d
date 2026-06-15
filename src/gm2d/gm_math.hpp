/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines math functions and classes.
*/

#ifndef FILE_GM_MATH_HPP_INCLUDED
#define FILE_GM_MATH_HPP_INCLUDED

// STD includes:
#include <cstdint>
#include <stdfloat>
#include <utility>
#include <array>

namespace gm2d {
enum struct GMAlignment: uint8_t {
    TOP_LEFT = 0,
    TOP_CENTER,
    TOP_RIGHT,

    MID_LEFT,
    MID_CENTER,
    MID_RIGHT,

    BTM_LEFT,
    BTM_CENTER,
    BTM_RIGHT
};

enum struct GMRepetition: uint8_t {
    FIXED = 0,
    FORWARD,
    BACKWARD,
    FORWARD_LOOP,
    BACKWARD_LOOP,
    PINGPONG_F,
    PINGPONG_B
};

class GMVec2D {
    public:
        // Constructor:
        GMVec2D();
        GMVec2D(const std::float32_t);
        GMVec2D(const std::float32_t, const std::float32_t);
        GMVec2D(const GMVec2D &);
        GMVec2D(const std::pair<std::float32_t, std::float32_t> &);
        GMVec2D(const std::array<std::float32_t, 2> &);

        // Methods:
        void add(const GMVec2D);
        void add(const std::float32_t, const std::float32_t);
        void mul(const std::float32_t);

        [[nodiscard]] std::float32_t len1();
        [[nodiscard]] std::float32_t len2();

        [[nodiscard]] GMVec2D norm1();
        void norm2();
        [[nodiscard]] std::float32_t dist(const GMVec2D &);

        [[nodiscard]] GMVec2D rotate1(const std::float32_t);
        void rotate2(std::float32_t);

        // Operators:
        void operator=(const GMVec2D);
        void operator+=(const GMVec2D);
        void operator-=(const GMVec2D);

        [[nodiscard]] GMVec2D operator*(const std::float32_t);
        void operator*=(const std::float32_t);

        friend bool operator==(const GMVec2D, const GMVec2D);
        friend bool operator!=(const GMVec2D, const GMVec2D);
        friend GMVec2D operator+(const GMVec2D, const GMVec2D);
        friend GMVec2D operator-(const GMVec2D, const GMVec2D);

        // Members:
        std::float32_t x;
        std::float32_t y;
};

class GMCircle {
        // Constructor:
        GMCircle();
        GMCircle(const std::float32_t, const std::float32_t, const std::float32_t);
        GMCircle(const GMVec2D &, const std::float32_t);
        GMCircle(const std::pair<std::float32_t, std::float32_t> &, const std::float32_t);
        GMCircle(const std::array<std::float32_t, 2> &, const std::float32_t);

        // Methods:

        // Operators:

        // Members:
        std::float32_t cx;
        std::float32_t cy;
        std::float32_t r;
};

class GMRectangle {
        // Constructor:
        GMRectangle();
        GMRectangle(const std::float32_t, const std::float32_t, const std::float32_t, const std::float32_t);
        GMRectangle(const GMVec2D &, const GMVec2D &);
        GMRectangle(const std::pair<std::float32_t, std::float32_t> &, const std::pair<std::float32_t, std::float32_t> &);

        // Methods:

        // Operators:

        // Members:
        std::float32_t x1;
        std::float32_t y1;
        std::float32_t x2;
        std::float32_t y2;
};

[[nodiscard]] bool approx(std::float32_t, std::float32_t);

}
#endif // FILE_GM_MATH_HPP_INCLUDED
