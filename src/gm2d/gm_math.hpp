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
#include <utility>
#include <array>
#include <optional>
#include <vector>

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

const float GM_EPSILON = 0.00001f;

class GMVec2D {
    public:
        // Constructor:
        GMVec2D();
        GMVec2D(const float);
        GMVec2D(const double);
        GMVec2D(const float, const float);
        GMVec2D(const double, const double);
        GMVec2D(const GMVec2D &);
        GMVec2D(const std::pair<float, float> &);
        GMVec2D(const std::pair<double, double> &);
        GMVec2D(const std::array<float, 2> &);
        GMVec2D(const std::array<double, 2> &);

        // Methods:
        void gm_add(const GMVec2D);
        void gm_mul(const float);
        void gm_mul(const double);
        void gm_div(const float);
        void gm_div(const double);

        [[nodiscard]] float gm_len1() const;
        [[nodiscard]] float gm_len2() const;

        [[nodiscard]] GMVec2D gm_norm1() const;
        void gm_norm2();
        [[nodiscard]] float gm_dist1(const GMVec2D &) const;
        [[nodiscard]] float gm_dist2(const GMVec2D &) const;
        [[nodiscard]] float gm_angle() const;
        [[nodiscard]] float gm_cross(const GMVec2D &) const;

        [[nodiscard]] GMVec2D gm_rotate1(const float) const;
        [[nodiscard]] GMVec2D gm_rotate1(const double) const;
        void gm_rotate2(float);
        void gm_rotate2(double);

        // Operators:
        void operator=(const GMVec2D);
        void operator+=(const GMVec2D);
        void operator-=(const GMVec2D);

        [[nodiscard]] GMVec2D operator*(const float) const;
        [[nodiscard]] GMVec2D operator*(const double) const;
        void operator*=(const float);
        void operator*=(const double);

        friend bool operator==(const GMVec2D, const GMVec2D);
        friend bool operator!=(const GMVec2D, const GMVec2D);
        friend GMVec2D operator+(const GMVec2D, const GMVec2D);
        friend GMVec2D operator-(const GMVec2D, const GMVec2D);

        // Members:
        float x;
        float y;
};

class GMLine {
    public:
        // Constructor:
        GMLine();
        GMLine(const float, const float, const float, const float);
        GMLine(const double, const double, const double, const double);
        GMLine(const GMVec2D &, const GMVec2D &);

        // Methods:
        [[nodiscard]] float gm_len1() const;
        [[nodiscard]] float gm_len2() const;
        [[nodiscard]] float gm_angle() const;
        void gm_rotate(float);
        void gm_rotate(double);
        void gm_scale(float);
        void gm_scale(double);

        // Operators:
        bool operator==(const GMLine &);
        bool operator!=(const GMLine &);

        // Members:
        GMVec2D v1;
        GMVec2D v2;
};

class GMCircle {
    public:
        // Constructor:
        GMCircle();
        GMCircle(const float, const float, const float);
        GMCircle(const double, const double, const double);
        GMCircle(const GMVec2D &, const float);
        GMCircle(const GMVec2D &, const double);

        // Methods:
        void gm_scale(float);
        void gm_scale(double);

        // Operators:
        bool operator==(const GMCircle &);
        bool operator!=(const GMCircle &);

        // Members:
        GMVec2D ctr;
        float r;
};

class GMRectangle {
    public:
        // Constructor:
        GMRectangle();
        GMRectangle(const float, const float, const float, const float);
        GMRectangle(const double, const double, const double, const double);
        GMRectangle(const GMVec2D &, const float, const float);
        GMRectangle(const GMVec2D &, const double, const double);
        GMRectangle(const GMVec2D &, const GMVec2D &);

        // Methods:
        [[nodiscard]] float gm_diagonal1() const;
        [[nodiscard]] float gm_diagonal2() const;
        [[nodiscard]] GMVec2D gm_opposite() const;
        void gm_scale(float);
        void gm_scale(double);

        // Operators:
        bool operator==(const GMRectangle &);
        bool operator!=(const GMRectangle &);

        // Members:
        GMVec2D v;
        float w;
        float h;
};


// Helper functions:
[[nodiscard]] bool gm_approx(float, float);
[[nodiscard]] bool gm_approx(float, double);
[[nodiscard]] bool gm_approx(double, float);
[[nodiscard]] bool gm_approx(double, double);
[[nodiscard]] bool gm_approx(const GMVec2D &, const GMVec2D &);
[[nodiscard]] bool gm_approx(const GMVec2D &, float, float);
[[nodiscard]] bool gm_approx(const GMVec2D &, double, double);

[[nodiscard]] bool gm_is_on_segment(const GMVec2D &, const GMVec2D &, const GMVec2D &);
[[nodiscard]] uint8_t gm_orientation(const GMVec2D &, const GMVec2D &, const GMVec2D &);
[[nodiscard]] bool gm_between(float, float, float);
[[nodiscard]] bool gm_between(double, double, double);
[[nodiscard]] GMVec2D gm_closest_point_on_segment(const GMLine&, const GMVec2D&);

[[nodiscard]] bool gm_intersect_line_point(const GMLine &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_line_line1(const GMLine &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_line_line2(const GMLine &, const GMLine &);
[[nodiscard]] bool gm_intersect_circle_point(const GMCircle &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_circle_line1(const GMCircle &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_line2(const GMCircle &, const GMLine &);
[[nodiscard]] bool gm_intersect_circle_circle1(const GMCircle &, const GMCircle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_circle2(const GMCircle &, const GMCircle &);
[[nodiscard]] bool gm_intersect_circle_rectangle1(const GMCircle &, const GMRectangle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_circle_rectangle2(const GMCircle &, const GMRectangle &);
[[nodiscard]] bool gm_intersect_rectangle_point(const GMRectangle &, const GMVec2D &);
[[nodiscard]] bool gm_intersect_rectangle_line1(const GMRectangle &, const GMLine &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_line2(const GMRectangle &, const GMLine &);
[[nodiscard]] bool gm_intersect_rectangle_rectangle1(const GMRectangle &, const GMRectangle &);
[[nodiscard]] std::vector<GMVec2D> gm_intersect_rectangle_rectangle2(const GMRectangle &, const GMRectangle &);


}
#endif // FILE_GM_MATH_HPP_INCLUDED
