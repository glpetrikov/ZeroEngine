using System;
using System.Runtime.InteropServices;

namespace Scripts;

public static class Script
{
    [UnmanagedCallersOnly]
    public static void OnCreate()
    {
        Console.WriteLine("Create");
    }

    [UnmanagedCallersOnly]
    public static void OnStart()
    {
        Console.WriteLine("Start");
    }

    [UnmanagedCallersOnly]
    public static void OnDestroy()
    {
        Console.WriteLine("Destroy");
    }

    [UnmanagedCallersOnly]
    public static void OnUpdate(float dt)
    {
        // Console.WriteLine($"Hello from C#! dt={dt}");
    }

    [UnmanagedCallersOnly]
    public static void OnFixedUpdate(float dt)
    {
        // Console.WriteLine("FixedUpdate");
    }

    [UnmanagedCallersOnly]
    public static void OnEnable()
    {
        Console.WriteLine("Enable");
    }

    [UnmanagedCallersOnly]
    public static void OnDisable()
    {
        Console.WriteLine("Disable");
    }

    [UnmanagedCallersOnly]
    public static void OnContactEnter(ulong otherEntity)
    {
        Console.WriteLine("ContactEnter");
    }

    [UnmanagedCallersOnly]
    public static void OnContactStay(ulong otherEntity)
    {
        Console.WriteLine("ContactStay");
    }

    [UnmanagedCallersOnly]
    public static void OnContactExit(ulong otherEntity)
    {
        Console.WriteLine("ContactExit");
    }

    [UnmanagedCallersOnly]
    public static void OnSensorEnter(ulong otherEntity)
    {
        Console.WriteLine("SensorEnter");
    }

    [UnmanagedCallersOnly]
    public static void OnSensorStay(ulong otherEntity)
    {
        Console.WriteLine("SensorStay");
    }

    [UnmanagedCallersOnly]
    public static void OnSensorExit(ulong otherEntity)
    {
        Console.WriteLine("SensorExit");
    }
}
