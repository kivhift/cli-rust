fn main() {
    if let Err(e) = r_cat::get_args().and_then(r_cat::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
