/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * Layer.cpp
 * ─────────────────────────────────────────────────
 * Layer class implementation
 * =================================================
 */
#include "../Common.h"
#include "Layer.h"

namespace ZeroEngine::Layers {
Layer::Layer(const std::string& debugname) : m_DebugName(debugname) {}

Layer::~Layer() {}
} // namespace ZeroEngine::Layers
