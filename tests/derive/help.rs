use clap::{AppSettings, Args, IntoApp, Parser, Subcommand};

#[test]
fn arg_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(long)]
        #[clap(help_heading = Some("HEADING A"))]
        should_be_in_section_a: u32,

        #[clap(long)]
        no_section: u32,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = app
        .get_arguments()
        .find(|a| a.get_name() == "no-section")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), None);
}

#[test]
fn app_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    #[clap(next_help_heading = "DEFAULT")]
    struct CliOptions {
        #[clap(long)]
        #[clap(help_heading = Some("HEADING A"))]
        should_be_in_section_a: u32,

        #[clap(long)]
        should_be_in_default_section: u32,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_default_section = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-default-section")
        .unwrap();
    assert_eq!(
        should_be_in_default_section.get_help_heading(),
        Some("DEFAULT")
    );
}

#[test]
fn app_help_heading_flattened() {
    // Used to help track the cause in tests
    #![allow(clippy::enum_variant_names)]

    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(flatten)]
        options_a: OptionsA,

        #[clap(flatten)]
        options_b: OptionsB,

        #[clap(subcommand)]
        sub_a: SubA,

        #[clap(long)]
        should_be_in_default_section: u32,
    }

    #[derive(Debug, Clone, Args)]
    #[clap(next_help_heading = "HEADING A")]
    struct OptionsA {
        #[clap(long)]
        should_be_in_section_a: u32,
    }

    #[derive(Debug, Clone, Args)]
    #[clap(next_help_heading = "HEADING B")]
    struct OptionsB {
        #[clap(long)]
        should_be_in_section_b: u32,
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubA {
        #[clap(flatten)]
        SubB(SubB),
        #[clap(subcommand)]
        SubC(SubC),
        SubAOne,
        #[clap(next_help_heading = "SUB A")]
        SubATwo {
            should_be_in_sub_a: u32,
        },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubB {
        #[clap(next_help_heading = "SUB B")]
        SubBOne { should_be_in_sub_b: u32 },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubC {
        #[clap(next_help_heading = "SUB C")]
        SubCOne { should_be_in_sub_c: u32 },
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-b")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), Some("HEADING B"));

    let should_be_in_default_section = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-default-section")
        .unwrap();
    assert_eq!(should_be_in_default_section.get_help_heading(), None);

    let sub_a_two = app.find_subcommand("sub-a-two").unwrap();

    let should_be_in_sub_a = sub_a_two
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-a")
        .unwrap();
    assert_eq!(should_be_in_sub_a.get_help_heading(), Some("SUB A"));

    let sub_b_one = app.find_subcommand("sub-b-one").unwrap();

    let should_be_in_sub_b = sub_b_one
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-b")
        .unwrap();
    assert_eq!(should_be_in_sub_b.get_help_heading(), Some("SUB B"));

    let sub_c = app.find_subcommand("sub-c").unwrap();
    let sub_c_one = sub_c.find_subcommand("sub-c-one").unwrap();

    let should_be_in_sub_c = sub_c_one
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-c")
        .unwrap();
    assert_eq!(should_be_in_sub_c.get_help_heading(), Some("SUB C"));
}

#[test]
fn flatten_field_with_help_heading() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(flatten)]
        #[clap(next_help_heading = "HEADING A")]
        options_a: OptionsA,
    }

    #[derive(Debug, Clone, Args)]
    struct OptionsA {
        #[clap(long)]
        should_be_in_section_a: u32,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));
}

// The challenge with this test is creating an error situation not caught by `clap`'s error checking
// but by the code that `clap_derive` generates.
//
// Ultimately, the easiest way to confirm is to put a debug statement in the desired error path.
#[test]
fn derive_generated_error_has_full_context() {
    #[derive(Debug, Parser)]
    #[clap(setting(AppSettings::SubcommandsNegateReqs))]
    struct Opts {
        #[clap(long)]
        req_str: String,

        #[clap(subcommand)]
        cmd: Option<SubCommands>,
    }

    #[derive(Debug, Parser)]
    enum SubCommands {
        Sub {
            #[clap(short, long, parse(from_occurrences))]
            verbose: u8,
        },
    }

    let result = Opts::try_parse_from(&["test", "sub"]);
    assert!(
        result.is_err(),
        "`SubcommandsNegateReqs` with non-optional `req_str` should fail: {:?}",
        result.unwrap()
    );

    let expected = r#"error: The following required argument was not provided: req-str

USAGE:
    clap --req-str <REQ_STR>
    clap <SUBCOMMAND>

For more information try --help
"#;
    assert_eq!(result.unwrap_err().to_string(), expected);
}

