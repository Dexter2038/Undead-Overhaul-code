shader_type canvas_item;

uniform float pulse_speed = 2.5;
uniform float base_alpha = 0.4;
uniform float pulse_alpha = 0.6;
uniform float border_thickness = 0.05;
uniform bool active = false;

void fragment() {
    // Pulse between 0 and 1
    float pulse = sin(TIME * pulse_speed) * 0.5 + 0.5;

    // Edge detection
    float left   = step(UV.x, border_thickness);
    float right  = step(1.0 - UV.x, border_thickness);
    float top    = step(UV.y, border_thickness);
    float bottom = step(1.0 - UV.y, border_thickness);
    float border = clamp(left + right + top + bottom, 0.0, 1.0);

    // Blend alpha based on pulse and click state
    float strength = mix(base_alpha, pulse_alpha, pulse);
    if (active) {
        strength = mix(base_alpha, 1.0, pulse);
    } else {
        strength = 0.0;
    }

    COLOR = vec4(1.0, 1.0, 1.0, border * strength);
}
