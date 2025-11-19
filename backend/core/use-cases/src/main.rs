fn main() {
    let a = use_cases::boundaries::__GenericErrResponse::Foo;
    let b = use_cases::boundaries::__GenericErrResponse::Bar("hello".into(), "world".into());
    let c = use_cases::boundaries::__GenericErrResponse::Baz {
        foo: "x".into(),
        bar: "y".into(),
    };

    println!("{}", serde_json::to_string_pretty(&a).unwrap());
    println!("{}", serde_json::to_string_pretty(&b).unwrap());
    println!("{}", serde_json::to_string_pretty(&c).unwrap());
}