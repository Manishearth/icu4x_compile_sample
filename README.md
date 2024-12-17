# icu4x_compile_sample

See: https://github.com/unicode-org/icu4x/issues/5230

```
$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
     Removed 8 files, 50.3KiB total
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling demo2 v0.1.0 (/home/manishearth/dev/Git/icu4x_compile_sample/demo)
time:   0.001; rss:   48MB ->   49MB (   +1MB)	parse_crate
time:   0.000; rss:   50MB ->   50MB (   +0MB)	incr_comp_prepare_session_directory
time:   0.000; rss:   50MB ->   51MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   52MB ->   52MB (   +0MB)	crate_injection
time:   0.715; rss:   52MB ->  254MB ( +201MB)	expand_crate
time:   0.715; rss:   52MB ->  254MB ( +201MB)	macro_expand_crate
time:   0.012; rss:  254MB ->  254MB (   +0MB)	AST_validation
time:   0.000; rss:  254MB ->  254MB (   +0MB)	compute_effective_visibilities
time:   0.001; rss:  254MB ->  256MB (   +2MB)	finalize_macro_resolutions
time:   0.431; rss:  256MB ->  304MB (  +48MB)	late_resolve_crate
time:   0.011; rss:  304MB ->  304MB (   +0MB)	resolve_check_unused
time:   0.020; rss:  304MB ->  304MB (   +0MB)	resolve_postprocess
time:   0.463; rss:  254MB ->  304MB (  +50MB)	resolve_crate
time:   0.012; rss:  304MB ->  304MB (   +0MB)	write_dep_info
time:   0.011; rss:  305MB ->  305MB (   +0MB)	complete_gated_feature_checking
time:   0.057; rss:  408MB ->  356MB (  -53MB)	drop_ast
time:   0.714; rss:  304MB ->  320MB (  +16MB)	looking_for_derive_registrar
time:   0.913; rss:  304MB ->  321MB (  +17MB)	misc_checking_1
time:   0.070; rss:  322MB ->  329MB (   +8MB)	coherence_checking
time:   3.728; rss:  321MB ->  576MB ( +255MB)	type_check_crate
time:  43.933; rss:  576MB ->  955MB ( +379MB)	MIR_borrow_checking
time:   0.896; rss:  955MB ->  953MB (   -2MB)	MIR_effect_checking
time:   0.127; rss:  953MB ->  953MB (   +0MB)	module_lints
time:   0.128; rss:  953MB ->  953MB (   +0MB)	lint_checking
time:   0.195; rss:  953MB ->  953MB (   +0MB)	privacy_checking_modules
time:   0.378; rss:  953MB ->  953MB (   +0MB)	misc_checking_3
time:   0.000; rss:  954MB ->  954MB (   +1MB)	monomorphization_collector_graph_walk
time:   0.155; rss:  953MB ->  954MB (   +1MB)	generate_crate_metadata
time:   0.006; rss:  967MB ->  974MB (   +7MB)	LLVM_passes
time:   0.001; rss:  955MB ->  974MB (  +19MB)	codegen_to_LLVM_IR
time:   0.010; rss:  954MB ->  971MB (  +17MB)	codegen_crate
time:   0.000; rss:  969MB ->  966MB (   -3MB)	incr_comp_persist_dep_graph
time:   0.133; rss:  966MB ->  965MB (   -1MB)	encode_query_results
time:   0.140; rss:  966MB ->  965MB (   -1MB)	incr_comp_serialize_result_cache
time:   0.140; rss:  966MB ->  965MB (   -2MB)	incr_comp_persist_result_cache
time:   0.140; rss:  971MB ->  965MB (   -6MB)	serialize_dep_graph
time:   0.066; rss:  965MB ->  510MB ( -455MB)	free_global_ctxt
time:   0.000; rss:  510MB ->  510MB (   +0MB)	finish_ongoing_codegen
time:   0.037; rss:  510MB ->  523MB (  +13MB)	link_rlib
time:   0.039; rss:  510MB ->  523MB (  +13MB)	link_binary
time:   0.040; rss:  510MB ->  510MB (   +0MB)	link_crate
time:   0.041; rss:  510MB ->  510MB (   +0MB)	link
time:  51.505; rss:   32MB ->  187MB ( +155MB)	total
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 51.63s
	Command being timed: "cargo +nightly build -j1 --all-features"
	User time (seconds): 40.35
	System time (seconds): 11.27
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:51.65
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 14722704
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 3654346
	Voluntary context switches: 220
	Involuntary context switches: 1001
	Swaps: 0
	File system inputs: 0
	File system outputs: 127208
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0

```

### Copyright & Licenses

Copyright © 2023 Unicode, Inc. Unicode and the Unicode Logo are registered trademarks of Unicode, Inc. in the United States and other countries.

The project is released under [LICENSE](./LICENSE).

A CLA is required to contribute to this project - please refer to the [CONTRIBUTING.md](https://github.com/unicode-org/.github/blob/main/.github/CONTRIBUTING.md) file (or start a Pull Request) for more information.
