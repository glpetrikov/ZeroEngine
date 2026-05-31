using System;
using ZeroEngine;

namespace Scripts;

public class TrapScript : ZEScript
{
    private const uint CircleEntityIndex = 4;

    public override void OnContactEnter(ulong otherEntity)
    {
        if (EntityIndexOf(otherEntity) == CircleEntityIndex)
        {
            Console.WriteLine("Circle trapped!");
        }
    }

    public override void OnContactExit(ulong otherEntity)
    {
        if (EntityIndexOf(otherEntity) == CircleEntityIndex)
        {
            Console.WriteLine("Circle escaped!");
        }
    }

    private static uint EntityIndexOf(ulong entityId) => (uint)(entityId & 0xFFFFFFFF);
}
