use lazy_tmux::plugins::ConfigFile;

#[test]
fn test_name() {
    let conf = ConfigFile::get_plugins().unwrap();
   
    for i in conf {
        i.install().unwrap();
    }
    
    assert!(dirs::data_dir().unwrap().join("tmux/plugins/abhinadh-s/lazy.tmux").exists())
}
