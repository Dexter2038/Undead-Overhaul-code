shader_type canvas_item;

// --- Glow Parameters ---
// Toggles the glowing effect on or off.
uniform bool enable_glow = true; // Default: Glow is enabled

// The color of the glow effect
uniform vec4 glow_color : source_color = vec4(1.0, 0.8, 0.2, 1.0); // Default: Orange-yellow (change to white or your preference)

// The maximum radius of the glow effect in pixels
uniform float glow_radius : hint_range(0.0, 20.0, 0.1) = 5.0; // Default: 5 pixels

// --- New Pulsation Parameters ---
// How fast the glow pulsates (speed of the "breathing" effect).
uniform float glow_pulse_speed : hint_range(0.1, 10.0, 0.1) = 2.0; // Default: 2.0 (cycles per second, roughly)

// How strong the pulsation is (how much the glow intensity fluctuates).
// 0.0 means no pulsation, 1.0 means it fluctuates from 0% to 100% of its base intensity.
uniform float glow_pulse_intensity : hint_range(0.0, 1.0, 0.01) = 0.3; // Default: 0.3 (30% fluctuation)

// --- Sprite Body Parameters ---
// The alpha threshold to determine what is considered part of the sprite vs. transparent background.
// Pixels with alpha above this value will be considered "solid" and show the original sprite.
uniform float alpha_threshold : hint_range(0.0, 1.0, 0.01) = 0.1; // Default: 0.1

void fragment() {
    vec4 base_texture_color = texture(TEXTURE, UV);
    float current_alpha = base_texture_color.a;

    vec4 final_pixel_color = vec4(0.0); // Initialize as fully transparent

    if (current_alpha > alpha_threshold) {
        final_pixel_color = base_texture_color; // Original sprite color
    } else {
        if (enable_glow) {
            float min_dist_sq = (glow_radius * glow_radius) + 1.0;
            bool found_opaque_neighbor = false;

            for (float y_offset = -glow_radius; y_offset <= glow_radius; y_offset += 1.0) {
                for (float x_offset = -glow_radius; x_offset <= glow_radius; x_offset += 1.0) {
                    vec2 sample_uv = UV + vec2(x_offset, y_offset) * TEXTURE_PIXEL_SIZE;

                    if (sample_uv.x >= 0.0 && sample_uv.x <= 1.0 &&
                        sample_uv.y >= 0.0 && sample_uv.y <= 1.0) {

                        vec4 sample_color = texture(TEXTURE, sample_uv);

                        if (sample_color.a > alpha_threshold) {
                            float dist_sq = x_offset * x_offset + y_offset * y_offset;

                            if (dist_sq < min_dist_sq) {
                                min_dist_sq = dist_sq;
                                found_opaque_neighbor = true;
                            }
                        }
                    }
                }
            }

            if (found_opaque_neighbor) {
                float distance = sqrt(min_dist_sq);
                // Calculate base glow alpha based on distance from sprite edge
                float base_glow_alpha = smoothstep(glow_radius, 0.0, distance);

                // --- Pulsation Logic ---
                // sin(TIME * speed) gives a value from -1.0 to 1.0.
                // (value * 0.5 + 0.5) normalizes it to 0.0 to 1.0.
                float pulse_value = (sin(TIME * glow_pulse_speed) * 0.5 + 0.5);

                // Adjust the final glow alpha based on the pulse value and intensity.
                // If glow_pulse_intensity is 0.0, final_glow_alpha = base_glow_alpha * 1.0 (no change).
                // If glow_pulse_intensity is 1.0, it fully fluctuates from 0% to 100% of base.
                float final_glow_alpha = base_glow_alpha * (1.0 - glow_pulse_intensity + (glow_pulse_intensity * pulse_value));

                // Ensure the final alpha doesn't exceed the glow_color's alpha
                // and is not less than 0.0, though the math here should keep it positive.
                final_pixel_color = vec4(glow_color.rgb, glow_color.a * final_glow_alpha);
            } else {
                final_pixel_color = vec4(0.0);
            }
        } else {
            final_pixel_color = vec4(0.0);
        }
    }

    COLOR = final_pixel_color;
}