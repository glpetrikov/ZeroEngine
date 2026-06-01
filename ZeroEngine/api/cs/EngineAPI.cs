namespace ZeroEngine;

public unsafe struct EngineAPI
{
    public delegate* unmanaged<int, bool> is_key_pressed;
    public delegate* unmanaged<int, bool> is_key_just_pressed;
    public delegate* unmanaged<int, bool> is_key_released;
    public delegate* unmanaged<int, bool> is_key_just_released;
    public delegate* unmanaged<int, bool> is_mouse_button_pressed;
    public delegate* unmanaged<int, bool> is_mouse_button_just_pressed;
    public delegate* unmanaged<float*, float*, void> get_mouse_position;
    public delegate* unmanaged<float*, float*, void> get_mouse_delta;
    public delegate* unmanaged<TimeState*> get_time_state_ptr;
    public delegate* unmanaged<ulong, uint, bool> has_component;
    public delegate* unmanaged<ulong, float*, float*, void> get_velocity;
    public delegate* unmanaged<ulong, float, float, void> add_2d_force;
    public delegate* unmanaged<ulong, float, float, void> add_2d_impulse;
    public delegate* unmanaged<ulong, float> get_sprite_texture_rotation_degrees;
    public delegate* unmanaged<ulong, float, void> set_sprite_texture_rotation_degrees;

    private static EngineAPI* current;

    internal static void Initialize(EngineAPI* api)
    {
        current = api;
        Time.Initialize(api->get_time_state_ptr());
    }

    internal static bool HasComponent(ulong entity, ComponentType componentType)
    {
        return Current->has_component(entity, (uint)componentType);
    }

    internal static EngineAPI* Current
    {
        get
        {
            if (current == null)
            {
                throw new InvalidOperationException("ZeroEngine API was used before OnEngineInit.");
            }

            return current;
        }
    }
}
