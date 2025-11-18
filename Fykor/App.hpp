#pragma once

#include "Core.hpp"

namespace Fykor {
    class FYKOR_API App {
    public:
        App();
        virtual ~App();

        virtual void Run();
    };

    // defined is User
    App* CreateApp();
}