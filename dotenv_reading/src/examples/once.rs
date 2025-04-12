use std::sync::Once;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        println!("init");
    });
}

pub fn once_example() {
    init();
    init();
    init();
}
