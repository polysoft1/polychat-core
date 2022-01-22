mod complie;

use complie::compile;
use polychat_core::plugin::Plugin;


#[test]
fn load_garbage_path() {
    let plugin = Plugin::new("panic_plugin", "panic_path.garbage");
    assert_eq!(plugin.is_err(), true);
}

#[test]
fn load_plugin_path() {
    compile();
}