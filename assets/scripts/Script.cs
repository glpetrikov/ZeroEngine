using System;
using ZeroEngine;

namespace Scripts;

public class Script : ZEScript
{
    private Rigidbody? rb;
    private bool moveLeft;
    private bool moveRight;
    private bool jumpRequested;

    public override void OnStart()
    {
        rb = GetComponent<Rigidbody>();
    }

    public override void OnUpdate()
    {
        moveLeft = Input.IsKeyPressed(KeyCode.A);
        moveRight = Input.IsKeyPressed(KeyCode.D);

        if (Input.IsKeyJustPressed(KeyCode.Space))
        {
            jumpRequested = true;
        }
    }

    public override void OnFixedUpdate()
    {
        if (rb is null) return;

        const float moveForce = 0.5f;
        const float jumpForce = 2.0f;
        var maxVelocity = new Vector2(2.5f, 5.0f);

        if (moveLeft)
        {
            rb.Add2DForceWithMax(new Vector2(-moveForce, 0.0f), maxVelocity);
        }
        else if (moveRight)
        {
            rb.Add2DForceWithMax(new Vector2(moveForce, 0.0f), maxVelocity);
        }

        if (jumpRequested)
        {
            rb.Add2DForceWithMax(new Vector2(0.0f, jumpForce), maxVelocity);
            jumpRequested = false;
        }
    }
}
