struct View {
    view_proj: mat4x4<f32>;
    inverse_view: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
    near: f32;
    far: f32;
    width: f32;
    height: f32;
};

struct Mesh2d {
    model: mat4x4<f32>;
    inverse_transpose_model: mat4x4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
};

struct Elapsed {
  seconds: f32;
};

[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var texture: texture_2d<f32>;
[[group(1), binding(1)]]
var texture_sampler: sampler;
[[group(1), binding(2)]]
var<uniform> elapsed: Elapsed;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
};


fn rgb2hsv(c: vec4<f32>) -> vec4<f32> {
    var k = vec4<f32>(0., -1. / 3., 2. / 3., -1.);

    var p = mix(vec4<f32>(c.bg, k.wz), vec4<f32>(c.gb, k.xy), step(c.b, c.g));
    var q = mix(vec4<f32>(p.xyw, c.r), vec4<f32>(c.r, p.yzx), step(p.x, c.r));

    var d = q.x - min(q.w, q.y);
    var e = 1.0e-10;

    return vec4<f32>(abs(q.z + (q.w - q.y) / (6. * d + e)), d / (q.x + e), q.x, c.a);
}

fn hsv2rgb(c: vec4<f32>) -> vec4<f32> {
    var k = vec4<f32>(1., 2. / 3., 1. / 3., 3.);

    var p = abs(fract(c.xxx + k.xyz) * 6. - k.www);

    return vec4<f32>(
        c.z * mix(k.xxx, clamp(p - k.xxx, vec3<f32>(0., 0., 0.), vec3<f32>(1., 1., 1.)), c.y), c.a);
}

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var tex_color: vec4<f32> = textureSample(texture, texture_sampler, in.uv);

    var color_hsv = rgb2hsv(tex_color);

    color_hsv.x = color_hsv.x + elapsed.seconds;

    return hsv2rgb(color_hsv);
}
