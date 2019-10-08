pub const BLANK: &str = "";
pub const QUOTA: &str = "\"";
pub const EQUAL: &str = "=";
pub const COMMA: &str = ",";
pub const COLON: &str = ":";
pub const AMPERSAND: &str = "&";
pub const LEFT_BRACE: &str = "{";
pub const RIGHT_BRACE: &str = "}";
pub const PERCENT_SIGN: &str = "%";

/** 版本号. */
pub const PARAM_VERSION: &str = "Version";
/** 交易类型. */
pub const PARAM_TXN_TYPE: &str = "TranType";
/** 待查交易类型 */
pub const PARAM_ORI_TRAN_TYPE: &str = "OriTranType";
/** 业务类型. */
pub const PARAM_BIZ_TYPE: &str = "BusiType";
/** 前台交易请求地址. */
pub const PARAM_POST_URL: &str = "PostUrl";
/** 前台通知地址. */
pub const PARAM_FRONT_URL: &str = "MerPageUrl";
/** 后台通知地址. */
pub const PARAM_BACK_URL: &str = "MerBgUrl";
/** 商户代码. */
pub const PARAM_MER_ID: &str = "MerId";
/** 商户订单号. */
pub const PARAM_ORDER_ID: &str = "MerOrderNo";
/** 订单金额. */
pub const PARAM_TXN_AMT: &str = "OrderAmt";
/** 已退款金额 */
pub const PARAM_REFUND_SUM_AMT: &str = "RefundSumAmount";
/** 支付机构号 */
pub const PARAM_BANK_INST_NO: &str = "BankInstNo";
/** 交易日期. */
pub const PARAM_TXN_DATE: &str = "TranDate";
/** 交易时间. */
pub const PARAM_TXN_TIME: &str = "TranTime";
/** 订单支付状态. */
pub const PARAM_ORDER_STATUS: &str = "OrderStatus";
/** 收单流水号. */
pub const PARAM_ACQ_SEQ_ID: &str = "AcqSeqId";
/** 订单完成日期. */
pub const PARAM_COMPLETE_DATE: &str = "CompleteDate";
/** 订单完成时间. */
pub const PARAM_COMPLETE_TIME: &str = "CompleteTime";
/** 分帐类型 */
pub const PARAM_SPLIT_TYPE: &str = "SplitType";
/** 分帐方式 */
pub const PARAM_SPLIT_METHOD: &str = "SplitMethod";
/** 分帐信息 */
pub const PARAM_MER_SPLIT_MSG: &str = "MerSplitMsg";
/** 防钓鱼客户浏览器IP */
pub const PARAM_REMOTE_ADDR: &str = "RemoteAddr";
/** 账号. */
pub const PARAM_CARD_NO: &str = "CardNo";
/** 账户名称. */
pub const PARAM_ACC_NAME: &str = "AccName";
/** 证件类型. */
pub const PARAM_CERT_TYPE: &str = "CertType";
/** 证件号码. */
pub const PARAM_CERT_NO: &str = "CertNo";
/** 银行预留手机号 */
pub const PARAM_MOBILE_NO: &str = "MobileNo";
/** 短信验证码 */
pub const PARAM_MOBILE_AUTH_CODE: &str = "MobileAuthCode";
/** 交易卡要素. */
pub const PARAM_CUSTOMER_INFO: &str = "CardTranData";
/** 签约状态. */
pub const PARAM_SIGN_STATUS: &str = "SignState";
/** 证书ID. */
pub const PARAM_SIGN_CERT_ID: &str = "CertId";
/** 签名. */
pub const PARAM_SIGNATURE: &str = "Signature";
/** 应答码. */
pub const PARAM_RESP_CODE: &str = "respCode";
/** 应答码信息. */
pub const PARAM_RESP_MSG: &str = "respMsg";
/** 通知标记：前端通知，后端通知 */
pub const PARAM_NOTICE: &str = "Notice";

/** 版本号，固定值 */
pub const SIGN_VERSION: &str = "20140728";
pub const FAST_PAY_VERSION: &str = "20150922";
/** 业务类型：固定值0001 */
pub const BUSI_TYPE: &str = "0001";
/** 分帐类型：0001实时分帐 */
pub const SPLIT_TYPE: &str = "0001";
/** 交易类型： 0001 个人网瘾支付 0004 快捷支付 0608 快捷签约短信 */
pub const TRAN_TYPE: &str = "0004";
pub const TRAN_TYPE_QUERY: &str = "0502";