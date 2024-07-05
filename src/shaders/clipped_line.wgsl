#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

const LOW: f32 = 0.001;
const HIGH: f32 = 0.002;

struct LineMaterial {
    color: vec4<f32>,
    alignment: vec3<f32>,
    radius: f32,
    offset: f32,
    x_axis_color: vec4<f32>,
    y_axis_color: vec4<f32>,
    z_axis_color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: LineMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let dist = view.world_position.xyz - mesh.world_position.xyz;
    let dist_aligned = dist * material.alignment;
    let dist_squared = dot(dist_aligned, dist_aligned);
    let radius_squared = material.radius * material.radius;
    // Discard pixels after round border
    if dist_squared > radius_squared {
        discard;
    }

    // Color axis
    let alignment_inverted = vec3(1.0) - material.alignment;
    let offset_position = abs(mesh.world_position.xyz - alignment_inverted * material.offset);
    let xmix = smoothstep(LOW, HIGH, max(offset_position.y, offset_position.z));
    let ymix = smoothstep(LOW, HIGH, max(offset_position.x, offset_position.z));
    let zmix = smoothstep(LOW, HIGH, max(offset_position.x, offset_position.y));

    var color = mix(material.x_axis_color, mix(material.y_axis_color, mix(material.z_axis_color, material.color, zmix), ymix), xmix);

    // Attenuate alpha based on normal to camera to avoid overwhelming brightness at shallow angles
    let dist_normal = abs(normalize(dist) * alignment_inverted);
    let normal_mix = smoothstep(0.9, 1.0, 1.0 - max(dist_normal.x, max(dist_normal.y, dist_normal.z)));
    // Attenuate based on distance to camera for smooth borders
    let dist_mix = smoothstep(radius_squared * 0.6, radius_squared, dist_squared);
    // Don't attenuate the cardinal axis
    let axis_mix = min(xmix, min(ymix, zmix));

    color.a *= 1.0 - max(dist_mix, min(axis_mix, normal_mix));

    return color;
}
