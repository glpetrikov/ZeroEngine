#pragma once

#include <FrameLog.hpp>

#include "../Platform.h"

namespace Fykor::Debug {
    using FLLogger = FrameLog::Logger;
    inline FLLogger FykorLogger("Fykor");
    inline FLLogger Logger("App");
}