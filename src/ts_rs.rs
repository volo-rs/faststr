use crate::FastStr;

impl ts_rs::TS for FastStr {
    type WithoutGenerics = Self;
    fn name() -> String {
        "string".to_owned()
    }
    fn inline() -> String {
        <Self as ts_rs::TS>::name()
    }
    /// This function is expected to panic because primitive types cannot be flattened.
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", <Self as ts_rs::TS>::name())
    }
    /// This function is expected to panic because primitive types cannot be declared.
    fn decl() -> String {
        panic!("{} cannot be declared", <Self as ts_rs::TS>::name())
    }
    /// Same as `decl` if the type is not generic.
    fn decl_concrete() -> String {
        panic!("{} cannot be declared", <Self as ts_rs::TS>::name())
    }
}

#[test]
fn test_ts_rs() {
    #[derive(ts_rs::TS)]
    struct Nested {
        #[allow(unused)]
        #[ts(rename = "nested_id")]
        id: FastStr,
    }
    #[derive(ts_rs::TS)]
    struct Test {
        #[allow(unused)]
        #[ts(optional)]
        id: Option<FastStr>,
        #[allow(unused)]
        #[ts(flatten)]
        nested: Nested,
    }

    assert_eq!(
        <Test as ts_rs::TS>::decl(),
        "type Test = { id?: string, nested_id: string, };"
    );
}
