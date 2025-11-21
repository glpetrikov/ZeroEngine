#pragma once

#include "Core.h"
#include "Events/Event.h"

#include "Window.h"

namespace Fykor {
    class FYKOR_API App {
    public:
        App();
        virtual ~App();

        virtual void Run();
    };

    // defined is User
    App *CreateApp();
}