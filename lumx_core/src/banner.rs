pub const BANNER: &str = r"
.____     ____ ___  _____  ____  ___
|    |   |    |   \/     \ \   \/  /
|    |   |    |   /  \ /  \ \     /
|    |___|    |  /    Y    \/     \
|_______ \______/\____|__  /___/\  \
        \/               \/      \_/";

pub fn print_banner() {
    println!("{BANNER}");
    println!("v{}", version());
    println!()
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
