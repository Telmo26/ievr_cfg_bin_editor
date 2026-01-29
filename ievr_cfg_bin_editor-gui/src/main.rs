use std::path::PathBuf;

use ievr_cfg_bin_editor_core::{parse_database};

fn main() {
    let input_path = "cpk_list-decrypted.cfg.bin";
    // let input_path = "chara_param_1.03.66.00.cfg.bin";
    // let input_path = "scout_phase_text_setting_1.03.25.cfg.bin";

    let file_path = PathBuf::from(input_path);

    let database = parse_database(&file_path).unwrap();

    for table in database.tables() {
        println!("{}", table.name());
        println!("{:?}", table.schema());
    }

    let table = database.table("CPK_ITEM").unwrap();

    println!("{:?}", table.rows()[0]);

    // println!("\nList elements:");
    // for list_element in rdbn.lists {
    //     println!("{}", list_element.name)
    // }

}
