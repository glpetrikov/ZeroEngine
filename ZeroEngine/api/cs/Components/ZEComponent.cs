namespace ZeroEngine;

public abstract class ZEComponent
{
    public ulong EntityId { get; private set; }

    public uint EntityIndex => (uint)(EntityId & 0xFFFFFFFF);

    public uint EntityGeneration => (uint)(EntityId >> 32);

    internal abstract ComponentType ComponentType { get; }

    internal void Bind(ulong entityId)
    {
        EntityId = entityId;
    }
}
