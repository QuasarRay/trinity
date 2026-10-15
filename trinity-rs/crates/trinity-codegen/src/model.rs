use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum LegacyTestKind {
    CppGTest,
    Python,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum VerificationClass {
    Kani,
    Native,
    RenderDevice,
    Screenshot,
    Integration,
    Unmapped,
}

impl VerificationClass {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Kani => "kani",
            Self::Native => "native",
            Self::RenderDevice => "render-device",
            Self::Screenshot => "screenshot",
            Self::Integration => "integration",
            Self::Unmapped => "unmapped",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LegacyTest {
    pub id: String,
    pub source: String,
    pub suite: String,
    pub name: String,
    pub line: Option<u32>,
    pub kind: LegacyTestKind,
    pub suggested_verification: VerificationClass,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RustTest {
    pub crate_name: String,
    pub source: String,
    pub function: String,
    pub body_name: String,
    pub legacy: String,
    pub verification: VerificationClass,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DoxygenSymbol {
    pub source: String,
    pub qualified_name: String,
    pub kind: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub baseline_commit: String,
    pub docs_sha256: String,
    pub doxygen_symbols: Vec<DoxygenSymbol>,
    pub legacy_tests: Vec<LegacyTest>,
    pub rust_tests: Vec<RustTest>,
}
