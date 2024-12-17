# icu4x_compile_sample

See: https://github.com/unicode-org/icu4x/issues/5230

```
/path/to/baked_data_slow/demo$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
     Removed 22 files, 74.6MiB total
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling demo2 v0.1.0 (/home/manishearth/dev/Git/icu4x_compile_sample/demo)
time:   0.001; rss:   48MB ->   49MB (   +1MB)	parse_crate
time:   0.000; rss:   50MB ->   50MB (   +0MB)	incr_comp_prepare_session_directory
time:   0.000; rss:   50MB ->   51MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   52MB ->   52MB (   +0MB)	crate_injection
time:   0.634; rss:   52MB ->  297MB ( +244MB)	expand_crate
time:   0.634; rss:   52MB ->  297MB ( +245MB)	macro_expand_crate
time:   0.009; rss:  297MB ->  297MB (   +0MB)	AST_validation
time:   0.001; rss:  297MB ->  298MB (   +1MB)	finalize_imports
time:   0.000; rss:  298MB ->  298MB (   +0MB)	compute_effective_visibilities
time:   0.184; rss:  298MB ->  327MB (  +29MB)	late_resolve_crate
time:   0.011; rss:  327MB ->  327MB (   +0MB)	resolve_check_unused
time:   0.019; rss:  327MB ->  327MB (   +0MB)	resolve_postprocess
time:   0.215; rss:  297MB ->  327MB (  +30MB)	resolve_crate
time:   0.010; rss:  319MB ->  319MB (   +0MB)	write_dep_info
time:   0.012; rss:  319MB ->  319MB (   +0MB)	complete_gated_feature_checking
time:   0.046; rss:  415MB ->  355MB (  -60MB)	drop_ast
time:   0.718; rss:  319MB ->  336MB (  +17MB)	looking_for_derive_registrar
time:   0.000; rss:  337MB ->  337MB (   +0MB)	unused_lib_feature_checking
time:   0.849; rss:  319MB ->  337MB (  +18MB)	misc_checking_1
time:   0.051; rss:  337MB ->  345MB (   +8MB)	coherence_checking
time:   3.605; rss:  337MB ->  580MB ( +242MB)	type_check_crate
time:  46.478; rss:  580MB ->  965MB ( +385MB)	MIR_borrow_checking
time:   0.958; rss:  965MB ->  956MB (   -9MB)	MIR_effect_checking
time:   0.111; rss:  956MB ->  956MB (   +0MB)	module_lints
time:   0.111; rss:  956MB ->  956MB (   +0MB)	lint_checking
time:   0.199; rss:  956MB ->  956MB (   +0MB)	privacy_checking_modules
time:   0.370; rss:  956MB ->  956MB (   +1MB)	misc_checking_3
time:   0.000; rss:  994MB ->  995MB (   +1MB)	monomorphization_collector_graph_walk
time:   0.322; rss:  956MB ->  957MB (   +1MB)	generate_crate_metadata
time:   0.005; rss:  969MB ->  977MB (   +7MB)	LLVM_passes
time:   0.001; rss:  957MB ->  976MB (  +19MB)	codegen_to_LLVM_IR
time:   0.009; rss:  957MB ->  975MB (  +18MB)	codegen_crate
time:   0.000; rss:  975MB ->  975MB (   +0MB)	assert_dep_graph
time:   0.000; rss:  974MB ->  970MB (   -4MB)	incr_comp_persist_dep_graph
time:   0.152; rss:  970MB ->  969MB (   -1MB)	encode_query_results
time:   0.159; rss:  970MB ->  969MB (   -1MB)	incr_comp_serialize_result_cache
time:   0.160; rss:  970MB ->  969MB (   -1MB)	incr_comp_persist_result_cache
time:   0.160; rss:  975MB ->  969MB (   -6MB)	serialize_dep_graph
time:   0.078; rss:  969MB ->  555MB ( -414MB)	free_global_ctxt
time:   0.000; rss:  555MB ->  555MB (   +0MB)	finish_ongoing_codegen
time:   0.062; rss:  555MB ->  579MB (  +24MB)	link_rlib
time:   0.065; rss:  555MB ->  579MB (  +24MB)	link_binary
time:   0.067; rss:  555MB ->  555MB (   +0MB)	link_crate
time:   0.068; rss:  555MB ->  555MB (   +0MB)	link
time:  53.831; rss:   32MB ->  193MB ( +161MB)	total
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 53.97s
	Command being timed: "cargo +nightly build -j1 --all-features"
	User time (seconds): 41.87
	System time (seconds): 12.07
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:53.98
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 14756780
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 3663023
	Voluntary context switches: 233
	Involuntary context switches: 1102
	Swaps: 0
	File system inputs: 0
	File system outputs: 168088
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
