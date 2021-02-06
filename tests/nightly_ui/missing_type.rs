use typemap_core::{typemap, Contains};

fn missing_value<Opts: Contains<u32>>(opts: &Opts) {
    println!("{}", opts.get::<u32>());
}

fn main() {
    let map = typemap!(&str = "Hello, world!");
    missing_value(&map);
}
