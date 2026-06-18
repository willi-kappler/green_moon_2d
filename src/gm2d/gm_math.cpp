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
    return static_cast<std::float32_t>(std::hypot(x, y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_len2() {
    return static_cast<std::float32_t>(std::pow(x, 2) + std::pow(y, 2));
}

[[nodiscard]] GMVec2D GMVec2D::gm_norm1() {
    GMVec2D result = GMVec2D(x, y);
    result.gm_norm2();

    return result;
}

void GMVec2D::gm_norm2() {
    std::float32_t l = static_cast<std::float32_t>(std::hypot(x, y));
    x = x / l;
    y = y / l;
}

[[nodiscard]] std::float32_t GMVec2D::gm_dist(const GMVec2D &other) {
    return static_cast<std::float32_t>(std::hypot(x - other.x, y - other.y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_angle() {
    std::float32_t rad = std::atan2(x, y);
    std::float32_t deg = static_cast<std::float32_t>(rad * 180.0f32 / M_PI);

    if (deg < 0.0) {
        deg += 360.0f32;
    }

    return deg;
}

[[nodiscard]] std::float32_t GMVec2D::gm_cross(const GMVec2D &v) {
    return x * v.y - y * v.x;
}

[[nodiscard]] GMVec2D GMVec2D::gm_rotate1(std::float32_t a) {
    GMVec2D result = GMVec2D(x, y);
    result.gm_rotate2(a);

    return result;
}

void GMVec2D::gm_rotate2(std::float32_t a) {
    std::float32_t rad = static_cast<std::float32_t>(std::numbers::pi * a / 180.0);
    std::float32_t x2, y2;

    x2 = static_cast<std::float32_t>(x * std::cos(rad) - y * std::sin(rad));
    y2 = static_cast<std::float32_t>(x * std::sin(rad) + y * std::cos(rad));

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

// GMLine:
GMLine::GMLine():
    v1(),
    v2()
{}

GMLine::GMLine(const std::float32_t x1, const std::float32_t y1, const std::float32_t x2, const std::float32_t y2):
    v1(x1, y1),
    v2(x2, y2)
{}

GMLine::GMLine(const GMVec2D &, const GMVec2D &):
    v1(),
    v2()
{}

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

/*
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

[[nodiscard]] std::vector<GMVec2D> GMCircle::gm_collides_with_circle3(const GMCircle &c) {
    std::vector<GMVec2D> intersections;

    // Calculate distance between centers:
    std::float32_t dx = ctr.x - c.ctr.x;
    std::float32_t dy = ctr.y - c.ctr.y;
    std::float32_t d = std::hypot(dx, dy);

    // No intersection points:
    if (d > r + c.r) {
        return intersections;
    } else if (d < std::abs(r - c.r)) {
        return intersections;
    } else if (d == 0 && r == c.r) {
        return intersections;
    }

    // Unit vector in direction of center points:
    GMVec2D uv1 = GMVec2D(dx / d, dy / d);
    // Perpendicular unit vector:
    GMVec2D uv2 = GMVec2D(-dy / d, dx / d);

    std::float32_t a = (r * r  - c.r * c.r + d * d) / (2.0f32 * d);
    GMVec2D p1 = ctr + (uv1 * a);

    // One intersection point:
    if (approx(d, r + c.r) || approx(d, std::abs(r - c.r))) {
        intersections.push_back(p1);
        return intersections;
    }

    // Two intersection points:
    std::float32_t h = std::sqrt(r * r - a * a);
    GMVec2D p2 = p1 + (uv2 * h);
    intersections.push_back(p2);

    GMVec2D p3 = p1 - (uv2 * h);
    intersections.push_back(p3);

    return intersections;
}
*/

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

[[nodiscard]] GMVec2D GMRectangle::gm_min_point() const {
    return GMVec2D(std::min(v1.x, v2.x), std::min(v1.y, v2.y));
}

[[nodiscard]] GMVec2D GMRectangle::gm_max_point() const {
    return GMVec2D(std::max(v1.x, v2.x), std::max(v1.y, v2.y));
}

/*
[[nodiscard]] bool GMRectangle::gm_inside(const GMVec2D &v) {
    return ((v1.x - v.x) * (v2.x - v.x) < 0.0) && ((v1.y - v.y) * (v2.y - v.y) < 0.0);
}

[[nodiscard]] std::vector<GMVec2D> GMRectangle::gm_collides_with_rectangle(const GMRectangle &r) {
    std::vector<GMVec2D> intersections;

    // Get normalized min/max coordinates for both rectangles:
    GMVec2D p_min = gm_min_point();
    GMVec2D p_max = gm_max_point();
    GMVec2D r_p_min = r.gm_min_point();
    GMVec2D r_p_max = r.gm_max_point();

    // Find the overlapping region boundaries:
    std::float32_t inter_min_x = std::max(p_min.x, r_p_min.x);
    std::float32_t inter_min_y = std::max(p_min.y, r_p_min.y);
    std::float32_t inter_max_x = std::min(p_max.x, r_p_max.x);
    std::float32_t inter_max_y = std::min(p_max.y, r_p_max.y);

    // Check if an actual overlap exists.
    // If min is greater than max on either axis, they don't overlap:
    if (inter_min_x > inter_max_x || inter_min_y > inter_max_y) {
        return intersections;
    }

    // Return the two points defining the intersecting rectangle:
    intersections.push_back(GMVec2D(inter_min_x, inter_min_y));
    intersections.push_back(GMVec2D(inter_max_x, inter_max_y));

    return intersections;
}
*/

[[nodiscard]] bool approx(std::float32_t a, std::float32_t b) {
    return std::abs(a - b) <= GM_EPSILON;
}

[[nodiscard]] bool gm_is_on_segment(const GMVec2D &p, const GMVec2D &a, const GMVec2D &b) {
    return (p.x >= std::min(a.x, b.x)) && (p.x <= std::max(a.x, b.x)) &&
        (p.y >= std::min(a.y, b.y)) && (p.y <= std::max(a.y, b.y));
}

[[nodiscard]] uint8_t gm_orientation(const GMVec2D &a, const GMVec2D &b, const GMVec2D &c) {
    std::float32_t v = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);

    if (std::abs(v < GM_EPSILON)) {
        return 0;
    }

    return (v > 0) ? 1 : 2;
}

[[nodiscard]] bool gm_intersect_line_point(const GMLine &line, const GMVec2D &point) {
    GMVec2D line_vec = GMVec2D(line.v1.x - line.v2.x, line.v1.y - line.v2.y);
    GMVec2D point_vec = GMVec2D(point.x - line.v1.x, point.y - line.v1.y);

    if (std::abs(line_vec.gm_cross(point_vec)) > GM_EPSILON) {
        return false;
    }

    return gm_is_on_segment(point, line.v1, line.v2);
}
/*
[[nodiscard]] bool gm_intersect_line_line1(const GMLine &, const GMLine &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_line_line2(const GMLine &, const GMLine &) {

}

[[nodiscard]] bool gm_intersect_circle_point(const GMCircle &, const GMVec2D &) {

}

[[nodiscard]] bool gm_intersect_circle_line1(const GMCircle &, const GMLine &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_line2(const GMCircle &, const GMLine &) {

}

[[nodiscard]] bool gm_intersect_circle_circle1(const GMCircle &, const GMCircle &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_circle2(const GMCircle &, const GMCircle &) {

}

[[nodiscard]] bool gm_intersect_circle_rectangle1(const GMCircle &, const GMRectangle &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_rectangle2(const GMCircle &, const GMRectangle &) {

}

[[nodiscard]] bool gm_intersect_rectangle_point(const GMRectangle &, const GMVec2D &) {

}

[[nodiscard]] bool gm_intersect_rectangle_line1(const GMRectangle &, const GMLine &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_line2(const GMRectangle &, const GMLine &) {

}

[[nodiscard]] bool gm_intersect_rectangle_rectangle1(const GMRectangle &, const GMRectangle &) {

}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_rectangle2(const GMRectangle &, const GMRectangle &) {

}
*/
}