#[test]
fn derive_order_next_order() {
    static HELP: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag-b                 first flag
        --option-b <OPTION_B>    first option
    -h, --help                   Print help information
    -V, --version                Print version information
        --flag-a                 second flag
        --option-a <OPTION_A>    second option
";

    #[derive(Parser, Debug)]
    #[clap(name = "test", version = "1.2")]
    #[clap(setting = AppSettings::DeriveDisplayOrder)]
    struct Args {
        #[clap(flatten)]
        a: A,
        #[clap(flatten)]
        b: B,
    }

    #[derive(Args, Debug)]
    #[clap(next_display_order = 10000)]
    struct A {
        /// second flag
        #[clap(long)]
        flag_a: bool,
        /// second option
        #[clap(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    #[clap(next_display_order = 10)]
    struct B {
        /// first flag
        #[clap(long)]
        flag_b: bool,
        /// first option
        #[clap(long)]
        option_b: Option<String>,
    }

    use clap::IntoApp;
    let mut app = Args::into_app();

    let mut buffer: Vec<u8> = Default::default();
    app.write_help(&mut buffer).unwrap();
    let help = String::from_utf8(buffer).unwrap();
    assert_eq!(help, HELP);
}

#[test]
fn derive_order_next_order_flatten() {
    static HELP: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag-b                 first flag
        --option-b <OPTION_B>    first option
    -h, --help                   Print help information
    -V, --version                Print version information
        --flag-a                 second flag
        --option-a <OPTION_A>    second option
";

    #[derive(Parser, Debug)]
    #[clap(setting = AppSettings::DeriveDisplayOrder)]
    #[clap(name = "test", version = "1.2")]
    struct Args {
        #[clap(flatten)]
        #[clap(next_display_order = 10000)]
        a: A,
        #[clap(flatten)]
        #[clap(next_display_order = 10)]
        b: B,
    }

    #[derive(Args, Debug)]
    struct A {
        /// second flag
        #[clap(long)]
        flag_a: bool,
        /// second option
        #[clap(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    struct B {
        /// first flag
        #[clap(long)]
        flag_b: bool,
        /// first option
        #[clap(long)]
        option_b: Option<String>,
    }

    use clap::IntoApp;
    let mut app = Args::into_app();

    let mut buffer: Vec<u8> = Default::default();
    app.write_help(&mut buffer).unwrap();
    let help = String::from_utf8(buffer).unwrap();
    assert_eq!(help, HELP);
}

#[test]
fn derive_order_no_next_order() {
    static HELP: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag-a                 first flag
        --flag-b                 second flag
    -h, --help                   Print help information
        --option-a <OPTION_A>    first option
        --option-b <OPTION_B>    second option
    -V, --version                Print version information
";

    #[derive(Parser, Debug)]
    #[clap(name = "test", version = "1.2")]
    #[clap(setting = AppSettings::DeriveDisplayOrder)]
    #[clap(next_display_order = None)]
    struct Args {
        #[clap(flatten)]
        a: A,
        #[clap(flatten)]
        b: B,
    }

    #[derive(Args, Debug)]
    struct A {
        /// first flag
        #[clap(long)]
        flag_a: bool,
        /// first option
        #[clap(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    struct B {
        /// second flag
        #[clap(long)]
        flag_b: bool,
        /// second option
        #[clap(long)]
        option_b: Option<String>,
    }

    use clap::IntoApp;
    let mut app = Args::into_app();

    let mut buffer: Vec<u8> = Default::default();
    app.write_help(&mut buffer).unwrap();
    let help = String::from_utf8(buffer).unwrap();
    assert_eq!(help, HELP);
}
