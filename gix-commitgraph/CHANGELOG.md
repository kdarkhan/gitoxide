# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-265b8ec07fd5357df629f0d29fb2412d0186a287/> Add support for hashes of different size
   Even though right now, there is only Sha1, in future it's easy to
   support other hash sizes.

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-2ef9a8424af51310db8c1e6df31dde9953ed3d21/> Change accessors named `hash_kind()` to `object_hash()` for consistency

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 265 commits contributed to the release over the course of 911 calendar days.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#222](https://github.com/Byron/gitoxide/issues/222), [#279](https://github.com/Byron/gitoxide/issues/279), [#293](https://github.com/Byron/gitoxide/issues/293), [#329](https://github.com/Byron/gitoxide/issues/329), [#384](https://github.com/Byron/gitoxide/issues/384), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#63](https://github.com/Byron/gitoxide/issues/63), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Maintenance release note to avoid being fully generated ([`56ef363`](https://github.com/Byron/gitoxide/commit/56ef363f0e7a8b9106765d96d0636e38b2bed550))
    - changelog for git-commitgraph ([`d981f1f`](https://github.com/Byron/gitoxide/commit/d981f1f18ecbc943202702ab25a31a371a4b294d))
 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Also consider the size of the fanout table as part of the min size ([`8190708`](https://github.com/Byron/gitoxide/commit/8190708bc2b6ac9900d5f98b6c7db8cb3f38a632))
    - use latest capabilities of `git-hash` ([`a489ac2`](https://github.com/Byron/gitoxide/commit/a489ac2ca19a9fbf64f590c0d36c02b55c1a0536))
    - cargo fmt ([`8b9da35`](https://github.com/Byron/gitoxide/commit/8b9da35b3e0d3458efcac150f7062c9d7382a6c4))
    - Access pack-indices and pack-offsets of multi-pack indices ([`c2a6918`](https://github.com/Byron/gitoxide/commit/c2a69189f88c53ab555158245ce647fcd33fca6a))
    - adapt to changes in git-hash ([`5eb0230`](https://github.com/Byron/gitoxide/commit/5eb0230b58c25c0aa744eee0bd878dd91410dbe1))
    - Change accessors named `hash_kind()` to `object_hash()` for consistency ([`2ef9a84`](https://github.com/Byron/gitoxide/commit/2ef9a8424af51310db8c1e6df31dde9953ed3d21))
    - adjust to changes in git-hash ([`9bf25cc`](https://github.com/Byron/gitoxide/commit/9bf25cc4f2e44821f93e85997677bc4e86a67bd4))
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
    - Add support for hashes of different size ([`265b8ec`](https://github.com/Byron/gitoxide/commit/265b8ec07fd5357df629f0d29fb2412d0186a287))
    - refactor ([`501b85b`](https://github.com/Byron/gitoxide/commit/501b85b0ba57873f13e3086983d3b7a8b20defdd))
    - refactor ([`8c9c7fc`](https://github.com/Byron/gitoxide/commit/8c9c7fc3bc46afa9c8567a8bc8079cac12ed8422))
    - Use `git-chunk` crate for all chunk-related operations ([`0cd7f3b`](https://github.com/Byron/gitoxide/commit/0cd7f3b796fec9fe3eac953b6e56bd78b0ea18f9))
    - First round of introducing git-chunk ([`51b991b`](https://github.com/Byron/gitoxide/commit/51b991b2ca5727deb3447a51b14088dfdad8e7fe))
    - Adapt to latest changes to git-chunk ([`743d696`](https://github.com/Byron/gitoxide/commit/743d6967d6236a4bb6a9c8817f957e7604bc9264))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - remove byteorder dependency from git-commitgraph ([`c526811`](https://github.com/Byron/gitoxide/commit/c5268115d9193ba2e309a943ee1d3c9e5825562c))
    - use memmap2 in git-commitgraph ([`0c946f5`](https://github.com/Byron/gitoxide/commit/0c946f5cb9d6eb13615b6c3d1a7b479ab5874441))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com/Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - git-commitgraph with a more convenient public interface with AsRef ([`ba04e4e`](https://github.com/Byron/gitoxide/commit/ba04e4ed673c654a8968532228571a93a3ebc8e2))
    - git-commitgraph uses `oid` now ([`0b72966`](https://github.com/Byron/gitoxide/commit/0b72966249523b97fce1bc7b29082ac68fa86a4f))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com/Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com/Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
    - Remove re-export of git_object::borrowed::Id ([`a3f2816`](https://github.com/Byron/gitoxide/commit/a3f28169c1268c1129852f279631d5a7f7540cdf))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - rename `git-commitgraph` to `gix-commitgraph` ([`21077da`](https://github.com/Byron/gitoxide/commit/21077da34c4c4c8adb2b58b8b7702de832a895a6))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - adjust to renaming of `git-lfs` to `gix-lfs` ([`b9225c8`](https://github.com/Byron/gitoxide/commit/b9225c830daf1388484ee7e05f727990fdeff43c))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
    - Release git-commitgraph v0.13.0, gitoxide-core v0.23.0, gitoxide v0.21.0 ([`230a11f`](https://github.com/Byron/gitoxide/commit/230a11f8fb9625587f7a1ce0911e54f0d8579fd6))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Optimize usage of `hex_to_id()` ([`6fa950d`](https://github.com/Byron/gitoxide/commit/6fa950d0ab1991a5577c06385169be1b390dd88a))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Release git-features v0.25.1, git-url v0.12.2, git-odb v0.38.1, git-transport v0.24.2, git-repository v0.30.2 ([`bb0a07b`](https://github.com/Byron/gitoxide/commit/bb0a07b5edd5f980989d1a92e74df7f183febe87))
    - Release git-url v0.12.1, git-transport v0.24.1, git-protocol v0.25.1, git-repository v0.30.1, git-commitgraph v0.12.0, gitoxide-core v0.22.0, gitoxide v0.20.0 ([`08ec3a9`](https://github.com/Byron/gitoxide/commit/08ec3a93d77a1018439a5c41c23729ffed27c5a5))
    - prepare changelogs prior to release ([`68ce15d`](https://github.com/Byron/gitoxide/commit/68ce15d07b50cfacdac0d1e42fe7f5e6330ba523))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Merge branch 'fix-ci-installation' ([`9245083`](https://github.com/Byron/gitoxide/commit/92450839621a4d99cb22d08cbf9f9a89ff6b9e3f))
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-commitgraph v0.8.1, gitoxide-core v0.16.0, gitoxide v0.14.0 ([`183c048`](https://github.com/Byron/gitoxide/commit/183c0488a808ef760a9f6795f5c040e73926c3a8))
    - prepare for gitoxide release ([`6305d52`](https://github.com/Byron/gitoxide/commit/6305d52236c094c412b221967f59eb264c2c3038))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`d4df661`](https://github.com/Byron/gitoxide/commit/d4df661dbf60dad75d07002ef9979cabe8a86935))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-commitgraph v0.7.0, gitoxide-core v0.13.0, gitoxide v0.11.0 ([`ab08a7f`](https://github.com/Byron/gitoxide/commit/ab08a7f066fb65671868424315d958ae985d76d8))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - thanks clippy ([`53bd30f`](https://github.com/Byron/gitoxide/commit/53bd30fd56c971b2be5a5d22045b97dc5f216303))
    - thanks clippy ([`6cc1bd1`](https://github.com/Byron/gitoxide/commit/6cc1bd15a49d9ec67a4a381ee3f64d557850733c))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Release git-commitgraph v0.5.0, gitoxide-core v0.11.0, gitoxide v0.9.0 ([`960eb0e`](https://github.com/Byron/gitoxide/commit/960eb0e5e5a7df117ed2ae2a8e2ec167b074c332))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Release git-commitgraph v0.4.4 ([`dec935c`](https://github.com/Byron/gitoxide/commit/dec935cd6ef9a70afd247e5fcf44983c97c1b10b))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - Release git-commitgraph v0.4.3 ([`7dfe16b`](https://github.com/Byron/gitoxide/commit/7dfe16bdebaf971b7101331ad037d1ca8ab491d2))
    - [repository #165] refactor ([`1547d0b`](https://github.com/Byron/gitoxide/commit/1547d0b062e35bad2229dac532e6f30bf105db73))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
    - Release git-commitgraph v0.4.2 ([`847c456`](https://github.com/Byron/gitoxide/commit/847c4564d9b64c071db790979654d0883d7a38d0))
    - Promote file-format constants to `git_commitgraph::file` module. ([`0afd354`](https://github.com/Byron/gitoxide/commit/0afd354f94fb1829d4c097b49cba503bac3d1c38))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - Release git-commitgraph v0.4.1 ([`1776a0d`](https://github.com/Byron/gitoxide/commit/1776a0d7168f1f15a18e0f873a9918a6db33b94a))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - change wording ([`6c82a16`](https://github.com/Byron/gitoxide/commit/6c82a16d340acb9b11c5cf56c917c9fe6f2cdf0e))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com/Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com/Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - fix git-commigraph build (broke after git-hash changed its ways) ([`08fd7a0`](https://github.com/Byron/gitoxide/commit/08fd7a08800d926bcfeb1cfe6faa1f02c0b8904e))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com/Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Put 'sha1' behind a feature toggle ([`4f326bc`](https://github.com/Byron/gitoxide/commit/4f326bc261c4e7f0d5510df74ad4215da3580696))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com/Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - prepare test utilities for release… ([`d35e654`](https://github.com/Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - Revert "FAIL: try to disable GPG signing with environment variables…" ([`e326352`](https://github.com/Byron/gitoxide/commit/e326352eec7bd1aae13f770328979e5730ffc32b))
    - try to disable GPG signing with environment variables… ([`29bf8ca`](https://github.com/Byron/gitoxide/commit/29bf8ca8399b6d4941aa242b9f08c74e59a179bb))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com/Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - faster repeated tests if fixtures don't change ([`792277f`](https://github.com/Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - git-commitgraph uses test-tools ([`5d30e5a`](https://github.com/Byron/gitoxide/commit/5d30e5a3474aabd67cb5d1afc826aa68957d2b7a))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com/Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
    - (cargo-release) version 0.4.0 ([`06612eb`](https://github.com/Byron/gitoxide/commit/06612eb12d4679bec7dae08a511dd87d80087151))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com/Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - (cargo-release) version 0.3.2 ([`d91dd9d`](https://github.com/Byron/gitoxide/commit/d91dd9d8c57688dc9c420460ef5800cd07b3c9b4))
    - Merge pull request #43 from avoidscorn/docs ([`1469be4`](https://github.com/Byron/gitoxide/commit/1469be45240126f855c9fcc2a72647e319963ef7))
    - [commitgraph] Tweak and expand documentation. ([`ac52867`](https://github.com/Byron/gitoxide/commit/ac5286772c0eefd994b3d85ab185e0d4960cdd0a))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com/Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
    - (cargo-release) version 0.3.1 ([`89db50c`](https://github.com/Byron/gitoxide/commit/89db50ce01ea7cc83b7f90484e2f8736dba7ccde))
    - remaining docs for git-commitgraph crate ([`9146176`](https://github.com/Byron/gitoxide/commit/91461760884979218617fcfdc56efd8be73b9d6f))
    - more commitgraph docs ([`a81ea67`](https://github.com/Byron/gitoxide/commit/a81ea6730f11f769caed9a70cad123cace96b625))
    - all docs for git-commitgraph::file ([`8b26201`](https://github.com/Byron/gitoxide/commit/8b262011ceffaff74bea9f4ffc730682884fff64))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com/Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
    - (cargo-release) version 0.2.0 ([`d61ad88`](https://github.com/Byron/gitoxide/commit/d61ad884021d3c0a61a14ba1df4daadfa1a0b561))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com/Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com/Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - (cargo-release) version 0.1.3 ([`a833fd1`](https://github.com/Byron/gitoxide/commit/a833fd18e1bc3a501e4f1ed66506f48673f79590))
    - thanks clippy ([`ba9b3c2`](https://github.com/Byron/gitoxide/commit/ba9b3c2345887353e02fc081be80733f1c5e22d9))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com/Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com/Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - Merge branch 'commit-graph' into main ([`9cb09b2`](https://github.com/Byron/gitoxide/commit/9cb09b248796f0ff5c9d3f3e857de4731324cfd5))
    - Note about why git_features::hash::bytes_of_file() is not yet used ([`ca48fc4`](https://github.com/Byron/gitoxide/commit/ca48fc4f7c00215acf95370fe894a6e585c18c13))
    - Add and use borrowed::Id::null_sha1() ([`c717492`](https://github.com/Byron/gitoxide/commit/c717492d0038f55a6f21b48937b56a756890d214))
    - refactor ([`e4935e0`](https://github.com/Byron/gitoxide/commit/e4935e03040e1f4ded652ed43a1e0177eefb44f4))
    - replace 'ImpossibleVariantError' with 'std::convert::Infallible'` ([`c53638c`](https://github.com/Byron/gitoxide/commit/c53638ccd9e392af839b7eb03826fa6aab94faff))
    - [commitgraph] Clean up `{file,graph}::verify::Error` types. ([`fa22cab`](https://github.com/Byron/gitoxide/commit/fa22cab259338dc140dd660f4f4b9bbc9d6cc3d0))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com/Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Loosen lifetime restrictions on return values. ([`701f33c`](https://github.com/Byron/gitoxide/commit/701f33c06b80deaabe7625b01d36e2a1b1af3a78))
    - [commitgraph] Replace `T as U` with `U::from(T)` or `t.try_into()`. ([`28f94b4`](https://github.com/Byron/gitoxide/commit/28f94b4bccdf317c9f4ccb62e0e3f3314f3995c9))
    - [commitgraph] Tweak `File::iter_base_graph_ids` implementation. ([`5b06780`](https://github.com/Byron/gitoxide/commit/5b067808a793e3515c0c12cf95c11b57beaa8d09))
    - [commitgraph] Add `Graph::at` constructor. ([`a783052`](https://github.com/Byron/gitoxide/commit/a783052d0cc2d3c9fa1dda3ea77286a79690d2c1))
    - [commitgraph] Validate trailer section when parsing files. ([`1b738ac`](https://github.com/Byron/gitoxide/commit/1b738ac0719ec20b24982d148a386d63ec4dc2d6))
    - [commitgraph] Use `thiserror` instead of `quick_error`. ([`c8b1f74`](https://github.com/Byron/gitoxide/commit/c8b1f74328965708e38a689b865660ad36f22ecb))
    - (cargo-release) version 0.1.2 ([`b401468`](https://github.com/Byron/gitoxide/commit/b40146828771d9837350e07250fb21851f700fcc))
    - Merge remote-tracking branch 'origin/main' into main ([`f3d90d7`](https://github.com/Byron/gitoxide/commit/f3d90d7f65cdbcfed4281c0382f8c6766809afaa))
    - (cargo-release) version 0.1.1 ([`04c7cdf`](https://github.com/Byron/gitoxide/commit/04c7cdf1418f43052390f5d67bd4e7e43ae8b2e6))
    - Fix repository URL ([`d721f47`](https://github.com/Byron/gitoxide/commit/d721f478ab441db30585af747d9f47717443d7e1))
    - update commitgraph information ([`275cfde`](https://github.com/Byron/gitoxide/commit/275cfde06192c8b3a3d633b21e970b54ddc1a53f))
    - [commitgraph] add size limit and prep for release ([`4eabf55`](https://github.com/Byron/gitoxide/commit/4eabf554dc7cc08416d1078fa29db606455dc031))
    - [commitgraph] bump minor version for first release ([`76bb4d3`](https://github.com/Byron/gitoxide/commit/76bb4d355dd1570340fe7d05d2a3378e15a36d4e))
    - [commitgraph] refactor file::init ([`8b003a0`](https://github.com/Byron/gitoxide/commit/8b003a01729e4bfcb433e34f32b8e450cbe75fea))
    - [commitgraph] refactor ([`c4b14c1`](https://github.com/Byron/gitoxide/commit/c4b14c1eae8dfcdcb3637d64e3c81dc424e26607))
    - [commitgraph] Rename LexPosition to 'file::Position' ([`6f90bee`](https://github.com/Byron/gitoxide/commit/6f90beeb418480f9cd8bb7ae3b5db678b24103cb))
    - [commitgraph] refactor graph::init module ([`d2eec1d`](https://github.com/Byron/gitoxide/commit/d2eec1dbedac6e87cc281cdd84423d9c7cfba323))
    - [commitgraph] refactor Graph, Position, and access module ([`3c8640e`](https://github.com/Byron/gitoxide/commit/3c8640e5baf4729f4394c569dc0aed9865121e7a))
    - [commitgraph] refactor ([`2ed0037`](https://github.com/Byron/gitoxide/commit/2ed0037c87fa17fbdb560cab46f72bf64805623b))
    - [commitgraph] refactor ([`7026961`](https://github.com/Byron/gitoxide/commit/7026961ab7de4ee66ae84bdfdeef359ae960d231))
    - [commitgraph] Assure git doesn't try to sign commits when fixtures are created ([`9ae1f4b`](https://github.com/Byron/gitoxide/commit/9ae1f4b9bb05a19ba279a1242f3c84d439421f18))
    - [commitgraph] Attempt to fix bash script execution on Windows. ([`5e78213`](https://github.com/Byron/gitoxide/commit/5e78213b1cd53986b8a39accf17da3456e496016))
    - [commitgraph] Use crate::graph::Graph instead of crate::Graph. ([`21e4527`](https://github.com/Byron/gitoxide/commit/21e45275221505b30f466a3b0223534d5a2281e5))
    - [commitgraph] Rearrange some `use` statements. ([`185d14b`](https://github.com/Byron/gitoxide/commit/185d14b25b8fc85308b1ba62391595dda51ce58a))
    - [commitgraph] Don't export Commit symbol at crate level. ([`be0e845`](https://github.com/Byron/gitoxide/commit/be0e845649b87acd3197ea212c78af8e0f9e22bf))
    - [commitgraph] Include Conor in crate manifest. ([`000748c`](https://github.com/Byron/gitoxide/commit/000748ccffc222729a7a1c1ce19c4fa1ba50fbed))
    - [commitgraph] Add some doc comments. ([`6cf5cd8`](https://github.com/Byron/gitoxide/commit/6cf5cd8da54e9d5670e3a44de95253df1091b110))
    - [commitgraph] Remove unused error variant. ([`66588f2`](https://github.com/Byron/gitoxide/commit/66588f227de8fd883a5f429821509e968c59b4fc))
    - [commitgraph] Rename GraphFile -> File. ([`f451822`](https://github.com/Byron/gitoxide/commit/f451822ec912253b2e5a5b0a63e1abd76939f58d))
    - [commitgraph] Rename CommitData -> Commit. ([`d8c2007`](https://github.com/Byron/gitoxide/commit/d8c20072fdce7cba249f4d9b5a0cba6136beb06f))
    - [commitgraph] Don't re-export graph_file symbols at crate level. ([`7c405ab`](https://github.com/Byron/gitoxide/commit/7c405aba660537999a24b6824198b3afb6dde529))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - [commitgraph] Ditch pre-generated test repos. ([`1ce8468`](https://github.com/Byron/gitoxide/commit/1ce84689ee89eb0f9e4f57cdba3a5ccac4a1a12d))
    - [commitgraph] Remove `Kind` enum. ([`3c92761`](https://github.com/Byron/gitoxide/commit/3c927610eb717645e7f83a257184e44f76918571))
    - [commitgraph] Take `info` dir as arg, not `objects` dir. ([`36953e0`](https://github.com/Byron/gitoxide/commit/36953e0ec6119e1a01ae9b8e46e40bbd083e732c))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
    - [commitgraph] implement basic, low-level read API ([`d1f0e9c`](https://github.com/Byron/gitoxide/commit/d1f0e9cbd259b460a7d12ae068fb95ede0000cb2))
    - Reorganize git-commitgraph goals; add crate ([`21c9b75`](https://github.com/Byron/gitoxide/commit/21c9b7500cb144b3169a6537961ec2b9e865be81))
</details>

## 0.13.0 (2023-02-09)

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

## 0.12.0 (2022-12-22)

A maintenance release without user-facing changes.

## 0.11.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.10.0 (2022-11-06)

A maintenance release without user-facing changes.

## 0.9.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.8.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.8.1 (2022-08-17)

A maintenance release without user-facing changes.

## 0.8.0 (2022-07-22)

A maintenance release without user-facing changes.

## 0.7.0 (2022-04-03)

A maintenance release, triggered by putting too many adjustments into a single commit.

### Changed (BREAKING)

 - <csr-id-2ef9a8424af51310db8c1e6df31dde9953ed3d21/> Change accessors named `hash_kind()` to `object_hash()` for consistency

### New Features

 - <csr-id-265b8ec07fd5357df629f0d29fb2412d0186a287/> Add support for hashes of different size
   Even though right now, there is only Sha1, in future it's easy to
   support other hash sizes.

## v0.6.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `gix-hash`.

## v0.5.0 (2021-10-15)

This is a maintenance release without functional changes.

## v0.4.4 (2021-09-07)

## v0.4.3 (2021-08-29)

## v0.4.2 (2021-08-17)

## v0.4.1 (2021-08-15)

<csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/>

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

## v0.4.0 (2021-04-08)

## v0.3.2 (2021-02-11)

## v0.3.1 (2021-01-09)

## v0.3.0 (2020-12-16)

## v0.2.0 (2020-12-15)

## v0.1.2 (2020-10-01)

## v0.1.1 (2020-10-01)

## v0.1.0 (2020-10-01)

## v0.0.0 (2020-08-20)
