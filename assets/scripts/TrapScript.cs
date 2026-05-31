using System;
using System.Runtime.InteropServices;

namespace Scripts;

public static class TrapScript
{
    private const ulong CircleEntity = 4;

    [UnmanagedCallersOnly]
    public static void OnContactEnter(ulong otherEntity)
    {
        if (otherEntity == CircleEntity)
        {
            Console.WriteLine("Circle trapped!");
        }
    }

    [UnmanagedCallersOnly]
    public static void OnContactExit(ulong otherEntity)
    {
        if (otherEntity == CircleEntity)
        {
            Console.WriteLine("Circle escaped!");
        }
    }
}
