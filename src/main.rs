fn main() -> std::io::Result<()> {
    match   scale_degrees::init() {
        Ok(r) => Ok(r),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}
