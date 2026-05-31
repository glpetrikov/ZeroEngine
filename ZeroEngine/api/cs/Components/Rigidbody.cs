namespace ZeroEngine;

public sealed unsafe class Rigidbody : ZEComponent
{
    internal override ComponentType ComponentType => ComponentType.Rigidbody;

    public Vector2 Velocity
    {
        get
        {
            float x = 0.0f;
            float y = 0.0f;
            EngineAPI.Current->get_velocity(EntityId, &x, &y);
            return new Vector2(x, y);
        }
    }

    public void Add2DForce(float x, float y)
    {
        EngineAPI.Current->add_2d_impulse(EntityId, x, y);
    }

    public void Add2DForce(Vector2 force)
    {
        Add2DForce(force.X, force.Y);
    }

    public void Add2DForceWithMax(Vector2 force, Vector2 maxVelocity)
    {
        Add2DForceWithMax(force.X, force.Y, maxVelocity.X, maxVelocity.Y);
    }

    public void Add2DForceWithMax(float x, float y, float maxX, float maxY)
    {
        var currentVel = Velocity;
        float requiredX = x;

        if (x > 0.0f && currentVel.X >= maxX) requiredX = 0.0f;
        if (x < 0.0f && currentVel.X <= -maxX) requiredX = 0.0f;

        float requiredY = y;
        if (y > 0.0f && currentVel.Y >= maxY) requiredY = 0.0f;
        if (y < 0.0f && currentVel.Y <= -maxY) requiredY = 0.0f;

        if (requiredX == 0.0f && requiredY == 0.0f)
        {
            return;
        }

        Add2DForce(requiredX, requiredY);
    }
}