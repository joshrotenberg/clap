use crate::utils;

use clap::{arg, error::ErrorKind, App, Arg, ArgGroup};

static REQ_GROUP_USAGE: &str = "error: The following required arguments were not provided:
    <base|--delete>

USAGE:
    clap-test <base|--delete>

For more information try --help
";

static REQ_GROUP_CONFLICT_USAGE: &str =
    "error: The argument '--delete' cannot be used with '<base>'

USAGE:
    clap-test <base|--delete>

For more information try --help
";

static REQ_GROUP_CONFLICT_ONLY_OPTIONS: &str =
    "error: The argument '--delete' cannot be used with '--all'

USAGE:
    clap-test <--all|--delete>

For more information try --help
";

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!( -c --color "some other flag"))
        .group(ArgGroup::new("req").args(&["flag", "color"]).required(true))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group 'req' contains non-existent argument"]
fn non_existing_arg() {
    let _ = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(&["flg", "color"]).required(true))
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name must be unique\n\n\t'req' is already in use"]
fn unique_group_name() {
    let _ = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(&["flag"]).required(true))
        .group(ArgGroup::new("req").args(&["color"]).required(true))
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name '' must not conflict with argument name"]
fn groups_new_of_arg_name() {
    let _ = App::new("group")
        .arg(Arg::new("a").long("a").group("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name 'a' must not conflict with argument name"]
fn arg_group_new_of_arg_name() {
    let _ = App::new("group")
        .arg(Arg::new("a").long("a").group("a"))
        .group(ArgGroup::new("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[test]
fn group_single_value() {
    let res = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color [color] "some option"))
        .group(ArgGroup::new("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let res = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color [color] "some option"))
        .group(ArgGroup::new("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let res = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color [color] "some option"))
        .group(ArgGroup::new("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec![""]);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let m = res.unwrap();
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some option"))
        .group(ArgGroup::new("grp").required(true).args(&["flag", "color"]))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let res = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color <color> "some option").multiple_values(true))
        .group(ArgGroup::new("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(
        &*m.values_of("grp").unwrap().collect::<Vec<_>>(),
        &["blue", "red", "green"]
    );
}

#[test]
fn empty_group() {
    let r = App::new("empty_group")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("vers").required(true))
        .try_get_matches_from(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn req_group_usage_string() {
    let app = App::new("req_group")
        .arg(arg!([base] "Base commit"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(utils::compare_output(
        app,
        "clap-test",
        REQ_GROUP_USAGE,
        true
    ));
}

#[test]
fn req_group_with_conflict_usage_string() {
    let app = App::new("req_group")
        .arg(arg!([base] "Base commit").conflicts_with("delete"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(utils::compare_output(
        app,
        "clap-test --delete base",
        REQ_GROUP_CONFLICT_USAGE,
        true
    ));
}

#[test]
fn req_group_with_conflict_usage_string_only_options() {
    let app = App::new("req_group")
        .arg(arg!(-a --all "All").conflicts_with("delete"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("all_or_delete")
                .args(&["all", "delete"])
                .required(true),
        );
    assert!(utils::compare_output(
        app,
        "clap-test --delete --all",
        REQ_GROUP_CONFLICT_ONLY_OPTIONS,
        true
    ));
}

#[test]
fn required_group_multiple_args() {
    let result = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(
            ArgGroup::new("req")
                .args(&["flag", "color"])
                .required(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn group_multiple_args_error() {
    let result = App::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(&["flag", "color"]))
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn group_usage_use_val_name() {
    static GROUP_USAGE_USE_VAL_NAME: &str = "prog 

USAGE:
    prog <A>

ARGS:
    <A>    

OPTIONS:
    -h, --help    Print help information
";
    let app = App::new("prog")
        .arg(Arg::new("a").value_name("A"))
        .group(ArgGroup::new("group").arg("a").required(true));
    assert!(utils::compare_output(
        app,
        "prog --help",
        GROUP_USAGE_USE_VAL_NAME,
        false,
    ));
}

#[test]
fn group_acts_like_arg() {
    let result = App::new("prog")
        .arg(Arg::new("debug").long("debug").group("mode"))
        .arg(Arg::new("verbose").long("verbose").group("mode"))
        .try_get_matches_from(vec!["prog", "--debug"]);

    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.is_present("mode"));
}

/* This is used to be fixed in a hack, we need to find a better way to fix it.
#[test]
fn issue_1794() {
    let app = clap::App::new("hello")
        .bin_name("deno")
        .arg(Arg::new("option1").long("option1").takes_value(false))
        .arg(Arg::new("pos1").takes_value(true))
        .arg(Arg::new("pos2").takes_value(true))
        .group(
            ArgGroup::new("arg1")
                .args(&["pos1", "option1"])
                .required(true),
        );

    let m = app.clone().try_get_matches_from(&["app", "pos1", "pos2"]).unwrap();
    assert_eq!(m.value_of("pos1"), Some("pos1"));
    assert_eq!(m.value_of("pos2"), Some("pos2"));
    assert!(!m.is_present("option1"));

    let m = app
        .clone()
        .try_get_matches_from(&["app", "--option1", "positional"]).unwrap();
    assert_eq!(m.value_of("pos1"), None);
    assert_eq!(m.value_of("pos2"), Some("positional"));
    assert!(m.is_present("option1"));
}
*/
