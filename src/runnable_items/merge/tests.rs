use super::*;
use std::path::Path;

const NO_INPUT: [&str; 3] = ["merge", "-o", "output.pdf"];
const NO_OUTPUT: [&str; 3] = ["merge", "-i", "input.pdf"];
const UNPARSEABLE_DEPTH: [&str; 7] = ["merge", "-i", "inputs", "-o", "some.pdf", "-d", "0"];
const UNPARSEABLE_ORDER_MODE: [&str; 7] = [
    "merge",
    "-i",
    "dir",
    "-o",
    "some.pdf",
    "--order-by",
    "invalid",
];
const SINGLE_FILE_INPUT: [&str; 5] = ["merge", "-i", "f.pdf", "-o", "out.pdf"];
const OUTPUT_IS_DIR: [&str; 6] = ["merge", "-i", "file.pdf", "other.pdf", "-o", "src"];
const INPUT_DIRECTORY_REFERENCE: [&str; 6] =
    ["merge", "-i", "somedir/../this.pdf", "dir", "-o", "out.pdf"];
const TOML_INPUT: [&str; 6] = ["merge", "-i", "pdf.pdf", "Cargo.toml", "-o", "out.pdf"];
const INPUT_REPETITION: [&str; 6] = ["merge", "-i", "src", "src", "-o", "out.pdf"];
const ALREADY_EXISTING_OUTPUT: [&str; 5] = ["merge", "-i", "src", "-o", "f.pdf"];
const PARENT_OUTPUT: [&str; 5] = ["merge", "-i", "src", "-o", "some/f.pdf"];

#[test]
fn merge_try_from() {
    [
        (
            MergeArgs::from_iter(NO_INPUT),
            MergeBuildError::InputIsEmpty,
        ),
        (
            MergeArgs::from_iter(NO_OUTPUT),
            MergeBuildError::OutputIsEmpty,
        ),
        (
            MergeArgs::from_iter(UNPARSEABLE_DEPTH),
            MergeBuildError::UnparseableDepth("0".into()),
        ),
        (
            MergeArgs::from_iter(UNPARSEABLE_ORDER_MODE),
            MergeBuildError::UnparseableOrderMode("invalid".into()),
        ),
    ]
    .into_iter()
    .for_each(|(m, err_variant)| assert!(Merge::try_from(m).is_err_and(|e| e == err_variant)));
}

#[test]
fn merge_check() {
    match Path::new("f.pdf").try_exists() {
        Ok(false) => {
            panic!(
                "this test requires a `f.pdf` at crate root. create it and try again...\n(can be empty)"
            )
        }
        Err(_) => panic!("failed to check the `f.pdf` file. This is an unexpected behavior"),
        _ => {}
    }
    [
        (
            MergeArgs::from_iter(SINGLE_FILE_INPUT),
            MergeCheckError::InputIsSingleFile("f.pdf".into()),
        ),
        (
            MergeArgs::from_iter(OUTPUT_IS_DIR),
            MergeCheckError::OutputIsDirectory("src".into()),
        ),
        (
            MergeArgs::from_iter(INPUT_DIRECTORY_REFERENCE),
            MergeCheckError::InputIsDirectoryReference("somedir/../this.pdf".into()),
        ),
        (
            MergeArgs::from_iter(TOML_INPUT),
            MergeCheckError::InputIsNotPdfFile("Cargo.toml".into()),
        ),
        (
            MergeArgs::from_iter(INPUT_REPETITION),
            MergeCheckError::InputRepetitionWithoutFlag("src".into()),
        ),
        (
            MergeArgs::from_iter(ALREADY_EXISTING_OUTPUT),
            MergeCheckError::OutputAlreadyExists("f.pdf".into()),
        ),
        (
            MergeArgs::from_iter(PARENT_OUTPUT),
            MergeCheckError::ParentOutputWithoutFlag("some/f.pdf".into()),
        ),
    ]
    .into_iter()
    .for_each(|(m, err)| {
        let m = Merge::try_from(m.clone())
            .unwrap_or_else(|_| panic!("This was expected to be OK {:?}", m));
        assert_eq!(m.check_item(), Err(err))
    });
}
