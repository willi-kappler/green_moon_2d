/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines math functions and classes.
*/

// STD include:
#include <cmath>
#include <numbers>

// Local includes:
#include "gm_math.hpp"

namespace gm2d {
GMVec2D::GMVec2D():
    x(0.0f32),
    y(0.0f32)
{}

GMVec2D::GMVec2D(std::float32_t a):
    x(a),
    y(a)
{}

GMVec2D::GMVec2D(std::float32_t a, std::float32_t b):
    x(a),
    y(b)
{}

GMVec2D::GMVec2D(const GMVec2D &other):
    x(other.x),
    y(other.y)
{}

void GMVec2D::add(std::float32_t a) {
    x += a;
    y += a;
}

void GMVec2D::add(std::float32_t a, std::float32_t b) {
    x += a;
    y += b;
}

void GMVec2D::add(const GMVec2D &other) {
    x += other.x;
    y += other.y;
}

void GMVec2D::mul(std::float32_t a) {
    x *= a;
    y *= a;
}

[[nodiscard]] std::float32_t GMVec2D::len1() {
    return static_cast<std::float32_t>(hypot(x, y));
}

[[nodiscard]] std::float32_t GMVec2D::len2() {
    return static_cast<std::float32_t>(pow(x, 2) + pow(y, 2));
}

[[nodiscard]] GMVec2D GMVec2D::norm1() {
    /*
    std::float32_t l = hypot(x, y);
    return GMVec2D(x / l, y / l);
    */

    GMVec2D result = GMVec2D(x, y);
    result.norm2();

    return result;
}

void GMVec2D::norm2() {
    std::float32_t l = static_cast<std::float32_t>(hypot(x, y));
    x = x / l;
    y = y / l;
}

[[nodiscard]] GMVec2D GMVec2D::rotate1(std::float32_t a) {
    /*
    std::float32_t rad = std::numbers::pi * a / 180.0;
    std::float32_t x2, y2;

    x2 = x * cos(rad) - y * sin(rad);
    y2 = x * sin(rad) + y * cos(rad);

    return GMVec2D(x2, y2);
    */

    GMVec2D result = GMVec2D(x, y);
    result.rotate2(a);

    return result;
}

void GMVec2D::rotate2(std::float32_t a) {
    std::float32_t rad = static_cast<std::float32_t>(std::numbers::pi * a / 180.0);
    std::float32_t x2, y2;

    x2 = static_cast<std::float32_t>(x * cos(rad) - y * sin(rad));
    y2 = static_cast<std::float32_t>(x * sin(rad) + y * cos(rad));

    x = x2;
    y = y2;
}

void GMVec2D::operator=(const GMVec2D &other) {
    x = other.x;
    y = other.y;
}

[[nodiscard]] bool GMVec2D::operator==(const GMVec2D &other) {
    return (x == other.x) && (y == other.y);
}

[[nodiscard]] bool GMVec2D::operator!=(const GMVec2D &other) {
    return (x != other.x) || (y != other.y);
}

[[nodiscard]] GMVec2D GMVec2D::operator+(const GMVec2D &other) {
    return GMVec2D(x + other.x, y + other.y);
}

[[nodiscard]] GMVec2D GMVec2D::operator+(const std::float32_t a) {
    return GMVec2D(x + a, y + a);
}

void GMVec2D::operator+=(const GMVec2D &other) {
    x += other.x;
    y += other.y;
}

void GMVec2D::operator+=(const std::float32_t a) {
    x += a;
    y += a;
}

[[nodiscard]] GMVec2D GMVec2D::operator-(const GMVec2D &other) {
    return GMVec2D(x - other.x, y - other.y);
}

[[nodiscard]] GMVec2D GMVec2D::operator-(const std::float32_t a) {
    return GMVec2D(x - a, y - a);
}

void GMVec2D::operator-=(const GMVec2D &other) {
    x -= other.x;
    y -= other.y;
}

void GMVec2D::operator-=(const std::float32_t a) {
    x -= a;
    y -= a;
}

[[nodiscard]] GMVec2D GMVec2D::operator*(const std::float32_t a) {
    return GMVec2D(x * a, y * a);
}

void GMVec2D::operator*=(const std::float32_t a) {
    x *= a;
    y *= a;
}

[[nodiscard]] bool approx(std::float32_t a, std::float32_t b) {
    return std::abs(a - b) <= 0.00001f32;
}

}
