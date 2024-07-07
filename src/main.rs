include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let s = unsafe { TestOk() };
    dbg!(s);

    let t = unsafe { TestErr() };
    dbg!(t);

}
