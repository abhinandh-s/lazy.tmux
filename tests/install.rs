use std::path;

use lazy_tmux::plugins::{ConfigFile, Plugins};

#[test]
fn test_name() {
    let conf = ConfigFile::get_plugins();
    match &conf {
        Some(config) => panic!("{:?}", config),
        None => panic!("found none"),
    }
    
   // let first = Plugins::new("abhinandh-s".into(), "lazy.tmux".into(), None, None// );
    // assert_eq!(conf.first().unwrap().to_owned(), first)
}
