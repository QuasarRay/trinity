use trinity_macros::trinity_test;
use trinityal::{
    Tr2VertexDefinition, Tr2VertexDefinitionItem, VertexDataType, VertexUsageCode,
};

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitionItems",
    verification = "kani"
)]
fn can_compare_vertex_definition_items() {
    let first = Tr2VertexDefinitionItem {
        usage: VertexUsageCode::Position,
        usage_index: 0,
        data_type: VertexDataType::FLOAT16_2,
        offset: 0,
        stream: 0,
        instance_step_rate: 0,
    };
    let second = Tr2VertexDefinitionItem {
        usage: VertexUsageCode::Texcoord,
        usage_index: 0,
        data_type: VertexDataType::FLOAT16_2,
        offset: 16,
        stream: 0,
        instance_step_rate: 0,
    };
    assert_ne!(first, second);
    assert!(first < second);
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanAddItemsToVertexDefinition",
    verification = "kani"
)]
fn can_add_items_to_vertex_definition() {
    let mut definition = Tr2VertexDefinition::default();
    definition.add_default(VertexDataType::FLOAT16_2, VertexUsageCode::Texcoord);
    definition.add(VertexDataType::FLOAT32_3, VertexUsageCode::Position, 2, 1, 1);
    definition.add(VertexDataType::FLOAT32_3, VertexUsageCode::Position, 0, 1, 0);
    definition.add(VertexDataType::FLOAT32_1, VertexUsageCode::Normal, 0, 0, 1);
    assert_eq!(
        [0, 0, 12, 4],
        [
            definition.items[0].offset,
            definition.items[1].offset,
            definition.items[2].offset,
            definition.items[3].offset
        ]
    );
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanSearchForVertexDefinitionItems",
    verification = "kani"
)]
fn can_search_for_vertex_definition_items() {
    let mut definition = Tr2VertexDefinition::default();
    definition.add_default(VertexDataType::FLOAT32_1, VertexUsageCode::Position);
    definition.add(VertexDataType::FLOAT32_2, VertexUsageCode::Position, 1, 0, 0);
    definition.add(VertexDataType::FLOAT32_3, VertexUsageCode::Normal, 0, 1, 0);

    let item = definition
        .find_default(VertexUsageCode::Position)
        .expect("position item");
    assert_eq!(VertexUsageCode::Position, item.usage);
    assert_eq!(0, item.usage_index);

    let item = definition
        .find(VertexUsageCode::Position, 1)
        .expect("position[1] item");
    assert_eq!(1, item.usage_index);
    assert!(definition.find_default(VertexUsageCode::Texcoord).is_none());
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanQueryVertexDefinitionItemSizeInMembers",
    verification = "kani"
)]
fn can_query_item_size_in_members() {
    assert_eq!(
        1,
        Tr2VertexDefinition::get_data_type_size_in_members(VertexDataType::FLOAT32_1)
    );
    assert_eq!(
        2,
        Tr2VertexDefinition::get_data_type_size_in_members(VertexDataType::FLOAT16_2)
    );
    assert_eq!(
        3,
        Tr2VertexDefinition::get_data_type_size_in_members(VertexDataType::UINT32_3)
    );
    assert_eq!(
        4,
        Tr2VertexDefinition::get_data_type_size_in_members(VertexDataType::FLOAT32_4)
    );
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanQueryVertexDefinitionItemSizeInBytes",
    verification = "kani"
)]
fn can_query_item_size_in_bytes() {
    assert_eq!(
        4,
        Tr2VertexDefinition::get_data_type_size_in_bytes(VertexDataType::FLOAT32_1)
    );
    assert_eq!(
        4,
        Tr2VertexDefinition::get_data_type_size_in_bytes(VertexDataType::FLOAT16_2)
    );
    assert_eq!(
        12,
        Tr2VertexDefinition::get_data_type_size_in_bytes(VertexDataType::UINT32_3)
    );
    assert_eq!(
        4,
        Tr2VertexDefinition::get_data_type_size_in_bytes(VertexDataType::BYTE_4)
    );
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanCompareVertexDefinitions",
    verification = "kani"
)]
fn can_compare_vertex_definitions() {
    let mut first = Tr2VertexDefinition::default();
    first.add_default(VertexDataType::FLOAT32_1, VertexUsageCode::Position);
    first.add(VertexDataType::FLOAT32_2, VertexUsageCode::Position, 1, 0, 0);
    first.add(VertexDataType::FLOAT32_3, VertexUsageCode::Normal, 0, 1, 0);

    let mut second = first.clone();
    assert_eq!(first, second);

    second.add(VertexDataType::FLOAT32_3, VertexUsageCode::Texcoord, 0, 2, 0);
    assert_ne!(first, second);
}

#[trinity_test(
    legacy = "trinityal/tests/VertexDefinition.cpp::VertexDefinition::CanMoveVertexDefinition",
    verification = "kani"
)]
fn can_move_vertex_definition() {
    let mut first = Tr2VertexDefinition::default();
    first.add_default(VertexDataType::FLOAT32_1, VertexUsageCode::Position);
    first.add(VertexDataType::FLOAT32_2, VertexUsageCode::Position, 1, 0, 0);
    first.add(VertexDataType::FLOAT32_3, VertexUsageCode::Normal, 0, 1, 0);

    let expected = first.clone();
    let moved = first;
    assert_eq!(expected, moved);
}
