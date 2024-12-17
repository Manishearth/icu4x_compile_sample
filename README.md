# icu4x_compile_sample

See: https://github.com/unicode-org/icu4x/issues/5230

```
/path/to/baked_data_slow/demo$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
$ cargo clean; RUSTFLAGS="-Ztime-passes" /usr/bin/time -v cargo +nightly build -j1 --all-features
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
     Removed 16 files, 70.1MiB total
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling demo2 v0.1.0 (/home/manishearth/dev/Git/icu4x_compile_sample/demo)
time:   0.001; rss:   47MB ->   49MB (   +1MB)	parse_crate
time:   0.001; rss:   50MB ->   50MB (   +0MB)	incr_comp_prepare_session_directory
time:   0.000; rss:   50MB ->   51MB (   +1MB)	setup_global_ctxt
time:   0.000; rss:   52MB ->   52MB (   +0MB)	crate_injection
time:   1.194; rss:   52MB ->  595MB ( +543MB)	expand_crate
time:   1.194; rss:   52MB ->  595MB ( +543MB)	macro_expand_crate
time:   0.013; rss:  595MB ->  595MB (   +0MB)	AST_validation
time:   0.008; rss:  595MB ->  597MB (   +1MB)	finalize_macro_resolutions
time:   0.285; rss:  597MB ->  642MB (  +45MB)	late_resolve_crate
time:   0.012; rss:  642MB ->  642MB (   +0MB)	resolve_check_unused
time:   0.020; rss:  642MB ->  642MB (   +0MB)	resolve_postprocess
time:   0.326; rss:  595MB ->  642MB (  +46MB)	resolve_crate
time:   0.011; rss:  610MB ->  610MB (   +0MB)	write_dep_info
time:   0.011; rss:  610MB ->  611MB (   +0MB)	complete_gated_feature_checking
time:   0.058; rss:  765MB ->  729MB (  -35MB)	drop_ast
time:   1.213; rss:  610MB ->  681MB (  +71MB)	looking_for_derive_registrar
time:   1.421; rss:  610MB ->  682MB (  +72MB)	misc_checking_1
time:   0.086; rss:  682MB ->  690MB (   +8MB)	coherence_checking
time:   3.720; rss:  682MB ->  837MB ( +155MB)	type_check_crate
time:   0.000; rss:  837MB ->  837MB (   +0MB)	MIR_coroutine_by_move_body
time:  55.505; rss:  837MB -> 1058MB ( +221MB)	MIR_borrow_checking
time:   1.571; rss: 1058MB -> 1068MB (  +10MB)	MIR_effect_checking
time:   0.217; rss: 1068MB -> 1067MB (   -1MB)	module_lints
time:   0.217; rss: 1068MB -> 1067MB (   -1MB)	lint_checking
time:   0.311; rss: 1067MB -> 1068MB (   +0MB)	privacy_checking_modules
time:   0.607; rss: 1068MB -> 1068MB (   +0MB)	misc_checking_3
time:   0.000; rss: 1136MB -> 1137MB (   +1MB)	monomorphization_collector_graph_walk
time:   0.778; rss: 1068MB -> 1064MB (   -4MB)	generate_crate_metadata
time:   0.005; rss: 1064MB -> 1085MB (  +22MB)	codegen_to_LLVM_IR
time:   0.007; rss: 1076MB -> 1085MB (  +10MB)	LLVM_passes
time:   0.014; rss: 1064MB -> 1085MB (  +22MB)	codegen_crate
time:   0.257; rss: 1084MB -> 1080MB (   -4MB)	encode_query_results
time:   0.270; rss: 1084MB -> 1080MB (   -4MB)	incr_comp_serialize_result_cache
time:   0.270; rss: 1084MB -> 1080MB (   -4MB)	incr_comp_persist_result_cache
time:   0.271; rss: 1084MB -> 1080MB (   -4MB)	serialize_dep_graph
time:   0.124; rss: 1080MB ->  624MB ( -456MB)	free_global_ctxt
time:   0.000; rss:  624MB ->  624MB (   +0MB)	finish_ongoing_codegen
time:   0.127; rss:  624MB ->  653MB (  +29MB)	link_rlib
time:   0.135; rss:  624MB ->  653MB (  +29MB)	link_binary
time:   0.138; rss:  624MB ->  618MB (   -6MB)	link_crate
time:   0.139; rss:  624MB ->  618MB (   -6MB)	link
time:  65.803; rss:   32MB ->  187MB ( +155MB)	total
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 07s
	Command being timed: "cargo +nightly build -j1 --all-features"
	User time (seconds): 50.56
	System time (seconds): 16.37
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 1:07.24
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 14866408
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 229
	Minor (reclaiming a frame) page faults: 3674797
	Voluntary context switches: 496
	Involuntary context switches: 1951
	Swaps: 0
	File system inputs: 641768
	File system outputs: 211656
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
