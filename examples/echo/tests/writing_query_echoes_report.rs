use cartesi_rollups_test::{Report, TestMachineIo};

#[test]
fn test_writing_query_echoes_report() {
    let (.., actual_reports) = TestMachineIo::default()
        .write_query("hello")
        .process();

    let expected_reports = vec![
        Report::new("hello"),
    ];

    assert_eq!(expected_reports, actual_reports);
}
