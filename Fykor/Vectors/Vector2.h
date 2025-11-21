#pragma once

#include "../Common.h"

#include <cmath>
#include <ostream>

namespace Fykor::Vectors {

    template <typename T>

    class Vector2 {
    public:
        T x = 0.0;
        T y = 0.0;

        Vector2() = default;
        Vector2(T x, T y) : x(x), y(y) {}

        static inline Vector2 Zero() noexcept { return {0, 0}; }
        static inline Vector2 Forward() noexcept { return {0.0, 1.0}; }
        static inline Vector2 Back() noexcept { return {0.0, -1.0}; }
        static inline Vector2 Right() noexcept { return {1.0, 0.0}; }
        static inline Vector2 Left() noexcept { return {-1.0, 0.0}; }

        static constexpr T EPS = T(1e-9);

        inline bool IsNearlyEqual(const Vector2& other, double eps = EPS) const noexcept {
            return std::abs(x - other.x) < eps &&
                   std::abs(y - other.y) < eps;
        }

        inline T Length() const noexcept {
            return std::sqrt(x * x + y * y);
        }
        inline T LengthSquared() const noexcept {
            return x * x + y * y;
        }

        inline Vector2 Normalized() const noexcept {
            double len = Length();
            return len > 0 ? *this / len : Vector2::Zero();
        }

        inline Vector2& Normalize() noexcept {
            double len = Length();
            if (len > 0)
                *this /= len;
            return *this;
        }

        inline T Dot(const Vector2& other) const noexcept {
            return x * other.x + y * other.y;
        }

        // ========================================
        // Operators
        // ========================================

        friend std::ostream& operator<<(std::ostream& os, const Vector2& v) noexcept {
            return os << v.x << ", " << v.y;
        }

        // === Plus ===
        inline Vector2 operator+(const Vector2& other) const noexcept {
            return Vector2(x + other.x, y + other.y);
        }
        inline Vector2& operator+=(const Vector2& other) noexcept {
            x += other.x;
            y += other.y;
            return *this;
        }

        // === Minus ===
        inline Vector2 operator-(const Vector2& other) const noexcept {
            return Vector2(x - other.x, y - other.y);
        }
        inline Vector2& operator-=(const Vector2& other) noexcept {
            x -= other.x;
            y -= other.y;
            return *this;
        }

        // === Multiplication ===
        inline Vector2 operator*(const Vector2& other) const noexcept {
            return Vector2(x * other.x, y * other.y);
        }
        inline Vector2& operator*=(const Vector2& other) noexcept {
            x *= other.x;
            y *= other.y;
            return *this;
        }
        // === Division ===
        inline Vector2 operator/(const Vector2& other) const noexcept {
            if (other.x == 0 && other.y == 0) {
                Debug::FykorLogger.ErrorLine("Division by zero vector!");
            }
            return Vector2(
                other.x != 0 ? x / other.x : 0,
                other.y != 0 ? y / other.y : 0);
        }
        inline Vector2 operator/=(const Vector2& other) noexcept {
            if (other.x == 0 && other.y == 0) {
                Debug::FykorLogger.ErrorLine("Division by zero vector!");
            }
            other.x != 0 ? x /= other.x : x;
            other.y != 0 ? y /= other.y : y;
            return *this;
        }
        // === Scalar ===

        inline Vector2 operator*(double scalar) const noexcept {
            return Vector2(x * scalar, y * scalar);
        }
        inline Vector2& operator*=(double scalar) noexcept {
            x *= scalar;
            y *= scalar;
            return *this;
        }

        inline Vector2 operator/(double scalar) const noexcept {
            return Vector2(x / scalar, y / scalar);
        }

        inline Vector2& operator/=(double scalar) noexcept {
            x /= scalar;
            y /= scalar;
            return *this;
        }

        // === Other ===

        inline Vector2& operator=(const Vector2& other) noexcept {
            if (this != &other) {
                x = other.x;
                y = other.y;
            }
            return *this;
        }
        inline bool operator==(const Vector2& other) const noexcept {
            constexpr double EPS = 1e-9;
            return IsNearlyEqual(other);
        }
        inline bool operator!=(const Vector2& other) const noexcept {
            return !(*this == other);
        }
        Vector2 operator-() const noexcept {
            return Vector2(-x, -y);
        }
    };

    template <typename T>
    inline Vector2<T> operator*(double scalar, const Vector2<T>& v) noexcept {
        return Vector2<T>(v.x * scalar, v.y * scalar);
    }
}