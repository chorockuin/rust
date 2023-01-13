extern crate my_lib;
mod common; // integration test에서 공통으로 사용하는 모듈

#[test]
fn integration_test_my_lib_testing_sample() {
    common::setup();
    assert_eq!(1024, my_lib::testing::sample());
}