package com.energy.sgx.utils;

/**
 * @author Bryan
 * @date 2019-09-27
 */
public class SDKConstants {

    public static final String BLANK = "";
    public final static String QUOTA = "\"";
    public static final String EQUAL = "=";
    public static final String COMMA = ",";
    public static final String COLON = ":";
    public static final String AMPERSAND = "&";
    public static final String LEFT_BRACE = "{";
    public static final String RIGHT_BRACE = "}";
    public static final String PERCENT_SIGN = "%";

    /** 版本号. */
    public static final String param_version = "Version";
    /** 交易类型. */
    public static final String param_txnType = "TranType";
    /** 待查交易类型 */
    public static final String param_oriTranType = "OriTranType";
    /** 业务类型. */
    public static final String param_bizType = "BusiType";
    /** 前台通知地址. */
    public static final String param_frontUrl = "MerPageUrl";
    /** 后台通知地址. */
    public static final String param_backUrl = "MerBgUrl";
    /** 前台交易请求地址. */
    public static final String param_postUrl = "PostUrl";
    /** 商户代码. */
    public static final String param_merId = "MerId";
    /** 商户订单号. */
    public static final String param_orderId = "MerOrderNo";
    /** 订单金额. */
    public static final String param_txnAmt = "OrderAmt";
    /** 已退款金额 */
    public static final String param_refundSumAmt = "RefundSumAmount";
    /** 支付机构号 */
    public static final String param_bankInstNo = "BankInstNo";
    /** 交易日期. */
    public static final String param_txnDate = "TranDate";
    /** 交易时间. */
    public static final String param_txnTime = "TranTime";
    /** 订单支付状态. */
    public static final String param_orderStatus = "OrderStatus";
    /** 收单流水号. */
    public static final String param_acqSeqId = "AcqSeqId";
    /** 订单完成日期. */
    public static final String param_completeDate = "CompleteDate";
    /** 订单完成时间. */
    public static final String param_completeTime = "CompleteTime";
    /** 分帐类型 */
    public static final String param_splitType = "SplitType";
    /** 分帐方式 */
    public static final String param_splitMethod = "SplitMethod";
    /** 分帐信息 */
    public static final String param_merSplitMsg = "MerSplitMsg";
    /** 防钓鱼客户浏览器IP */
    public static final String param_remoteAddr = "RemoteAddr";

    /** 账号. */
    public static final String param_cardNo = "CardNo";
    /** 账户名称. */
    public static final String param_accName = "AccName";
    /** 证件类型. */
    public static final String param_certType = "CertType";
    /** 证件号码. */
    public static final String param_certNo = "CertNo";
    /** 银行预留手机号 */
    public static final String param_mobileNo = "MobileNo";
    /** 短信验证码 */
    public static final String param_mobileAuthCode = "MobileAuthCode";
    /** 交易卡要素. */
    public static final String param_customerInfo = "CardTranData";
    /** 签约状态. */
    public static final String param_signStatus = "SignState";
    /** 证书ID. */
    public static final String param_signCertId = "CertId";
    /** 签名. */
    public static final String param_signature = "Signature";

    /** 应答码. */
    public static final String param_respCode = "respCode";
    /** 应答码信息. */
    public static final String param_respMsg = "respMsg";

    /** 交易通知：true前端，false后端 */
    public static final String param_Notice = "Notice";
}
