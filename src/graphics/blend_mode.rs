//! Blending modes for drawing (for render states)

use csfml_sys::{
    sfBlendEquation, sfBlendEquationAdd, sfBlendEquationMax, sfBlendEquationMin,
    sfBlendEquationReverseSubtract, sfBlendEquationSubtract, sfBlendFactor, sfBlendFactorDstAlpha,
    sfBlendFactorDstColor, sfBlendFactorOne, sfBlendFactorOneMinusDstAlpha,
    sfBlendFactorOneMinusDstColor, sfBlendFactorOneMinusSrcAlpha, sfBlendFactorOneMinusSrcColor,
    sfBlendFactorSrcAlpha, sfBlendFactorSrcColor, sfBlendFactorZero, sfBlendMode,
};

/// Defines the blending factors used in blending equations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Factor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
}

impl Factor {
    const fn to_sfml(self) -> sfBlendFactor {
        match self {
            Self::Zero => sfBlendFactorZero,
            Self::One => sfBlendFactorOne,
            Self::SrcColor => sfBlendFactorSrcColor,
            Self::OneMinusSrcColor => sfBlendFactorOneMinusSrcColor,
            Self::DstColor => sfBlendFactorDstColor,
            Self::OneMinusDstColor => sfBlendFactorOneMinusDstColor,
            Self::SrcAlpha => sfBlendFactorSrcAlpha,
            Self::OneMinusSrcAlpha => sfBlendFactorOneMinusSrcAlpha,
            Self::DstAlpha => sfBlendFactorDstAlpha,
            Self::OneMinusDstAlpha => sfBlendFactorOneMinusDstAlpha,
        }
    }
}

impl From<Factor> for sfBlendFactor {
    fn from(value: Factor) -> Self {
        value.to_sfml()
    }
}

/// Defines the blending equations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Equation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

impl Equation {
    const fn to_sfml(self) -> sfBlendEquation {
        match self {
            Self::Add => sfBlendEquationAdd,
            Self::Subtract => sfBlendEquationSubtract,
            Self::ReverseSubtract => sfBlendEquationReverseSubtract,
            Self::Min => sfBlendEquationMin,
            Self::Max => sfBlendEquationMax,
        }
    }
}

impl From<Equation> for sfBlendEquation {
    fn from(value: Equation) -> Self {
        value.to_sfml()
    }
}

/// The struct representing a blend mode.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlendMode {
    pub color_src_factor: Factor,
    pub color_dst_factor: Factor,
    pub color_equation: Equation,
    pub alpha_src_factor: Factor,
    pub alpha_dst_factor: Factor,
    pub alpha_equation: Equation,
}

impl BlendMode {
    /// Bitcasts this blend mode to the csfml struct.
    ///
    /// This is used for interacting with the CSFML C bindings.
    #[must_use]
    pub fn to_csfml(self) -> sfBlendMode {
        unsafe { std::mem::transmute(self) }
    }
}

/// Preset blend modes
pub const BLEND_ALPHA: BlendMode = BlendMode {
    color_src_factor: Factor::SrcAlpha,
    color_dst_factor: Factor::OneMinusSrcAlpha,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::OneMinusSrcAlpha,
    alpha_equation: Equation::Add,
};

pub const BLEND_ADD: BlendMode = BlendMode {
    color_src_factor: Factor::SrcAlpha,
    color_dst_factor: Factor::One,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::One,
    alpha_equation: Equation::Add,
};

pub const BLEND_MULTIPLY: BlendMode = BlendMode {
    color_src_factor: Factor::DstColor,
    color_dst_factor: Factor::Zero,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::DstColor,
    alpha_dst_factor: Factor::Zero,
    alpha_equation: Equation::Add,
};

pub const BLEND_MIN: BlendMode = BlendMode {
    color_src_factor: Factor::One,
    color_dst_factor: Factor::One,
    color_equation: Equation::Min,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::One,
    alpha_equation: Equation::Min,
};

pub const BLEND_MAX: BlendMode = BlendMode {
    color_src_factor: Factor::One,
    color_dst_factor: Factor::One,
    color_equation: Equation::Max,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::One,
    alpha_equation: Equation::Max,
};

pub const BLEND_NONE: BlendMode = BlendMode {
    color_src_factor: Factor::One,
    color_dst_factor: Factor::Zero,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::Zero,
    alpha_equation: Equation::Add,
};
