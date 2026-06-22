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
#include <algorithm>

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

void GMVec2D::gm_div(std::float32_t a) {
    x /= a;
    y /= a;
}

[[nodiscard]] std::float32_t GMVec2D::gm_len1() const {
    return static_cast<std::float32_t>(std::hypot(x, y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_len2() const {
    return (x * x) + (y * y);
}

[[nodiscard]] GMVec2D GMVec2D::gm_norm1() const {
    GMVec2D result = GMVec2D(x, y);
    result.gm_norm2();

    return result;
}

void GMVec2D::gm_norm2() {
    std::float32_t l = static_cast<std::float32_t>(std::hypot(x, y));
    x = x / l;
    y = y / l;
}

[[nodiscard]] std::float32_t GMVec2D::gm_dist1(const GMVec2D &other) const {
    return static_cast<std::float32_t>(std::hypot(x - other.x, y - other.y));
}

[[nodiscard]] std::float32_t GMVec2D::gm_dist2(const GMVec2D &other) const {
    std::float32_t dx = x - other.x;
    std::float32_t dy = y - other.y;
    return (dx * dx) + (dy * dy);
}

[[nodiscard]] std::float32_t GMVec2D::gm_angle() const {
    std::float32_t rad = std::atan2(x, y);
    std::float32_t deg = static_cast<std::float32_t>(90.0 - (rad * 180.0 / M_PI));

    if (deg < 0.0 - GM_EPSILON) {
        deg += 360.0f32;
    }

    return deg;
}

[[nodiscard]] std::float32_t GMVec2D::gm_cross(const GMVec2D &v) const {
    return x * v.y - y * v.x;
}

[[nodiscard]] GMVec2D GMVec2D::gm_rotate1(std::float32_t a) const {
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

[[nodiscard]] GMVec2D GMVec2D::operator*(const std::float32_t a) const {
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

[[nodiscard]] std::float32_t GMLine::gm_len1() const {
    return v1.gm_dist1(v2);
}

[[nodiscard]] std::float32_t GMLine::gm_len2() const {
    return v1.gm_dist2(v2);
}

[[nodiscard]] std::float32_t GMLine::gm_angle() const {
    GMVec2D line_dir = v2 - v1;
    return line_dir.gm_angle();
}

void GMLine::gm_rotate(std::float32_t angle) {
    GMVec2D line_dir = v2 - v1;
    line_dir.gm_rotate2(angle);
    v2 = v1 + line_dir;
}

void GMLine::gm_scale(std::float32_t s) {
    GMVec2D line_dir = v2 - v1;
    line_dir *= s;
    v2 = v1 + line_dir;
}

bool GMLine::operator==(const GMLine &l) {
    return gm_approx(v1, l.v1) && gm_approx(v2, l.v2);
}

bool GMLine::operator!=(const GMLine &l) {
    return (v1 != l.v1) || (v2 != l.v2);
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

void GMCircle::gm_scale(std::float32_t s) {
    r *= s;
}

bool GMCircle::operator==(const GMCircle &c) {
    return gm_approx(r, c.r) && gm_approx(ctr, c.ctr);
}

bool GMCircle::operator!=(const GMCircle &c) {
    return (r != c.r) || (ctr != c.ctr);
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

[[nodiscard]] std::float32_t GMRectangle::gm_width() const {
    return abs(v1.x - v2.x);
}

[[nodiscard]] std::float32_t GMRectangle::gm_height() const {
    return abs(v1.y - v2.y);
}

[[nodiscard]] std::float32_t GMRectangle::gm_diagonal1() const {
    GMVec2D diagonal = v2 - v1;
    return diagonal.gm_len1();
}

[[nodiscard]] std::float32_t GMRectangle::gm_diagonal2() const {
    GMVec2D diagonal = v2 - v1;
    return diagonal.gm_len2();
}

[[nodiscard]] GMVec2D GMRectangle::gm_min_point() const {
    return GMVec2D(std::min(v1.x, v2.x), std::min(v1.y, v2.y));
}

[[nodiscard]] GMVec2D GMRectangle::gm_max_point() const {
    return GMVec2D(std::max(v1.x, v2.x), std::max(v1.y, v2.y));
}

void GMRectangle::gm_scale(std::float32_t s) {
    GMVec2D diagonal = v2 - v1;
    diagonal *= s;
    v2 = v1 + diagonal;
}

bool GMRectangle::operator==(const GMRectangle &r) {
    return gm_approx(v1, r.v1) && gm_approx(v2, r.v2);
}

bool GMRectangle::operator!=(const GMRectangle &r) {
    return (v1 != r.v1) || (v2 != r.v2);
}


// Helper functions:
[[nodiscard]] bool gm_approx(std::float32_t a, std::float32_t b) {
    return std::abs(a - b) <= GM_EPSILON;
}

[[nodiscard]] bool gm_approx(const GMVec2D &v1, const GMVec2D &v2) {
    return gm_approx(v1.x, v2.x) && gm_approx(v1.y, v2.y);
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

[[nodiscard]] bool gm_between(std::float32_t val, std::float32_t low, std::float32_t high) {
    return (low <= val) && (val <= high);
}

[[nodiscard]] GMVec2D gm_closest_point_on_segment(const GMLine& line, const GMVec2D& point) {
    const GMVec2D ab = line.v2 - line.v1;
    const GMVec2D ac = point - line.v1;

    const std::float32_t ab_len2 = ab.gm_len2();

    if (ab_len2 < GM_EPSILON) {
        return line.v1;
    }

    std::float32_t t = (ac.x * ab.x + ac.y * ab.y) / ab_len2;
    t = std::clamp(t, 0.0f32, 1.0f32);

    return line.v1 + ab * t;
}

[[nodiscard]] bool gm_intersect_line_point(const GMLine &line, const GMVec2D &point) {
    const GMVec2D line_vec = GMVec2D(line.v1.x - line.v2.x, line.v1.y - line.v2.y);
    const GMVec2D point_vec = GMVec2D(point.x - line.v1.x, point.y - line.v1.y);

    if (std::abs(line_vec.gm_cross(point_vec)) > GM_EPSILON) {
        return false;
    }

    return gm_is_on_segment(point, line.v1, line.v2);
}

[[nodiscard]] bool gm_intersect_line_line1(const GMLine &l1, const GMLine &l2) {
    const uint8_t o1 = gm_orientation(l1.v1, l1.v2, l2.v1);
    const uint8_t o2 = gm_orientation(l1.v1, l1.v2, l2.v2);
    const uint8_t o3 = gm_orientation(l2.v1, l2.v2, l1.v1);
    const uint8_t o4 = gm_orientation(l2.v1, l2.v2, l1.v2);

    if (o1 != o2 && o3 != o4) {
        return true;
    }

    if (o1 == 0 && gm_is_on_segment(l2.v1, l1.v1, l1.v2)) return true;
    if (o2 == 0 && gm_is_on_segment(l2.v2, l1.v1, l1.v2)) return true;
    if (o3 == 0 && gm_is_on_segment(l1.v1, l2.v1, l2.v2)) return true;
    if (o4 == 0 && gm_is_on_segment(l1.v2, l2.v1, l2.v2)) return true;

    return false;
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_line_line2(const GMLine &l1, const GMLine &l2) {
    std::vector<GMVec2D> intersections;

    const GMVec2D r = { l1.v2.x - l1.v1.x, l1.v2.y - l1.v1.y };
    const GMVec2D s = { l2.v2.x - l2.v1.x, l2.v2.y - l2.v1.y };
    const std::float32_t r_cross_s = r.gm_cross(s);
    const GMVec2D l2_minus_l1 = GMVec2D(l2.v1.x - l1.v1.x, l2.v1.y - l1.v1.y);
    const std::float32_t l2_minus_l1_cross_r = l2_minus_l1.gm_cross(r);

    if (std::abs(r_cross_s) < GM_EPSILON) {
        if (std::abs(l2_minus_l1_cross_r) < GM_EPSILON) {
            if (gm_is_on_segment(l2.v1, l1.v1, l1.v2)) intersections.push_back(l2.v1);
            if (gm_is_on_segment(l2.v2, l1.v1, l1.v2)) intersections.push_back(l2.v2);
            if (gm_is_on_segment(l1.v1, l2.v1, l2.v2)) intersections.push_back(l1.v1);
            if (gm_is_on_segment(l1.v2, l2.v1, l2.v2)) intersections.push_back(l1.v2);

            if (intersections.size() > 1) {
                if (gm_approx(intersections[0].x, intersections[1].x) && gm_approx(intersections[0].y, intersections[1].y)) {
                    intersections.pop_back();
                }
            }
        }

        return intersections;
    }

    const std::float32_t t = l2_minus_l1.gm_cross(s) / r_cross_s;
    const std::float32_t u = l2_minus_l1_cross_r / r_cross_s;

    if (gm_between(t, 0.0f32, 1.0f32) && gm_between(u, 0.0f32, 1.0f32)) {
        intersections.push_back(GMVec2D(l1.v1.x + t * r.x, l1.v1.y + t * r.y));
    }

    return intersections;
}

[[nodiscard]] bool gm_intersect_circle_point(const GMCircle &c, const GMVec2D &v) {
    return c.ctr.gm_dist1(v) <= c.r;
}


[[nodiscard]] bool gm_intersect_circle_line1(const GMCircle &c, const GMLine &l) {
    const GMVec2D closest = gm_closest_point_on_segment(l, c.ctr);

    const std::float32_t dist2 = c.ctr.gm_dist2(closest);
    return dist2 <= (c.r * c.r);
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_line2(const GMCircle &c, const GMLine &l) {
    std::vector<GMVec2D> intersections;

    const GMVec2D closest = gm_closest_point_on_segment(l, c.ctr);
    const std::float32_t dist2 = c.ctr.gm_dist2(closest);
    const std::float32_t radius2 = c.r * c.r;

    // Tangent intersection (1 point):
    if (gm_approx(dist2, radius2)) {
        intersections.push_back(closest);
        return intersections;
    }

    // No intersection:
    if (dist2 > radius2) {
        return intersections;
    }

    GMVec2D line_dir = l.v2 - l.v1;
    const std::float32_t line_len = line_dir.gm_len1();

    // Degraded line:
    if (line_len < GM_EPSILON) {
        return intersections;
    }

    // Normalize vector:
    line_dir.gm_div(line_len);

    const std::float32_t dt = std::sqrt(radius2 - dist2);

    // Calculate the two possible points on the infinite line:
    const GMVec2D p1 = closest + (line_dir * dt);
    const GMVec2D p2 = closest - (line_dir * dt);

    // Verify if the points actually lie on the finite line segment:
    if (gm_is_on_segment(l.v1, p1, l.v2)) {
        intersections.push_back(p1);
    }
    if (gm_is_on_segment(l.v1, p2, l.v2)) {
        // Prevent duplicate entry if p1 and p2 happen to crash into the same boundary due to precision:
        if (intersections.empty() || p1 != p2) {
            intersections.push_back(p2);
        }
    }

    return intersections;
}

[[nodiscard]] bool gm_intersect_circle_circle1(const GMCircle &c1, const GMCircle &c2) {
    return c1.ctr.gm_dist1(c2.ctr) <= c1.r + c2.r;
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_circle2(const GMCircle &c1, const GMCircle &c2) {
    std::vector<GMVec2D> intersections;

    // Calculate distance between centers:
    const std::float32_t dx = c1.ctr.x - c2.ctr.x;
    const std::float32_t dy = c1.ctr.y - c2.ctr.y;
    const std::float32_t d = std::hypot(dx, dy);

    // No intersection points:
    if (d > c1.r + c2.r) {
        return intersections;
    } else if (d < std::abs(c1.r - c2.r)) {
        return intersections;
    } else if (d == 0 && c1.r == c2.r) {
        return intersections;
    }

    // Unit vector in direction of center points:
    const GMVec2D uv1 = GMVec2D(dx / d, dy / d);
    // Perpendicular unit vector:
    const GMVec2D uv2 = GMVec2D(-dy / d, dx / d);

    const std::float32_t a = (c1.r * c1.r  - c2.r * c2.r + d * d) / (2.0f32 * d);
    const GMVec2D p1 = c1.ctr + (uv1 * a);

    // One intersection point:
    if (gm_approx(d, c1.r + c2.r) || gm_approx(d, std::abs(c1.r - c2.r))) {
        intersections.push_back(p1);
        return intersections;
    }

    // Two intersection points:
    const std::float32_t h = std::sqrt(c1.r * c1.r - a * a);
    const GMVec2D p2 = p1 + (uv2 * h);
    intersections.push_back(p2);

    const GMVec2D p3 = p1 - (uv2 * h);
    intersections.push_back(p3);

    return intersections;
}


[[nodiscard]] bool gm_intersect_circle_rectangle1(const GMCircle &c, const GMRectangle &r) {
    const GMVec2D min_p = r.gm_min_point();
    const GMVec2D max_p = r.gm_max_point();

    // Clamp the circle's center to the rectangle's boundaries to find the closest point:
    const std::float32_t closest_x = std::clamp(c.ctr.x, min_p.x, max_p.x);
    const std::float32_t closest_y = std::clamp(c.ctr.y, min_p.y, max_p.y);
    const GMVec2D closest_point(closest_x, closest_y);

    // If the squared distance to the closest point is <= radius^2, they intersect.
    // Note: This naturally handles the case where the circle center is fully inside the rectangle!
    return c.ctr.gm_dist2(closest_point) <= (c.r * c.r);
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_rectangle2(const GMCircle &c, const GMRectangle &r) {
    std::vector<GMVec2D> intersections;
    intersections.reserve(8); // A circle can intersect a rectangle at most 8 times

    const GMVec2D min_p = r.gm_min_point();
    const GMVec2D max_p = r.gm_max_point();

    // Define the 4 corners of the rectangle:
    const GMVec2D top_left(min_p.x, max_p.y);
    const GMVec2D top_right(max_p.x, max_p.y);
    const GMVec2D bottom_right(max_p.x, min_p.y);
    const GMVec2D bottom_left(min_p.x, min_p.y);

    // Create the 4 bounding line segments:
    const std::array<GMLine, 4> edges = {
        GMLine(top_left, top_right),     // Top
        GMLine(top_right, bottom_right), // Right
        GMLine(bottom_right, bottom_left),// Bottom
        GMLine(bottom_left, top_left)    // Left
    };

    // Helper to prevent duplicate points (e.g., if a circle intersects exactly on a corner)
    auto add_unique_points = [&](const std::vector<GMVec2D>& points) {
        for (const auto& p : points) {
            bool duplicate = false;

            for (const auto& existing: intersections) {
                if (p.gm_dist2(existing) < GM_EPSILON) {
                    duplicate = true;
                    break;
                }
            }

            if (!duplicate) {
                intersections.push_back(p);
            }
        }
    };

    // Check intersections against all 4 edges:
    for (const auto& edge: edges) {
        std::vector<GMVec2D> line_intersections = gm_intersect_circle_line2(c, edge);
        if (!line_intersections.empty()) {
            add_unique_points(line_intersections);
        }
    }

    return intersections;
}

[[nodiscard]] bool gm_intersect_rectangle_point(const GMRectangle &r, const GMVec2D &v) {
    return ((r.v1.x - v.x) * (r.v2.x - v.x) < 0.0) && ((r.v1.y - v.y) * (r.v2.y - v.y) < 0.0);
}


[[nodiscard]] bool gm_intersect_rectangle_line1(const GMRectangle &r, const GMLine &l) {
    if (gm_intersect_rectangle_point(r, l.v1) || gm_intersect_rectangle_point(r, l.v2)) {
        return true;
    }

    const GMVec2D min_p = r.gm_min_point();
    const GMVec2D max_p = r.gm_max_point();

    // Define the 4 edges of the rectangle:
    const GMVec2D top_left(min_p.x, max_p.y);
    const GMVec2D top_right(max_p.x, max_p.y);
    const GMVec2D bottom_right(max_p.x, min_p.y);
    const GMVec2D bottom_left(min_p.x, min_p.y);

    const std::array<GMLine, 4> edges = {
        GMLine(top_left, top_right),
        GMLine(top_right, bottom_right),
        GMLine(bottom_right, bottom_left),
        GMLine(bottom_left, top_left)
    };

    // Check if the line intersects any of the edges using your existing helper:
    for (const auto& edge: edges) {
        if (gm_intersect_line_line1(edge, l)) {
            return true;
        }
    }

    return false;
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_line2(const GMRectangle &r, const GMLine &l) {
    std::vector<GMVec2D> intersections;
    const GMVec2D min_p = r.gm_min_point();
    const GMVec2D max_p = r.gm_max_point();

    const GMVec2D top_left(min_p.x, max_p.y);
    const GMVec2D top_right(max_p.x, max_p.y);
    const GMVec2D bottom_right(max_p.x, min_p.y);
    const GMVec2D bottom_left(min_p.x, min_p.y);

    const std::array<GMLine, 4> edges = {
        GMLine(top_left, top_right),
        GMLine(top_right, bottom_right),
        GMLine(bottom_right, bottom_left),
        GMLine(bottom_left, top_left)
    };

    for (const auto& edge : edges) {
        std::vector<GMVec2D> points = gm_intersect_line_line2(edge, l);
        for (const auto& p : points) {
            bool duplicate = false;
            for (const auto& existing : intersections) {
                if (p.gm_dist2(existing) < GM_EPSILON * GM_EPSILON) {
                    duplicate = true;
                    break;
                }
            }
            if (!duplicate) {
                intersections.push_back(p);
            }
        }
    }

    return intersections;
}

[[nodiscard]] bool gm_intersect_rectangle_rectangle1(const GMRectangle &r1, const GMRectangle &r2) {
    const GMVec2D min1 = r1.gm_min_point();
    const GMVec2D max1 = r1.gm_max_point();
    const GMVec2D min2 = r2.gm_min_point();
    const GMVec2D max2 = r2.gm_max_point();

    // Check X-axis overlap:
    const bool overlap_x = (max1.x >= min2.x - GM_EPSILON) && (min1.x <= max2.x + GM_EPSILON);

    // Check Y-axis overlap:
    const bool overlap_y = (max1.y >= min2.y - GM_EPSILON) && (min1.y <= max2.y + GM_EPSILON);

    // It is an intersection if they overlap on BOTH axes:
    return overlap_x && overlap_y;
}

[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_rectangle2(const GMRectangle &r1, const GMRectangle &r2) {
    std::vector<GMVec2D> intersections;

    // Get normalized min/max coordinates for both rectangles:
    const GMVec2D r1_min = r1.gm_min_point();
    const GMVec2D r1_max = r1.gm_max_point();
    const GMVec2D r2_min = r2.gm_min_point();
    const GMVec2D r2_max = r2.gm_max_point();

    // Find the overlapping region boundaries:
    const std::float32_t inter_min_x = std::max(r1_min.x, r2_min.x);
    const std::float32_t inter_min_y = std::max(r1_min.y, r2_min.y);
    const std::float32_t inter_max_x = std::min(r1_max.x, r2_max.x);
    const std::float32_t inter_max_y = std::min(r1_max.y, r2_max.y);

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

}
