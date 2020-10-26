initSidebarItems({"constant":[["PROGRESS_FREQUENCY_SECONDS",""]],"enum":[["Parallelism",""]],"fn":[["basename",""],["clamp",""],["contains_duplicates",""],["delete_file","Idempotent"],["deserialize_btreemap","Deserializes a BTreeMap from a list of tuples. Necessary when the keys are structs; see https://github.com/serde-rs/json/issues/402."],["deserialize_hashmap","Deserializes a HashMap from a list of tuples."],["deserialize_multimap","Deserializes a MultiMap."],["deserialize_usize","Deserializes a `usize` from a `u32`."],["elapsed_seconds",""],["file_exists",""],["find_next_file",""],["find_prev_file","Keeps file extensions"],["from_binary","Deserializes an object from the bincode format."],["from_json","Deserializes an object from a JSON string."],["list_all_objects","Just list all things from a directory, return sorted by name, with file extension removed."],["list_dir","Returns full paths"],["load_all_objects","Load all serialized things from a directory, return sorted by name, with file extension removed. Detects JSON or binary. Filters out broken files."],["maybe_read_binary",""],["maybe_read_json",""],["must_run_cmd","Runs a command, asserts success. STDOUT and STDERR aren't touched."],["parent_path",""],["path",""],["path_all_edits",""],["path_all_maps",""],["path_all_raw_maps",""],["path_all_saves",""],["path_all_scenarios",""],["path_camera_state",""],["path_edits",""],["path_map",""],["path_popdat",""],["path_prebaked_results",""],["path_raw_map",""],["path_save",""],["path_scenario",""],["plain_list_names",""],["prettyprint_time",""],["prettyprint_usize",""],["read_binary",""],["read_json",""],["read_object",""],["retain_btreemap",""],["retain_btreeset",""],["serialize_btreemap","Serializes a BTreeMap as a list of tuples. Necessary when the keys are structs; see https://github.com/serde-rs/json/issues/402."],["serialize_hashmap","Serializes a HashMap as a list of tuples, first sorting by the keys. This ensures the serialized form is deterministic."],["serialize_multimap","Serializes a MultiMap."],["serialize_usize","Serializes a `usize` as a `u32` to save space. Useful when you need `usize` for indexing, but the values don't exceed 2^32."],["serialized_size_bytes","The number of bytes for an object serialized to bincode."],["slurp_file",""],["to_json","Stringifies an object to nicely formatted JSON."],["to_json_terse","Stringifies an object to terse JSON."],["wraparound_get",""],["write_binary",""],["write_json",""]],"mod":[["abst_data",""],["abst_paths","Generate paths for different A/B Street files"],["cli",""],["collections",""],["io",""],["io_native","Normal file IO using the filesystem"],["process",""],["serde",""],["time",""],["utils",""]],"struct":[["CmdArgs",""],["Counter",""],["Entry","A single file"],["FileWithProgress",""],["FixedMap","A drop-in replacement for `BTreeMap`, where the keys have the property of being array indices. Some values may be missing. Much more efficient at operations on individual objects, because it just becomes a simple array lookup."],["Manifest","A list of all canonical data files for A/B Street that're uploaded somewhere. The file formats are tied to the latest version of the git repo. Players use the updater crate to sync these files with local copies."],["MultiMap",""],["Tags","Convenience functions around a string->string map"],["Timer","Hierarchial magic"],["VecMap","Use when your key is just PartialEq, not Ord or Hash."]],"trait":[["IndexableKey","Use with `FixedMap`. From a particular key, extract a `usize`. These values should be roughly contiguous; the space used by the `FixedMap` will be `O(n)` with respect to the largest value returned here."],["TimerSink",""]]});