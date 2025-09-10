use faststr::FastStr;

const LONG_STRING: &str = "1145141919810114514191981011451419198101145141919810";
const SHORT_STRING: &str = "Hello, World";

fn main() {
    let s1 = FastStr::empty();
    let s2 = FastStr::from_string(LONG_STRING.to_string());
    let s3 = FastStr::from_arc_str(std::sync::Arc::from(LONG_STRING));
    let s4 = FastStr::from_arc_string(std::sync::Arc::new(LONG_STRING.to_string()).clone());
    let s5 = FastStr::from_static_str(LONG_STRING);
    let s6 = FastStr::from_string(SHORT_STRING.to_string());

    println!("{s1:?}");
    println!("{s2:?}");
    println!("{s3:?}");
    println!("{s4:?}");
    println!("{s5:?}");
    println!("{s6:?}");
}
