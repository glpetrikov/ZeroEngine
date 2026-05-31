using System;
using System.Collections.Generic;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text;

namespace ZeroEngine;

public abstract class ZEScript
{
    private static readonly Dictionary<ulong, ZEScript> _instances = new();

    public ulong EntityId { get; protected set; }

    public uint EntityIndex => (uint)(EntityId & 0xFFFFFFFF);

    public uint EntityGeneration => (uint)(EntityId >> 32);

    public T GetComponent<T>()
        where T : ZEComponent, new()
    {
        var component = new T();
        component.Bind(EntityId);

        if (!EngineAPI.HasComponent(EntityId, component.ComponentType))
        {
            throw new InvalidOperationException(
                $"Entity {EntityIndex}.{EntityGeneration} does not have component {typeof(T).Name}.");
        }

        return component;
    }

    public virtual void OnCreate() { }

    public virtual void OnStart() { }

    public virtual void OnDestroy() { }

    public virtual void OnUpdate() { }

    public virtual void OnFixedUpdate() { }

    public virtual void OnEnable() { }

    public virtual void OnDisable() { }

    public virtual void OnContactEnter(ulong otherEntity) { }

    public virtual void OnContactStay(ulong otherEntity) { }

    public virtual void OnContactExit(ulong otherEntity) { }

    public virtual void OnSensorEnter(ulong otherEntity) { }

    public virtual void OnSensorStay(ulong otherEntity) { }

    public virtual void OnSensorExit(ulong otherEntity) { }

    [UnmanagedCallersOnly]
    public static unsafe void NativeOnEngineInit(
        ulong entityId,
        EngineAPI* api,
        byte* classPathPtr,
        int classPathLength)
    {
        EngineAPI.Initialize(api);
        var classPath = Encoding.UTF8.GetString(classPathPtr, classPathLength);

        if (_instances.TryGetValue(entityId, out var existing) && existing.GetType().FullName == classPath)
        {
            return;
        }

        var scriptType = ResolveScriptType(classPath);
        var instance = (ZEScript)Activator.CreateInstance(scriptType)!;
        instance.EntityId = entityId;
        _instances[entityId] = instance;
    }

    [UnmanagedCallersOnly]
    public static void NativeOnCreate(ulong entityId) => Lookup(entityId).OnCreate();

    [UnmanagedCallersOnly]
    public static void NativeOnStart(ulong entityId) => Lookup(entityId).OnStart();

    [UnmanagedCallersOnly]
    public static void NativeOnDestroy(ulong entityId)
    {
        if (!_instances.Remove(entityId, out var instance))
        {
            return;
        }

        instance.OnDestroy();
    }

    [UnmanagedCallersOnly]
    public static void NativeOnUpdate(ulong entityId) => Lookup(entityId).OnUpdate();

    [UnmanagedCallersOnly]
    public static void NativeOnFixedUpdate(ulong entityId) => Lookup(entityId).OnFixedUpdate();

    [UnmanagedCallersOnly]
    public static void NativeOnEnable(ulong entityId) => Lookup(entityId).OnEnable();

    [UnmanagedCallersOnly]
    public static void NativeOnDisable(ulong entityId) => Lookup(entityId).OnDisable();

    [UnmanagedCallersOnly]
    public static void NativeOnContactEnter(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnContactEnter(otherEntity);

    [UnmanagedCallersOnly]
    public static void NativeOnContactStay(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnContactStay(otherEntity);

    [UnmanagedCallersOnly]
    public static void NativeOnContactExit(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnContactExit(otherEntity);

    [UnmanagedCallersOnly]
    public static void NativeOnSensorEnter(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnSensorEnter(otherEntity);

    [UnmanagedCallersOnly]
    public static void NativeOnSensorStay(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnSensorStay(otherEntity);

    [UnmanagedCallersOnly]
    public static void NativeOnSensorExit(ulong entityId, ulong otherEntity) =>
        Lookup(entityId).OnSensorExit(otherEntity);

    private static ZEScript Lookup(ulong entityId)
    {
        if (!_instances.TryGetValue(entityId, out var instance))
        {
            throw new InvalidOperationException($"No script instance registered for entity id {entityId}.");
        }

        return instance;
    }

    private static Type ResolveScriptType(string classPath)
    {
        var scriptType = Type.GetType($"{classPath}, Scripts", throwOnError: false)
            ?? Assembly.GetExecutingAssembly().GetType(classPath, throwOnError: false);

        if (scriptType is null)
        {
            throw new InvalidOperationException($"Could not resolve script type `{classPath}`.");
        }

        if (!typeof(ZEScript).IsAssignableFrom(scriptType))
        {
            throw new InvalidOperationException($"Script type `{classPath}` must inherit from ZEScript.");
        }

        return scriptType;
    }
}
