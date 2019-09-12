use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

pub mod sdkconstants;
pub mod sdkutil;
pub mod signutil;

//转换格式为：xx=aaa&yy=bbb
pub fn get_signed_param_str(data: &BTreeMap<&str, String>) -> String {
    let mut result: String = String::from("");
    for (key, value) in data {
        let temp = format!("{}={}&", key, value);
        result.push_str(temp.as_str());
    }
    result.remove(result.len() - 1);
    result
}

//转换格式为(JSON)：{"key1":"value1","key2":"value2"}
pub fn convert_to_json_str(data: &BTreeMap<&str, String>) -> String {
    let mut result: String = String::from("{");
    for (key, value) in data {
        let temp = format!("\"{}\":\"{}\",", key, value);
        result.push_str(temp.as_str());
    }
    result.remove(result.len() - 1);
    result.push_str("}");
    result
}

//input格式：xx=aaa&yy=bbb
pub fn convert_from_json_str(param: &String) -> BTreeMap<&str, &str> {
    let mut result = BTreeMap::new();
    let v: Vec<&str> = param.rsplit('&').collect();
    for d in v.into_iter() {
        match d.find('=') {
            None => { continue; }
            Some(v) => {
                result.insert(&d[..v], &d[(v + 1)..]);
            }
        }
    }
    result
}
