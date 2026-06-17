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


void GMVec2D::gm_add(const GMVec2D other) {
    x += other.x;
    y += other.y;
}

void GMVec2D::gm_mul(std::float32_t a) {
    x *= a;
    y *= a;
}

[[nodiscard]] std::float32_t GMVec2D::gm_len1() {
    return static_cast<std::float32_t>(hypot(x, y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_len2() {
    return static_cast<std::float32_t>(pow(x, 2) + pow(y, 2));
}

[[nodiscard]] GMVec2D GMVec2D::gm_norm1() {
    GMVec2D result = GMVec2D(x, y);
    result.gm_norm2();

    return result;
}

void GMVec2D::gm_norm2() {
    std::float32_t l = static_cast<std::float32_t>(hypot(x, y));
    x = x / l;
    y = y / l;
}

[[nodiscard]] std::float32_t GMVec2D::gm_dist(const GMVec2D &other) {
    return static_cast<std::float32_t>(hypot(x - other.x, y - other.y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_angle() {
    std::float32_t rad = std::atan2(x, y);
    std::float32_t deg = static_cast<std::float32_t>(rad * 180.0f32 / M_PI);

    if (deg < 0.0) {
        deg += 360.0f32;
    }

    return deg;
}

[[nodiscard]] GMVec2D GMVec2D::gm_rotate1(std::float32_t a) {
    GMVec2D result = GMVec2D(x, y);
    result.gm_rotate2(a);

    return result;
}

void GMVec2D::gm_rotate2(std::float32_t a) {
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
    ctr(),
    r(0.0f32)
{}

GMCircle::GMCircle(const std::float32_t x, const std::float32_t y, const std::float32_t radius):
    ctr(x, y),
    r(radius)
{}

GMCircle::GMCircle(const GMVec2D &v, const std::float32_t radius):
    ctr(v),
    r(radius)
{}

[[nodiscard]] bool GMCircle::gm_inside(const GMVec2D &v) {
    return ctr.gm_dist(v) < r;
}

[[nodiscard]] bool GMCircle::gm_collides_with_circle1(const GMCircle &c) {
    return ctr.gm_dist(c.ctr) <= (r + c.r);
}

[[nodiscard]] std::optional<GMVec2D> GMCircle::gm_collides_with_circle2(const GMCircle &c) {
    GMVec2D v = c.ctr - ctr;
    std::float32_t d = v.gm_len1();

    if (d > r + c.r) {
        return {};
    } else {
        return v;
    }
}


// GMRectangle:
GMRectangle::GMRectangle():
    v1(),
    v2()
{}

GMRectangle::GMRectangle(const std::float32_t x1, const std::float32_t y1, const std::float32_t x2, const std::float32_t y2):
    v1(x1, y1),
    v2(x2, y2)
{}

GMRectangle::GMRectangle(const GMVec2D &u1, const GMVec2D &u2):
    v1(u1),
    v2(u2)
{}

[[nodiscard]] std::float32_t GMRectangle::gm_width() {
    return abs(v1.x - v2.x);
}

[[nodiscard]] std::float32_t GMRectangle::gm_height() {
    return abs(v1.y - v2.y);
}

[[nodiscard]] bool GMRectangle::gm_inside(const GMVec2D &v) {
    return ((v1.x - v.x) * (v2.x - v.x) < 0.0) && ((v1.y - v.y) * (v2.y - v.y) < 0.0);
}

[[nodiscard]] bool GMRectangle::gm_collides_with_rectangle1(const GMRectangle &r) {
    // TODO: finish / fix
    return gm_inside(r.v1) || gm_inside(r.v2);
}

[[nodiscard]] bool approx(std::float32_t a, std::float32_t b) {
    return std::abs(a - b) <= 0.00001f32;
}
}
