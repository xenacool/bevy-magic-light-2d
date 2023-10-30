use bevy::prelude::*;
#[cfg(feature = "egui")]
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
#[cfg(feature = "egui")]
use bevy_inspector_egui::InspectorOptions;

#[derive(Copy, Clone, Reflect)]
pub struct TargetScalingParams {
    /// Scale factor for SDF map.
    pub sdf_scale: f32,
}

impl Default for TargetScalingParams {
    fn default() -> Self {
        Self { sdf_scale: 0.5 }
    }
}

#[rustfmt::skip]
#[derive(Resource, Default, Reflect, Copy, Clone)]
pub struct BevyMagicLight2DSettings {
    pub light_pass_params: LightPassParams,
    pub target_scaling_params: TargetScalingParams,
}

#[rustfmt::skip]
#[derive(Reflect, Copy, Clone, Debug)]
#[cfg_attr(feature = "egui", derive(InspectorOptions))]
#[cfg_attr(feature = "egui", reflect(InspectorOptions))]
pub struct LightPassParams {

    /// Number of previous frames to keep in the reservoir.
    #[cfg_attr(feature = "egui", inspector(min = 1, max = 64))]
    pub reservoir_size: u32,

    /// Size of the bilateral filter kernel used to smooth/denoise
    /// irradiance values.
    pub smooth_kernel_size: (u32, u32),

    /// How much of the final light contribution should be direct light.
    #[cfg_attr(feature = "egui", inspector(min = 0.0, max = 1.0))]
    pub direct_light_contrib: f32,

    /// How much of the final light contribution should be indirect light.
    #[cfg_attr(feature = "egui", inspector(min = 0.0, max = 1.0))]
    pub indirect_light_contrib: f32,

    /// Number of rays to cast when sampling the indirect light
    /// from direct light irradiance map.
    #[cfg_attr(feature = "egui", inspector(min = 0, max = 512))]
    pub indirect_rays_per_sample: i32,

    /// TODO(zaycev): document
    #[cfg_attr(feature = "egui", inspector(min = 1.0, max = 100.0))]
    pub indirect_rays_radius_factor: f32,
}

impl Default for LightPassParams {
    fn default() -> Self {
        Self {
            reservoir_size: 8,
            smooth_kernel_size: (2, 1),
            direct_light_contrib: 0.5,
            indirect_light_contrib: 0.5,
            indirect_rays_per_sample: 32,
            indirect_rays_radius_factor: 3.5,
        }
    }
}

#[derive(Default, Debug, Resource, Copy, Clone)]
pub struct ComputedTargetSizes {
    pub(crate) primary_target_size: Vec2,
    pub(crate) primary_target_isize: IVec2,
    pub(crate) primary_target_usize: UVec2,
    pub(crate) sdf_target_size: Vec2,
    pub(crate) sdf_target_isize: IVec2,
    pub(crate) sdf_target_usize: UVec2,
}

impl ComputedTargetSizes {
    pub fn from_window(window: &Window, params: &TargetScalingParams) -> Self {
        let primary_size = Vec2::new(
            (window.physical_width() as f64 / window.scale_factor()) as f32,
            (window.physical_height() as f64 / window.scale_factor()) as f32,
        );

        let mut sizes = Self::default();

        sizes.primary_target_size = primary_size;
        sizes.primary_target_isize = sizes.primary_target_size.as_ivec2();
        sizes.primary_target_usize = sizes.primary_target_size.as_uvec2();

        sizes.sdf_target_size = primary_size * params.sdf_scale;
        sizes.sdf_target_isize = sizes.sdf_target_size.as_ivec2();
        sizes.sdf_target_usize = sizes.sdf_target_size.as_uvec2();

        sizes
    }
}
