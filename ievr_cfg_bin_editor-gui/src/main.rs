use std::path::PathBuf;

use ievr_cfg_bin_editor_core::parse_database;

fn main() {
    let input_path = "scout_phase_text_setting_1.03.25.cfg.bin";

    let file_path = PathBuf::from(input_path);

    let database = parse_database(&file_path).unwrap();

    // println!("Types:");
    // for r#type in rdbn.types {
    //     println!("{:?}", r#type.name)
    // }

    for table in database.tables() {
        println!("{}", table.name());
        println!("{:?}", table.schema());
    }

    // println!("\nList elements:");
    // for list_element in rdbn.lists {
    //     println!("{}", list_element.name)
    // }

}
