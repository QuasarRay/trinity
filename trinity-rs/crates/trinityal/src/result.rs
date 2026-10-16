use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};
use trinity_macros::TrinityClass;

pub type HResult = i32;
pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const E_FAIL: HResult = 0x8000_4005_u32.cast_signed();
pub const E_INVALIDARG: HResult = 0x8007_0057_u32.cast_signed();
pub const E_INVALIDCALL: HResult = 0x8876_086C_u32.cast_signed();
pub const E_OUTOFMEMORY: HResult = 0x8007_000E_u32.cast_signed();
pub const E_DEVICELOST: HResult = 0x8876_0868_u32.cast_signed();

#[must_use]
pub const fn succeeded(result: HResult) -> bool {
    result >= 0
}

#[must_use]
pub const fn failed(result: HResult) -> bool {
    result < 0
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub enum ALResultCategory {
    #[default]
    Success,
    InvalidCall,
    OutOfMemory,
    DeviceLost,
    Other,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Reflect,
    Serialize,
    Deserialize,
    TrinityClass,
)]
#[trinity(
    module = "trinity",
    name = "ALResult",
    legacy_header = "trinityal/ALResult.h",
    kind = "value"
)]
pub struct ALResult {
    #[trinity(read, persist)]
    result: HResult,
}

impl Default for ALResult {
    fn default() -> Self {
        Self { result: S_OK }
    }
}

impl ALResult {
    #[must_use]
    pub const fn new(result: HResult) -> Self {
        Self { result }
    }

    #[must_use]
    pub const fn get_result(self) -> HResult {
        self.result
    }

    #[must_use]
    pub const fn get_category(self) -> ALResultCategory {
        match self.result {
            value if value >= 0 => ALResultCategory::Success,
            E_INVALIDCALL | E_INVALIDARG => ALResultCategory::InvalidCall,
            E_OUTOFMEMORY => ALResultCategory::OutOfMemory,
            E_DEVICELOST => ALResultCategory::DeviceLost,
            _ => ALResultCategory::Other,
        }
    }

    #[must_use]
    pub const fn is_success(self) -> bool {
        succeeded(self.result)
    }

    #[must_use]
    pub const fn error_message(self) -> &'static str {
        match self.result {
            S_OK => "S_OK",
            S_FALSE => "S_FALSE",
            E_FAIL => "E_FAIL",
            E_INVALIDARG => "E_INVALIDARG",
            E_INVALIDCALL => "E_INVALIDCALL",
            E_OUTOFMEMORY => "E_OUTOFMEMORY",
            E_DEVICELOST => "E_DEVICELOST",
            _ => "Unknown AL error",
        }
    }
}

impl From<HResult> for ALResult {
    fn from(result: HResult) -> Self {
        Self::new(result)
    }
}

impl From<ALResult> for HResult {
    fn from(result: ALResult) -> Self {
        result.result
    }
}

#[must_use]
pub const fn be_is_success(result: ALResult) -> bool {
    result.is_success()
}

#[must_use]
pub const fn be_get_error_message(result: ALResult) -> &'static str {
    result.error_message()
}
