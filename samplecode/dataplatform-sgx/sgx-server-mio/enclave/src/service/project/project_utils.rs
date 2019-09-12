use service::response::LastUpdatedTime;
use std::string::String;
use std::vec::Vec;
use utils::db::DbContext;

pub enum ProjectTable {
    Asset,
    Bill,
    Ledger,
    Receipt,
}

pub fn get_ids(projects: &Vec<u32>) -> String {
    let mut ids = String::new();

    let mut index: i32 = 0;
    let last_pos: i32 = (projects.len() - 1) as i32;
    for x in projects.iter() {
        ids.push_str(format!("{}", x).as_str());
        if index != last_pos {
            ids.push_str(",");
        }
        index += 1;
    }
    ids
}

pub fn get_updated_time(db_context: &DbContext, tbl: &ProjectTable) -> String {
    let table_name = match tbl {
        ProjectTable::Asset => "project_asset",
        ProjectTable::Bill => "project_bill",
        ProjectTable::Ledger => "project_ledger",
        ProjectTable::Receipt => "project_receipt",
    };
    let time: Option<String> = db_context.get_last_update_time_local(table_name);
    LastUpdatedTime::local_updated_time_json_str(time)
}