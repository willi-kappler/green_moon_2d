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

// GMVec2D:
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

GMVec2D::GMVec2D(const std::pair<std::float32_t, std::float32_t> &p):
    x(p.first),
    y(p.second)
{}

GMVec2D::GMVec2D(const std::array<std::float32_t, 2> &a):
    x(a[0]),
    y(a[1])
{}


void GMVec2D::add(const GMVec2D other) {
    x += other.x;
    y += other.y;
}

void GMVec2D::add(const std::float32_t xx, const std::float32_t yy) {
    x += xx;
    y += yy;
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

[[nodiscard]] std::float32_t GMVec2D::dist(const GMVec2D &other) {
    return static_cast<std::float32_t>(hypot(x - other.x, y - other.y));
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

void GMVec2D::operator=(const GMVec2D other) {
    x = other.x;
    y = other.y;
}

void GMVec2D::operator+=(const GMVec2D other) {
    x += other.x;
    y += other.y;
}

void GMVec2D::operator-=(const GMVec2D other) {
    x -= other.x;
    y -= other.y;
}

[[nodiscard]] GMVec2D GMVec2D::operator*(const std::float32_t a) {
    return GMVec2D(x * a, y * a);
}

void GMVec2D::operator*=(const std::float32_t a) {
    x *= a;
    y *= a;
}

[[nodiscard]] bool operator==(const GMVec2D v1, const GMVec2D v2) {
    return (v1.x == v2.x) && (v1.y == v2.y);
}

[[nodiscard]] bool operator!=(const GMVec2D v1, const GMVec2D v2) {
    return (v1.x != v2.x) || (v1.y != v2.y);
}

[[nodiscard]] GMVec2D operator+(const GMVec2D v1, const GMVec2D v2) {
    return GMVec2D(v1.x + v2.x, v1.y + v2.y);
}

[[nodiscard]] GMVec2D operator-(const GMVec2D v1, const GMVec2D v2) {
    return GMVec2D(v1.x - v2.x, v1.y - v2.y);
}

// GMCircle:
GMCircle::GMCircle():
    cx(0.0f32),
    cy(0.0f32),
    r(0.0f32)
{}

GMCircle::GMCircle(const std::float32_t x, const std::float32_t y, const std::float32_t radius):
    cx(x),
    cy(y),
    r(radius)
{}

GMCircle::GMCircle(const GMVec2D &v, const std::float32_t radius):
    cx(v.x),
    cy(v.y),
    r(radius)
{}

GMCircle::GMCircle(const std::pair<std::float32_t, std::float32_t> &p, const std::float32_t radius):
    cx(p.first),
    cy(p.second),
    r(radius)
{}

GMCircle::GMCircle(const std::array<std::float32_t, 2> &a, const std::float32_t radius):
    cx(a[0]),
    cy(a[1]),
    r(radius)
{}

// GMRectangle:
GMRectangle::GMRectangle():
    x1(0.0f32),
    y1(0.0f32),
    x2(0.0f32),
    y2(0.0f32)
{}
GMRectangle::GMRectangle(const std::float32_t xx1, const std::float32_t yy1, const std::float32_t xx2, const std::float32_t yy2):
    x1(xx1),
    y1(yy1),
    x2(xx2),
    y2(yy2)
{}
GMRectangle::GMRectangle(const GMVec2D &v1, const GMVec2D &v2):
    x1(v1.x),
    y1(v1.y),
    x2(v2.x),
    y2(v2.y)
{}
GMRectangle::GMRectangle(const std::pair<std::float32_t, std::float32_t> &p1, const std::pair<std::float32_t, std::float32_t> &p2):
    x1(p1.first),
    y1(p1.second),
    x2(p2.first),
    y2(p2.second)
{}



[[nodiscard]] bool approx(std::float32_t a, std::float32_t b) {
    return std::abs(a - b) <= 0.00001f32;
}
}
