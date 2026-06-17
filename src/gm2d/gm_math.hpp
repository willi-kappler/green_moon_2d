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
        void gm_add(const GMVec2D);
        void gm_mul(const std::float32_t);

        [[nodiscard]] std::float32_t gm_len1();
        [[nodiscard]] std::float32_t gm_len2();

        [[nodiscard]] GMVec2D gm_norm1();
        void gm_norm2();
        [[nodiscard]] std::float32_t gm_dist(const GMVec2D &);

        [[nodiscard]] GMVec2D gm_rotate1(const std::float32_t);
        void gm_rotate2(std::float32_t);

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

        // Methods:
        [[nodiscard]] bool gm_inside(const GMVec2D &);
        [[nodiscard]] bool gm_collides_with_circle1(const GMCircle &);
        [[nodiscard]] GMVec2D gm_collides_with_circle2(const GMCircle &);

        // Operators:

        // Members:
        GMVec2D ctr;
        std::float32_t r;
};

class GMRectangle {
        // Constructor:
        GMRectangle();
        GMRectangle(const std::float32_t, const std::float32_t, const std::float32_t, const std::float32_t);
        GMRectangle(const GMVec2D &, const GMVec2D &);

        // Methods:
        [[nodiscard]] bool gm_inside(const GMVec2D &);
        [[nodiscard]] std::float32_t gm_width();
        [[nodiscard]] std::float32_t gm_height();
        [[nodiscard]] bool gm_collides_with_rectangle1(const GMRectangle &);

        // Operators:

        // Members:
        GMVec2D v1;
        GMVec2D v2;
};

[[nodiscard]] bool approx(std::float32_t, std::float32_t);

}
#endif // FILE_GM_MATH_HPP_INCLUDED
