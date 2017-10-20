extern crate difference;
extern crate trace_pretty;
use difference::Changeset;

#[test]
fn test() {
    let input = r#"struct => PRIV struct Params [ @ app test ::std::string::ParseError ] [
subcommands => {  } params => {
( PRIV quiet QUIET bool { q } {  } { flag } ) (
PRIV verbose VERBOSE bool { v } {  } { flag } ) (
PRIV cfg CFG String { c } {  } { default String::from("") } ) (
PRIV path PATH String { p } {  } { required } ) (
PRIV foo FOO Option<String> {  } { foo } { optional } ) (
PRIV n N Option<u32> { n } {  } { option map (|v: Option<&str>| v.map(|_| 3))
} ) ( PRIV x X u32 { x } {  } { map (|v| 4) } ) } ]"#;

    let desired = r#"struct => PRIV struct Params [ @ app test ::std::string::ParseError ]
[
    subcommands => {  }
    params => {
        ( PRIV quiet QUIET bool { q } {  } { flag } )
        ( PRIV verbose VERBOSE bool { v } {  } { flag } )
        ( PRIV cfg CFG String { c } {  } { default String::from("") } )
        ( PRIV path PATH String { p } {  } { required } )
        ( PRIV foo FOO Option<String> {  } { foo } { optional } )
        ( PRIV n N Option<u32> { n } {  } {
            option map (|v: Option<&str>| v.map(|_| 3))
        } )
        ( PRIV x X u32 { x } {  } { map (|v| 4) } )
    }
]"#;

    let output = trace_pretty::print(input);

    if output != desired {
        println!("{}", Changeset::new(&output, desired, "\n"));
        panic!("output != desired");
    }
}

