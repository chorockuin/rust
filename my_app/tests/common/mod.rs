// integration test에서 공통을 사용해야 하는 함수들(예를들면 환경 setup 같은)은
// 이렇게 원하는 이름으로 폴더를 따로 만들어서 mod.rs 파일을 넣으면 module 처럼 사용할 수 있다
pub fn setup() {
    println!("setup test environment...");
}
