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
#include <stdfloat>

namespace gm2d {
class GMVec2D {
    public:
        // Constructor:
        GMVec2D();
        GMVec2D(std::float32_t);
        GMVec2D(std::float32_t, std::float32_t);

        void add(std::float32_t);
        void add(std::float32_t, std::float32_t);
        void add(GMVec2D);

        void mul(std::float32_t);

        [[nodiscard]] std::float32_t len1();
        [[nodiscard]] std::float32_t len2();

        [[nodiscard]] GMVec2D norm1();
        void norm2();

        [[nodiscard]] GMVec2D rotate1(std::float32_t);
        void rotate2(std::float32_t);

        std::float32_t x;
        std::float32_t y;
};
}

// Operators:
bool operator==(const gm2d::GMVec2D&, const gm2d::GMVec2D&);
bool operator!=(const gm2d::GMVec2D&, const gm2d::GMVec2D&);

gm2d::GMVec2D operator+(const gm2d::GMVec2D&, const gm2d::GMVec2D&);
gm2d::GMVec2D operator+(const gm2d::GMVec2D&, const std::float32_t);
void operator+=(gm2d::GMVec2D&, const gm2d::GMVec2D&);
void operator+=(gm2d::GMVec2D&, const std::float32_t);

gm2d::GMVec2D operator-(const gm2d::GMVec2D&, const gm2d::GMVec2D&);
gm2d::GMVec2D operator-(const gm2d::GMVec2D&, const std::float32_t);
void operator-=(gm2d::GMVec2D&, const gm2d::GMVec2D&);
void operator-=(gm2d::GMVec2D&, const std::float32_t);

gm2d::GMVec2D operator*(const gm2d::GMVec2D&, const std::float32_t);
void operator*=(gm2d::GMVec2D&, const std::float32_t);


#endif // FILE_GM_MATH_HPP_INCLUDED
