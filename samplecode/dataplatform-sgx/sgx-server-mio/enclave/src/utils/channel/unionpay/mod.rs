use std::collections::HashMap;
use std::slice::SliceConcatExt;
use std::string::String;
use std::vec::Vec;

pub mod sdkconstants;

//转换格式为：xx=aaa&yy=bbb
pub fn convert_to_url_param_str(data: &HashMap<&str, &str>) -> String {
    let mut result: String = String::from("");
    for (key, value) in data {
        let temp = format!("{}={}&", key, value);
        result.push_str(temp.as_str());
    }
    result.remove(result.len() - 1);
    result
}

//转换格式为(JSON)：{"key1":"value1","key2":"value2"}
pub fn convert_to_json_str(data: &HashMap<&str, &str>) -> String {
    let mut result: String;
    {
        let mut kv = Vec::new();
        for (key, value) in data {
            kv.push(format!("\"{}\":\"{}\"", key, value));
        }
        result = format!("{}{}{}", "{", kv.join(","), "}")
    }
    result
}

//input格式：xx=aaa&yy=bbb
pub fn convert_from_json_str(param: &String) -> HashMap<&str, &str> {
    let mut result = HashMap::new();
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

//从HashMap中获取字符串参数值，不存在默认返回空
pub fn get_str_from_map(data: &HashMap<&str, &str>, key: &str) -> Option<String> {
    let mut result = None;
    let value = data.get(key);
    match value {
        Some(v) => { result = Some(v.parse::<String>().unwrap()) }
        None => {}
    }
    result
}

//从HashMap中获取数字参数值，不存在默认返回0
pub fn get_num_from_map(data: &HashMap<&str, &str>, key: &str) -> Option<i32> {
    let mut result = None;
    let value = data.get(key);
    match value {
        Some(v) => { result = Some(v.parse::<i32>().unwrap()) }
        None => {}
    }
    result
}
