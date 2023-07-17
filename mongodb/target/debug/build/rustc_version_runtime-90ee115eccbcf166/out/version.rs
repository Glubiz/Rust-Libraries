
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 71,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-pc-windows-msvc".to_owned(),
                    short_version_string: "rustc 1.71.0 (8ede3aae2 2023-07-12)".to_owned(),
                    commit_hash: Some("8ede3aae28fe6e4d52b38157d7bfe0d3bceef225".to_owned()),
                    commit_date: Some("2023-07-12".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            