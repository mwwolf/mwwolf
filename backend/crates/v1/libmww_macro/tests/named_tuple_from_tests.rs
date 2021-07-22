#[macro_use]
extern crate libmww_macro;

#[derive(NamedTupleFrom)]
struct Foo(String);

#[test]
fn it_works() {
    let target = Foo("this is test".into());
    let actual: String = target.into();
    assert_eq!(actual, "this is test".to_string());
}
