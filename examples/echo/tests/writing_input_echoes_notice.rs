use cartesi_rollups_test::{Notice, TestMachineIo};

#[test]
fn test_writing_input_echoes_notice() {
    let (actual_notices, ..) = TestMachineIo::default()
        .write_input("hello")
        .write_input("world")
        .process();

    let expected_notices = vec![
        Notice::new("hello").with_input_index(1),
        Notice::new("world").with_input_index(2),
    ];

    assert_eq!(expected_notices, actual_notices);
}
