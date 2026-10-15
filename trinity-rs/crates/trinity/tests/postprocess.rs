use trinity_macros::trinity_test;
use trinity_rs::{Tr2PPExecutionSlot, Tr2PPGenericEffect, Tr2PPParameter};

fn effect(blendable_value: u32, fixed_value: u32) -> Tr2PPGenericEffect {
    Tr2PPGenericEffect {
        effect: "res:/post/test.fx".to_owned(), quality: 2,
        execution_slot: Tr2PPExecutionSlot::BeforeUpscaling, order: 0,
        parameters: vec![Tr2PPParameter { name: "Blend".to_owned(), value_bits: blendable_value, blendable: true }, Tr2PPParameter { name: "Fixed".to_owned(), value_bits: fixed_value, blendable: false }],
    }
}
#[trinity_test(legacy = "doc/source/eve/generic-pp-effects.rst::identical-effects-group", verification = "kani")]
fn blendable_parameters_do_not_split_groups() { assert!(effect(1,9).can_group_with(&effect(99,9))); }
#[trinity_test(legacy = "doc/source/eve/generic-pp-effects.rst::non-blendable-parameters-split-groups", verification = "kani")]
fn non_blendable_parameters_split_groups() { assert!(!effect(1,9).can_group_with(&effect(1,10))); }
