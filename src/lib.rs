mod plugin;

#[cfg(test)]
mod tests {
    use crate::plugin::Plugin;

    #[test]
    fn test_garbage_path() {
        let plugin = Plugin::new("panic_plugin", "panic_path.garbage");
        assert_eq!(plugin.is_err(), true);
    }
}
