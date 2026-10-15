use trinity_macros::trinity_test;
use trinityal::{Tr2VertexDefinition, Tr2VertexDefinitionItem, VertexDataType, VertexUsageCode};

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitionItems",
    verification = "kani"
)]
fn can_compare_vertex_definition_items() {
    let item1 = Tr2VertexDefinitionItem {
        usage: VertexUsageCode::Position,
        usage_index: 0,
        data_type: VertexDataType::FLOAT16_2,
        offset: 0,
        stream: 0,
        instance_step_rate: 0,
    };
    let item2 = Tr2VertexDefinitionItem {
        usage: VertexUsageCode::Texcoord,
        usage_index: 0,
        data_type: VertexDataType::FLOAT16_2,
        offset: 16,
        stream: 0,
        instance_step_rate: 0,
    };
    assert_eq!(item1, item1);
    assert_eq!(item2, item2);
    assert_ne!(item1, item2);
    assert!(item1 < item2);
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanAddItemsToVertexDefinition",
    verification = "kani"
)]
fn can_add_items_to_vertex_definition() {
    let mut def = Tr2VertexDefinition::default();
    assert!(def.is_empty());
    def.add_default(VertexDataType::FLOAT16_2, VertexUsageCode::Texcoord);
    def.add(
        VertexDataType::FLOAT32_3,
        VertexUsageCode::Position,
        2,
        1,
        1,
    );
    def.add(
        VertexDataType::FLOAT32_3,
        VertexUsageCode::Position,
        0,
        1,
        0,
    );
    def.add(VertexDataType::FLOAT32_1, VertexUsageCode::Normal, 0, 0, 1);
    assert_eq!(0, def.items[0].offset);
    assert_eq!(0, def.items[1].offset);
    assert_eq!(12, def.items[2].offset);
    assert_eq!(4, def.items[3].offset);
}
