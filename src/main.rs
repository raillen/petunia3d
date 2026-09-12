fn main() {
    // CLI mínimo (o resto é env): --smoke-test | --screenshot ARQ [--frames N] [--size WxH]
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--smoke-test") {
        match petunia_app::smoke_test() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("SMOKE FAIL: {e:?}");
                std::process::exit(1);
            }
        }
        return;
    }
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot" => {
                if let Some(p) = args.get(i + 1) {
                    std::env::set_var("PETUNIA_SCREENSHOT", p);
                    i += 1;
                }
            }
            "--frames" => {
                if let Some(n) = args.get(i + 1) {
                    std::env::set_var("PETUNIA_SHOT_FRAMES", n);
                    i += 1;
                }
            }
            "--size" => {
                if let Some(s) = args.get(i + 1) {
                    std::env::set_var("PETUNIA_SHOT_SIZE", s);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    petunia_app::run();
}
