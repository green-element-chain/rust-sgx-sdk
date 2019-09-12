use rustls::{Certificate, PrivateKey};

use config::{ApplicationConfig, UnionpayTranParam};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::string::{String, ToString};
use utils::channel::unionpay::{self, signutil::*};
use utils::file;

#[derive(Clone)]
pub struct SDKUtil {
    pub_key: Certificate,
    pri_key: PrivateKey,
}

impl SDKUtil {
    pub fn new(tran_param: &UnionpayTranParam) -> SDKUtil {
        let verify_file_name = tran_param.sign_cert_verify();
        let verify_certs = file::load_certs(verify_file_name.as_str());
        let cert = verify_certs.get(0).unwrap().clone();
        //info!("cert: {:?}", cert);

        let cert_file_name = tran_param.sign_cert_tran();
        let pkey = file::load_private_key(cert_file_name.as_str());
        //info!("private key: {:?}", pkey);

        SDKUtil {
            pub_key: cert,
            pri_key: pkey,
        }
    }

    pub fn get_customer_info(&self, data: &BTreeMap<&str, String>) -> String {
        let json_str = unionpay::convert_to_json_str(data);
        let encode_str = base64::encode(&json_str);

        let public_key = "";
        let encode_str1 = "";

        base64::encode(encode_str1)
    }

    pub fn sign(&self, req_data: &mut BTreeMap<&str, String>) -> bool {
        info!("pri key: {:?}", self.pri_key.0.as_slice());
        let data: &str = "BusiType=0001&MerBgUrl=http://127.0.0.1:9000/dataplatformServer/payment/backRcvResponse&MerId=000091905069034&MerOrderNo=20190523183849760L&MerPageUrl=http://127.0.0.1:9000/dataplatformServer/payment/frontRcvResponse&MerSplitMsg=999991905069034^60;999991905069035^40&OrderAmt=457123&RemoteAddr=101.87.163.240&SplitMethod=1&SplitType=0001&TranDate=20190907&TranTime=161151&TranType=0001&Version=20150922";
        let sign_data = signature(data.as_bytes(), self.pri_key.0.as_slice());
        let result = String::from_utf8_lossy(&sign_data).to_string();
        info!("result: {}", result.as_str());
        //let content = sign_data.to_base64();

        true
    }

    pub fn verify(&self, data: &str, signed_str: &str) -> bool {
        let result = verify(data.as_bytes(), self.pub_key.as_ref(), signed_str.as_bytes());
        result
    }
}
