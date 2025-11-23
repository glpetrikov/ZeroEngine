#pragma once

#include "Event.h"
#include "Common.h"
#include "Vectors/Vector3.h"

namespace Fykor::Events{
    class EntityEvent : public Event {
    public:
        inline int GetEntityId() const { return EntityId; }

        virtual int GetCategoryFlags() const { return EventCategory::CategoryEntity; }

   	protected:
        EntityEvent(int entityId) : EntityId(entityId) {}

        int EntityId;
    }

	class EntityMovedEvent : public EntityEvent {
	public:
		EntityMovedEvent(int entityId, Vectors::Vector3<float> position) : Position(position), EntityEvent(entityId) {}

		inline Vectors::Vector3<float> GetPosition() const { return Position; }

	private:
		Vectors::Vector3<float> Position;
	}
}

EntityMoved,
		EntityDestroy,
		EntityCreated,
		EntityAddComponent