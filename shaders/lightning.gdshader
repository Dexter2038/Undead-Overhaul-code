shader_type canvas_item;

// Uniforms allow you to control shader properties from your Godot script.
// You can adjust these values in the Inspector when the shader is applied to a node.

// The normalized direction of the sun.
// Examples:
// vec2(0.0, -1.0) = Sun directly above (light from top)
// vec2(1.0, -1.0) = Sun from top-right
// vec2(-1.0, -1.0) = Sun from top-left
// vec2(1.0, 0.0) = Sun from right
uniform vec2 sun_direction = vec2(0.707, -0.707); // Default: Top-right (normalized)

// The color of the direct sunlight.
uniform vec4 light_color : source_color = vec4(1.0, 0.9, 0.7, 1.0); // Warm yellow/orange

// The color of the ambient light (light that's everywhere, even in shadows).
uniform vec4 ambient_color : source_color = vec4(0.2, 0.25, 0.35, 1.0); // Cool blue/purple

// Controls the overall brightness of the direct sunlight.
uniform float light_intensity : hint_range(0.0, 2.0) = 1.0;

// Controls how dark the shadows appear. A value of 0.0 means no shadows, 1.0 means full darkness.
uniform float shadow_strength : hint_range(0.0, 1.0) = 0.6;

// An optional texture used for occlusion (shadow casting).
// This texture should represent your world's solid objects.
// Typically, black pixels mean fully opaque/blocking, and white pixels mean transparent/non-blocking.
// You would generate this texture by rendering your solid tiles to a separate Viewport.
uniform sampler2D occlusion_texture;

void fragment() {
    // Get the original color of the pixel from the rendered scene.
    vec4 base_color = texture(TEXTURE, UV);

    // Normalize the sun direction to ensure it's a unit vector.
    // This is important for consistent lighting calculations.
    vec2 normalized_sun_direction = normalize(sun_direction);

    // --- Occlusion Calculation ---
    float occlusion_factor = 1.0; // Default to no occlusion (full light)

    // Check if an occlusion texture has been provided and is valid.
    // `textureSize(occlusion_texture, 0).x > 0` checks if the texture has a width greater than 0.
    if (textureSize(occlusion_texture, 0).x > 0) {
        // To simulate shadows, we sample the occlusion texture slightly *against* the sun's direction.
        // If we find a dark pixel (an occluder) in that direction, it means this pixel should be in shadow.

        // `SCREEN_PIXEL_SIZE` gives us the size of one pixel in UV coordinates (0-1 range).
        // We offset the UV by a small amount (e.g., 2 pixels) in the opposite direction of the sun.
        // This creates a simple "ray" check for blockers.
        vec2 offset_uv = UV - normalized_sun_direction * SCREEN_PIXEL_SIZE * 2.0; // Offset by 2 pixels

        // Sample the red channel of the occlusion texture.
        // Assuming the occlusion texture is grayscale where 0.0 is occluded and 1.0 is clear.
        occlusion_factor = texture(occlusion_texture, offset_uv).r;

        // Clamp the occlusion factor to ensure it stays within valid range [0, 1].
        occlusion_factor = clamp(occlusion_factor, 0.0, 1.0);
    }

    // --- Lighting Calculation ---

    // Mix between ambient light and direct light based on light_intensity.
    // If light_intensity is 0, it's all ambient. If 1, it's a full mix.
    vec4 direct_light = light_color * light_intensity;
    vec4 final_light_color = mix(ambient_color, direct_light, light_intensity);

    // Apply the occlusion factor to the final light color.
    // We mix the `final_light_color` with a darker version of itself.
    // `1.0 - occlusion_factor` means:
    //   - If `occlusion_factor` is 1.0 (no occlusion), `1.0 - 1.0 = 0.0`, so no shadow is applied.
    //   - If `occlusion_factor` is 0.0 (full occlusion), `1.0 - 0.0 = 1.0`, so full shadow is applied.
    // The `shadow_strength` controls how much the shadow darkens the light.
    final_light_color = mix(final_light_color, final_light_color * (1.0 - shadow_strength), 1.0 - occlusion_factor);

    // Multiply the base scene color by the calculated light color.
    // This applies the lighting effect to your entire game scene.
    COLOR = base_color * final_light_color;
}
