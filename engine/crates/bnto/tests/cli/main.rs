// The CLI's integration tests in one binary. Each file below was once its own
// test binary, linked and cached apart; one binary is one link and one
// incremental cache (ISS-120). A new test file goes in as a `mod` here.
//
// Two suites keep their own binaries: the TUI journeys (`tui_journeys`), which
// time a real terminal and would share their test threads with every test
// here, and the shell-run security suite (`shell_security`).

#[path = "../helpers/mod.rs"]
mod helpers;

mod assert_files;
mod check_edges;
mod check_folders;
mod check_run_time;
mod cli_commands;
mod copy_held_files;
mod dogfood_recipes;
mod dry_run_footer;
mod dry_run_path;
mod dry_run_quoting;
mod dry_run_temp_files;
mod dry_run_vars;
mod edges;
mod edit_fields;
mod file_delete;
mod file_inspect;
mod file_join;
mod file_params;
mod golden_helpers;
mod golden_tests;
mod image_pad;
mod keep_awake;
mod keep_awake_linux;
mod keep_awake_windows;
mod layout;
mod loop_failure;
mod loop_item_current;
mod loop_item_fill;
mod loop_step_item;
mod move_delete_held_files;
mod one_pass_fill;
mod output_dir_placeholders;
mod output_suffix;
mod progressive_output;
mod recipe_tests;
mod removed_params;
mod rename_pattern;
mod report_file_names;
mod run_args;
mod run_json;
mod run_temp_files;
mod run_timeout;
mod scoped_requires;
mod shell_each;
mod shell_loop_items;
mod shell_options;
mod telemetry_notice;
mod telemetry_recipe_label;
mod telemetry_setting;
mod unfilled_fields;
mod unfilled_params;
mod update_notice;
mod upgrade;
mod when_guard;
mod write_into_source_folder;
