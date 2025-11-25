/* =================================================
* Fykor, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * FykorEngine
 * Trasnform.h
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.11.25
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Transform Component
 * =================================================
 */

#include "Fykor/Types.h"
#include "Fykor/Vectors/Vector3.h"

namespace Fy::Components {
    class Transform {
    public:
        Vector3<float> Position;
        Vector3<float> Rotation;
        Vector3<float> Scale;

        Transform() :
            Position(Vector3<float>::Zero()), Rotation(Vector3<float>::Zero()), Scale(Vector3<float>(1, 1, 1)) {}
    };
}
