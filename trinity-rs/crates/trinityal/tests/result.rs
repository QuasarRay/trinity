use trinity_macros::trinity_test;
use trinityal::{
    ALResult, ALResultCategory, E_DEVICELOST, E_INVALIDARG, E_INVALIDCALL, E_OUTOFMEMORY, S_OK,
    be_get_error_message, be_is_success,
};

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::DefaultResultIsSuccess",
    verification = "kani"
)]
fn default_result_is_success() {
    assert_eq!(S_OK, ALResult::default().get_result());
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanCreateALResultFromHRESULT",
    verification = "kani"
)]
fn can_create_al_result_from_hresult() {
    assert_eq!(E_INVALIDARG, ALResult::new(E_INVALIDARG).get_result());
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanCopyResult",
    verification = "kani"
)]
fn can_copy_result() {
    let first = ALResult::new(E_INVALIDARG);
    let second = first;
    assert_eq!(first, second);
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanAssignResult",
    verification = "kani"
)]
fn can_assign_result() {
    let source = ALResult::new(E_INVALIDARG);
    let mut destination = ALResult::default();
    assert_eq!(S_OK, destination.get_result());
    destination = source;
    assert_eq!(source, destination);

    let source = ALResult::new(E_INVALIDARG);
    let mut destination = ALResult::default();
    assert_eq!(S_OK, destination.get_result());
    destination = source;
    assert_eq!(source, destination);
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanCastALResultToHRESULT",
    verification = "kani"
)]
fn can_cast_al_result_to_hresult() {
    let ok: i32 = ALResult::default().into();
    let bad: i32 = ALResult::new(E_INVALIDCALL).into();
    assert_eq!(S_OK, ok);
    assert_eq!(E_INVALIDCALL, bad);
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanQueryResultCategory",
    verification = "kani"
)]
fn can_query_result_category() {
    assert_eq!(ALResultCategory::Success, ALResult::default().get_category());
    assert_eq!(
        ALResultCategory::InvalidCall,
        ALResult::new(E_INVALIDARG).get_category()
    );
    assert_eq!(
        ALResultCategory::OutOfMemory,
        ALResult::new(E_OUTOFMEMORY).get_category()
    );
    assert_eq!(
        ALResultCategory::DeviceLost,
        ALResult::new(E_DEVICELOST).get_category()
    );
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanCheckIfResultIsSuccessful",
    verification = "kani"
)]
fn can_check_if_result_is_successful() {
    assert!(be_is_success(ALResult::default()));
    assert!(!be_is_success(ALResult::new(E_INVALIDCALL)));
}

#[trinity_test(
    legacy = "trinityal/tests/ALResultTest.cpp::ALResult::CanGetErrorMessage",
    verification = "kani"
)]
fn can_get_error_message() {
    assert!(!be_get_error_message(ALResult::new(E_INVALIDARG)).is_empty());
}
