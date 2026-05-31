namespace ZeroEngine;

public static unsafe class Physics
{
    public static Vector2 GetVelocity(ulong entity)
    {
        float x = 0.0f;
        float y = 0.0f;
        EngineAPI.Current->get_velocity(entity, &x, &y);
        return new Vector2(x, y);
    }

    public static void Add2DForce(ulong entity, float x, float y) => EngineAPI.Current->add_2d_force(entity, x, y);

    public static void Add2DImpulse(ulong entity, float x, float y) => EngineAPI.Current->add_2d_impulse(entity, x, y);
}
