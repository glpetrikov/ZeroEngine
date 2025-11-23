#include "Vectors/Vector3.h"

using namespace Fykor::Vectors;
using namespace Fykor;

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
