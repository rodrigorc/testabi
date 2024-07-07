include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let s = unsafe { TestYYY() };
    dbg!(s);

    let t = unsafe { TestXXX() };
    dbg!(t);

}
