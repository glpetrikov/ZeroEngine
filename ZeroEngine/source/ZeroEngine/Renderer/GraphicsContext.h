/* =================================================
 * ZeroEngine, Apache 2.0 - License
 * ─────────────────────────────────────────────────
 * GraphicsContext.h
 * ─────────────────────────────────────────────────
 * Graphics Context
 * =================================================
 */

#pragma once

namespace ZeroEngine {
class GraphicsContext {
  public:
	virtual ~GraphicsContext() = default;

	virtual void Init() = 0;
	virtual void SwapBuffers() = 0;
	virtual void SetVSync(bool enable) = 0;
};
} // namespace ZeroEngine
