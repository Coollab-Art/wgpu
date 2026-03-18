@group(1) @binding(0) 
var tex2D: texture_2d<f32>;
@group(1) @binding(1) 
var sampShadow: sampler_comparison;

fn CalcShadowPCF1_(T_P_t_TextureDepth: texture_2d<f32>, S_P_t_TextureDepth: sampler_comparison, t_ProjCoord: vec3<f32>) -> f32 {
}

fn CalcShadowPCF(T_P_t_TextureDepth_1: texture_2d<f32>, S_P_t_TextureDepth_1: sampler_comparison, t_ProjCoord_1: vec3<f32>, t_Bias: f32) -> f32 {
    var t_ProjCoord_2: vec3<f32>;
    var t_Bias_1: f32;

    t_ProjCoord_2 = t_ProjCoord_1;
    t_Bias_1 = t_Bias;
    let _e7 = t_ProjCoord_2;
    let _e9 = t_Bias_1;
    t_ProjCoord_2.z = (_e7.z + _e9);
    let _e11 = t_ProjCoord_2;
    let _e13 = CalcShadowPCF1_(T_P_t_TextureDepth_1, S_P_t_TextureDepth_1, _e11.xyz);
    return _e13;
}

fn main_1() {
    let _e4 = CalcShadowPCF1_(tex2D, sampShadow, vec3(0f));
    let _e8 = CalcShadowPCF(tex2D, sampShadow, vec3(0f), 1f);
    return;
}

@fragment 
fn main() {
    main_1();
    return;
}
