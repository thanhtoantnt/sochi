//! Test Verazt utilities

// #[cfg(test)]
// use llvm::assert_stdout_eq;
// use llvm::util::report;

// TODO: how to configure `cargo test` to run the `--nocapture` flag
// only for some test cases.
// #[test]
// fn test_print_code_fragment() {
//     use std::io::Write;
//     use tempfile::NamedTempFile;

//     let input_str = [
//         "int foo() {",
//         "  int k = 0;",
//         "  while (k < 100) {",
//         "    int i = 0;",
//         "    int j = k;",
//         "    while (i < j) {",
//         "      i = i + 1;",
//         "      j = j - 1;",
//         "    }",
//         "    k = k + 1;",
//         "  }",
//         "  return k;",
//         "}",
//     ]
//     .join("\n");

//     let mut file =
//         NamedTempFile::new().expect("Cannot create file for testing");
//     write!(file, "{}", input_str).expect("Cannot write file for testing");

//     let file_path = file.path().to_str().unwrap_or("");

//     let expected_str = [
//         "Code segment:",
//         "  5|    int j = k;",
//         "  6|    while (i < j) {",
//         "  7|>      i = i + 1;",
//         "   |>           ^^^",
//         "  8|      j = j - 1;",
//         "  9|    }",
//         "Location: 7:13",
//         format!("File: {file_path}\n").as_str(),
//     ]
//     .join("\n");

//     assert_stdout_eq!(
//         report::print_code_fragment_and_position(file_path, 7, 13),
//         expected_str
//     );

//     file.close().expect("Cannot remove test data file");
// }
