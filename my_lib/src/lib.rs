// lib.rs에 module을 publishing 하면 crate를 만들 수 있고 외부에서 참조 가능

pub mod guess_game;                 // 2. 추리 게임 튜토리얼
pub mod common;                     // 3. 보편적인 프로그래밍 개념
pub mod ownership;                  // 4. 소유권 이해하기
pub mod structures;                 // 5. 연관된 데이터들을 구조체로 다루기
pub mod enum_match;                 // 6. 열거형과 패턴 매칭
pub mod modules;                    // 7. 모듈
pub mod collections;                // 8. 일반적인 컬렉션
pub mod error;                      // 9. 에러 처리
pub mod generic_traits_lifetime;    // 10. 제너릭 타입, 트레잇, 그리고 라이프타임
pub mod testing;                    // 11. 테스팅
pub mod command_program;            // 12. I/O 프로젝트: 커맨드 라인 프로그램 만들기
pub mod functional;                 // 13. 함수형 언어의 특성들: 반복자들과 클로저들
pub mod cargo_crates;               // 14. Cargo와 Crates.io 더 알아보기
pub mod smart_pointer;              // 15. 스마트 포인터
pub mod concurrency;                // 16. 겁없는 동시성
pub mod oop;                     // 17. 러스트의 객체 지향 프로그래밍
pub mod pattern_match;           // 18. 값의 구조와 매칭되는 패턴
pub mod advanced;                // 19. 고급 기능들
pub mod webserver;              // 20. 마지막 프로젝트: 멀티스레드 웹서버 만들기