use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};
use trinity_macros::TrinityClass;

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Reflect, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum VertexUsageCode {
    Position = 0,
    Color = 1,
    Normal = 2,
    Tangent = 3,
    Bitangent = 4,
    Texcoord = 5,
    BlendIndices = 6,
    BlendWeights = 7,
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Reflect, Serialize, Deserialize,
)]
pub struct VertexDataType(pub u8);

impl VertexDataType {
    pub const DT_INT8: Self = Self(0);
    pub const DT_INT16: Self = Self(1);
    pub const DT_INT32: Self = Self(2);
    pub const DT_FLOAT16: Self = Self(3);
    pub const DT_FLOAT32: Self = Self(4);
    pub const DT_UNSIGNED_BIT: u8 = 1 << 3;
    pub const DT_NORMALIZED_BIT: u8 = 1 << 4;
    pub const DT_SIZE_OFFSET: u8 = 5;
    pub const FLOAT16_2: Self = Self(3 | (1 << 5));
    pub const FLOAT32_1: Self = Self(4);
    pub const FLOAT32_2: Self = Self(4 | (1 << 5));
    pub const FLOAT32_3: Self = Self(4 | (2 << 5));
    pub const FLOAT32_4: Self = Self(4 | (3 << 5));

    #[must_use]
    pub const fn scalar_bytes(self) -> u32 {
        match self.0 & 0b111 {
            0 => 1,
            1 | 3 => 2,
            2 | 4 => 4,
            _ => 0,
        }
    }

    #[must_use]
    pub const fn element_count(self) -> u32 {
        u32::from((self.0 >> Self::DT_SIZE_OFFSET) & 0b11) + 1
    }

    #[must_use]
    pub const fn byte_size(self) -> u32 {
        self.scalar_bytes() * self.element_count()
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Reflect, Serialize, Deserialize,
)]
pub struct Tr2VertexDefinitionItem {
    pub usage: VertexUsageCode,
    pub usage_index: u8,
    pub data_type: VertexDataType,
    pub offset: u32,
    pub stream: u8,
    pub instance_step_rate: u8,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Reflect, Serialize, Deserialize, TrinityClass)]
#[trinity(
    module = "trinity",
    name = "Tr2VertexDefinition",
    legacy_header = "trinityal/Tr2VertexDefinition.h",
    kind = "value"
)]
pub struct Tr2VertexDefinition {
    #[trinity(read, persist)]
    pub items: Vec<Tr2VertexDefinitionItem>,
}

impl Tr2VertexDefinition {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(
        &mut self,
        data_type: VertexDataType,
        usage: VertexUsageCode,
        usage_index: u8,
        stream: u8,
        instance_step_rate: u8,
    ) {
        let offset = self
            .items
            .iter()
            .filter(|item| item.stream == stream)
            .map(|item| item.data_type.byte_size())
            .sum();
        self.items.push(Tr2VertexDefinitionItem {
            usage,
            usage_index,
            data_type,
            offset,
            stream,
            instance_step_rate,
        });
    }

    pub fn add_default(&mut self, data_type: VertexDataType, usage: VertexUsageCode) {
        self.add(data_type, usage, 0, 0, 0);
    }

    #[must_use]
    pub fn find(
        &self,
        usage: VertexUsageCode,
        usage_index: u8,
    ) -> Option<&Tr2VertexDefinitionItem> {
        self.items
            .iter()
            .find(|item| item.usage == usage && item.usage_index == usage_index)
    }
}
