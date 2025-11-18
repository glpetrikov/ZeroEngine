#pragma once

#include <cmath>
#include <ostream>

namespace Fykor::Vectors {
    class Vector2 {
    public:
        double x = 0.0;
        double y = 0.0;

        Vector2() = default;
        constexpr Vector2(double x, double y) : x(x), y(y) {}
    };
}