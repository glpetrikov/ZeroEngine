namespace ZeroEngine;

public static unsafe class Input
{
    public static bool IsKeyPressed(KeyCode key) => EngineAPI.Current->is_key_pressed((int)key);

    public static bool IsKeyJustPressed(KeyCode key) => EngineAPI.Current->is_key_just_pressed((int)key);

    public static bool IsKeyReleased(KeyCode key) => EngineAPI.Current->is_key_released((int)key);

    public static bool IsKeyJustReleased(KeyCode key) => EngineAPI.Current->is_key_just_released((int)key);

    public static bool IsMouseButtonPressed(int button) => EngineAPI.Current->is_mouse_button_pressed(button);

    public static bool IsMouseButtonJustPressed(int button) => EngineAPI.Current->is_mouse_button_just_pressed(button);

    public static Vector2 GetMousePosition()
    {
        float x = 0.0f;
        float y = 0.0f;
        EngineAPI.Current->get_mouse_position(&x, &y);
        return new Vector2(x, y);
    }

    public static Vector2 GetMouseDelta()
    {
        float x = 0.0f;
        float y = 0.0f;
        EngineAPI.Current->get_mouse_delta(&x, &y);
        return new Vector2(x, y);
    }
}
