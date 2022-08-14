use crate::fixture_index_path;
use crate::index::file::read::loose_file_path;
use filetime::FileTime;
use git_index::verify::extensions::no_find;
use git_index::write::Options;
use git_index::{decode, entry, extension, write, State, Version};

/// Round-trips should eventually be possible for all files we have, as we write them back exactly as they were read.
#[test]
fn roundtrips() -> crate::Result {
    enum Kind {
        Generated(&'static str),
        Loose(&'static str),
    }
    use Kind::*;
    let input = [
        (Loose("extended-flags"), all_ext_but_eoie()),
        (Loose("conflicting-file"), all_ext_but_eoie()),
        (Loose("very-long-path"), all_ext_but_eoie()),
        (Generated("v2"), write::Options::default()),
        (Generated("V2_empty"), write::Options::default()),
        (Generated("v2_more_files"), all_ext_but_eoie()),
    ];

    for (fixture, options) in input {
        let (path, fixture) = match fixture {
            Generated(name) => (fixture_index_path(name), name),
            Loose(name) => (loose_file_path(name), name),
        };
        let expected = git_index::File::at(&path, decode::Options::default())?;
        let expected_bytes = std::fs::read(&path)?;
        let mut out_bytes = Vec::new();

        let actual_version = expected.write_to(&mut out_bytes, options)?;
        assert_eq!(
            actual_version,
            expected.version(),
            "{} didn't write the expected version",
            fixture
        );
        let (actual, _) = State::from_bytes(&out_bytes, FileTime::now(), decode::Options::default())?;

        compare_states(&actual, actual_version, &expected, options, fixture);
        compare_raw_bytes(&out_bytes, &expected_bytes, fixture);
    }
    Ok(())
}

#[test]
fn extended_flags_automatically_upgrade_the_version_to_avoid_data_loss() -> crate::Result {
    let mut expected = git_index::File::at(fixture_index_path("V2"), Default::default())?;
    assert_eq!(expected.version(), Version::V2);
    expected.entries_mut()[0].flags.insert(entry::Flags::EXTENDED);

    let mut buf = Vec::new();
    let actual_version = expected.write_to(&mut buf, Default::default())?;
    assert_eq!(actual_version, Version::V3, "extended flags need V3");

    Ok(())
}

#[test]
fn v2_index_no_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            extensions: write::Extensions::None,
        };

        let actual_version = expected.write_to(&mut out, options).unwrap();

        let (actual, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&actual, actual_version, &expected, options, fixture);
    }
}

#[test]
fn v2_index_tree_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = all_ext_but_eoie();

        let actual_version = expected.write_to(&mut out, options).unwrap();

        let (actual, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&actual, actual_version, &expected, options, fixture);
    }
}

#[test]
fn v2_index_eoie_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            extensions: write::Extensions::Given {
                tree_cache: false,
                end_of_index_entry: true,
            },
        };

        let version = expected.write_to(&mut out, options).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, version, &expected, options, fixture);
    }
}

fn compare_states(actual: &State, actual_version: Version, expected: &State, options: write::Options, fixture: &str) {
    actual.verify_entries().expect("valid");
    actual.verify_extensions(false, no_find).expect("valid");

    assert_eq!(actual.version(), actual_version, "version mismatch in {}", fixture);
    assert_eq!(
        actual.tree(),
        options
            .extensions
            .should_write(extension::tree::SIGNATURE)
            .and_then(|_| expected.tree()),
        "tree extension mismatch in {}",
        fixture
    );
    assert_eq!(
        actual.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {}",
        fixture
    );
    assert_eq!(actual.entries(), expected.entries(), "entries mismatch in {}", fixture);
    assert_eq!(
        actual.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {}",
        fixture
    );
}

fn compare_raw_bytes(generated: &[u8], expected: &[u8], fixture: &str) {
    assert_eq!(generated.len(), expected.len(), "file length mismatch in {}", fixture);

    let print_range = 10;
    for (index, (a, b)) in generated.iter().zip(expected.iter()).enumerate() {
        if a != b {
            let range_left = index.saturating_sub(print_range);
            let range_right = (index + print_range).min(generated.len());
            let generated = &generated[range_left..range_right];
            let expected = &expected[range_left..range_right];

            panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
            \t  Actual: ... {:?} ...\n\
            \tExpected: ... {:?} ...\n\n\
            ", &fixture, index, generated, expected}
        }
    }
}

fn all_ext_but_eoie() -> Options {
    write::Options {
        extensions: write::Extensions::Given {
            end_of_index_entry: false,
            tree_cache: true,
        },
        ..write::Options::default()
    }
}
