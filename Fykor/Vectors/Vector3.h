#pragma once

#include "../Common.h"

#include <cmath>
#include <ostream>

namespace Fykor::Vectors {

    template <typename T>

    class Vector3 {
    public:
        T x = T(0);
        T y = T(0);
        T z = T(0);

        Vector3() = default;
        constexpr Vector3(T x, T y, T z) noexcept
            : x(x), y(y), z(z) {}

        static inline Vector3 Zero() noexcept { return {T(0), T(0), T(0)}; }
        static inline Vector3 Forward() noexcept { return {T(0), T(1), T(0)}; }
        static inline Vector3 Back() noexcept { return {T(0), T(-1), T(0)}; }
        static inline Vector3 Right() noexcept { return {T(1), T(0), T(0)}; }
        static inline Vector3 Left() noexcept { return {T(-1), T(0), T(0)}; }
        static inline Vector3 Up() noexcept { return {T(0), T(0), T(1)}; }
        static inline Vector3 Down() noexcept { return {T(0), T(0), T(-1)}; }

        static constexpr T EPS = T(1e-9);
        inline bool IsNearlyEqual(const Vector3& other, double eps = EPS) const noexcept {
            return std::abs(x - other.x) < eps &&
                   std::abs(y - other.y) < eps &&
                   std::abs(z - other.z) < eps;
        }

        inline T Length() const noexcept {
            return std::sqrt(x * x + y * y + z * z);
        }

        inline T LengthSquared() const noexcept {
            return x * x + y * y + z * z;
        }

        inline Vector3 Normalized() const noexcept {
            double len = Length();
            return len > 0 ? *this / len : Vector3::Zero();
        }

        inline Vector3& Normalize() noexcept {
            double len = Length();
            if (len > 0)
                *this /= len;
            return *this;
        }

        inline T Dot(const Vector3& other) const noexcept {
            return x * other.x + y * other.y + z * other.z;
        }

        inline Vector3 Cross(const Vector3& other) const noexcept {
            return Vector3(
                y * other.z - z * other.y,
                z * other.x - x * other.z,
                x * other.y - y * other.x);
        }
        // ========================================
        // Operators
        // ========================================

        friend std::ostream& operator<<(std::ostream& os, const Vector3& v) {
            return os << v.x << ", " << v.y << ", " << v.z;
        }

        // === Plus ===
        inline Vector3 operator+(const Vector3& other) const noexcept {
            return Vector3(x + other.x, y + other.y, z + other.z);
        }
        inline Vector3& operator+=(const Vector3& other) noexcept {
            x += other.x;
            y += other.y;
            z += other.z;
            return *this;
        }

        // === Minus ===
        inline Vector3 operator-(const Vector3& other) const noexcept {
            return Vector3(x - other.x, y - other.y, z - other.z);
        }
        inline Vector3& operator-=(const Vector3& other) noexcept {
            x -= other.x;
            y -= other.y;
            z -= other.z;
            return *this;
        }

        // === Multiplication ===
        inline Vector3 operator*(const Vector3& other) const noexcept {
            return Vector3(x * other.x, y * other.y, z * other.z);
        }
        inline Vector3& operator*=(const Vector3& other) noexcept {
            x *= other.x;
            y *= other.y;
            z *= other.z;
            return *this;
        }
        // === Division ===
        inline Vector3 operator/(const Vector3& other) const noexcept {
            if (other.x == 0 && other.y == 0 && other.z == 0) {
                Debug::FykorLogger.ErrorLine("Division by zero vector!");
            }
            return Vector3(
                other.x != 0 ? x / other.x : 0,
                other.y != 0 ? y / other.y : 0,
                other.z != 0 ? z / other.z : 0);
        }
        inline Vector3 operator/=(const Vector3& other) noexcept {
            if (other.x == 0 && other.y == 0 && other.z == 0) {
                Debug::FykorLogger.ErrorLine("Division by zero vector!");
            }
            other.x != 0 ? x /= other.x : x;
            other.y != 0 ? y /= other.y : y;
            other.z != 0 ? z /= other.z : z;
            return *this;
        }

        // === Scalar ===

        inline Vector3 operator*(double scalar) const noexcept {
            return Vector3(x * scalar, y * scalar, z * scalar);
        }

        inline Vector3& operator*=(double scalar) noexcept {
            x *= scalar;
            y *= scalar;
            z *= scalar;
            return *this;
        }

        inline Vector3 operator/(double scalar) const noexcept {
            return Vector3(x / scalar, y / scalar, z / scalar);
        }

        inline Vector3& operator/=(double scalar) noexcept {
            x /= scalar;
            y /= scalar;
            z /= scalar;
            return *this;
        }

        // === Other ===

        inline Vector3& operator=(const Vector3& other) noexcept {
            if (this != &other) {
                x = other.x;
                y = other.y;
                z = other.z;
            }
            return *this;
        }
        inline bool operator==(const Vector3& other) const noexcept {
            constexpr double EPS = 1e-9;
            return IsNearlyEqual(other);
        }
        inline bool operator!=(const Vector3& other) const noexcept {
            return !(*this == other);
        }
        Vector3 operator-() const noexcept {
            return Vector3(-x, -y, -z);
        }
    };

    template <typename T>
    inline Vector3<T> operator*(double scalar, const Vector3<T>& v) noexcept {
        return v * scalar;
    }
} // namespace Fykor::Vectors