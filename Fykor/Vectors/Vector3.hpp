#pragma once

#include <cmath>
#include <ostream>

namespace Fykor::Vectors {
    class Vector3 {
    public:
        double x = 0.0;
        double y = 0.0;
        double z = 0.0;

        Vector3() = default;
        constexpr Vector3(double x, double y, double z) noexcept
            : x(x), y(y), z(z) {}

        static inline Vector3 Zero() noexcept { return {0.0, 0.0, 0.0}; }
        static inline Vector3 Forward() noexcept { return {0.0, 1.0, 0.0}; }
        static inline Vector3 Back() noexcept { return {0.0, -1.0, 0.0}; }
        static inline Vector3 Right() noexcept { return {1.0, 0.0, 0.0}; }
        static inline Vector3 Left() noexcept { return {-1.0, 0.0, 0.0}; }
        static inline Vector3 Up() noexcept { return {0.0, 0.0, 1.0}; }
        static inline Vector3 Down() noexcept { return {0.0, 0.0, -1.0}; }

        static constexpr double EPS = 1e-9;
        inline bool isNearlyEqual(const Vector3& other, double eps = EPS) const noexcept {
            return std::abs(x - other.x) < eps &&
                   std::abs(y - other.y) < eps &&
                   std::abs(z - other.z) < eps;
        }

        inline double Length() const noexcept {
            return std::sqrt(x * x + y * y + z * z);
        }

        inline double LengthSquared() const noexcept {
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

        inline double Dot(const Vector3& other) const noexcept {
            return x * other.x + y * other.y + z * other.z;
        }

        inline Vector3 Cross(const Vector3& other) const noexcept {
            return Vector3(
                y * other.z - z * other.y,
                z * other.x - x * other.z,
                x * other.y - y * other.x);
        }

        friend std::ostream& operator<<(std::ostream& os, const Vector3& v) {
            return os << "(" << v.x << ", " << v.y << ", " << v.z << ")";
        }
        // ========================================
        // Operators
        // ========================================

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
        inline Vector3 operator*=(const Vector3& other) noexcept {
            x *= other.x;
            y *= other.y;
            z *= other.z;
            return *this;
        }
        // === Division ===
        inline Vector3 operator/(const Vector3& other) const noexcept {
            return Vector3(x / other.x, y / other.y, z / other.z);
        }
        inline Vector3 operator/=(const Vector3& other) noexcept {
            x /= other.x;
            y /= other.y;
            z /= other.z;
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
            return std::abs(x - other.x) < EPS &&
                   std::abs(y - other.y) < EPS &&
                   std::abs(z - other.z) < EPS;
        }
        inline bool operator!=(const Vector3& other) const noexcept {
            return !(*this == other);
        }
        Vector3 operator-() const noexcept {
            return Vector3(-x, -y, -z);
        }
        // Scalar

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
    };

    inline constexpr Vector3 operator*(double scalar, const Vector3& v) noexcept {
        return v * scalar;
    }
} // namespace Fykor::Vectors