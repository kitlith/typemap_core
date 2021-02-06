use typemap_core::{typemap, TypeMapGet};

fn not_enough_bounds<Opts: TypeMapGet>(opts: &Opts) {
    println!("{}", opts.get::<u32>());
}

fn main() {
    not_enough_bounds(&typemap!(u32 = 42u32));
}
