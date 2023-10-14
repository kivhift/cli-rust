fn main() {
    if let Err(e) = r_head::get_args().and_then(r_head::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
