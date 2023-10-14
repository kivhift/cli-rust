fn main() {
    if let Err(e) = r_wc::get_args().and_then(r_wc::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
