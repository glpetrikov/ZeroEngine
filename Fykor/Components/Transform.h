#include "../Vectors/Vector3.h"

using namespace Fykor::Vectors;
using namespace Fykor;

namespace Fy::Components {
    class Transform {
    public:
        Vector3 Position;
        Vector3 Rotation;
        Vector3 Scale;

        Transform() : Position(Vector3::Zero()),
                      Rotation(Vector3::Zero()),
                      Scale(Vector3(1, 1, 1)) {}
    };
}