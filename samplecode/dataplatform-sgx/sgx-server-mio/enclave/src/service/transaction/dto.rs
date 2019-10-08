use service::project::dto::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResponseCode {
    code_map: HashMap<i16, i16>,
}

impl ResponseCode {
    pub fn new() -> ResponseCode {
        let mut resp_code = ResponseCode {
            code_map: HashMap::new(),
        };

        resp_code.add(0000, "成功", 0, "成功状态");
        resp_code.add(0001, "初始状态", 1, "中间状态");
        resp_code.add(0003, "消费交易失败", -1, "失败状态");
        resp_code.add(0006, "签约失败", -1, "失败状态");
        resp_code.add(0007, "重复签约", -1, "失败状态");
        resp_code.add(0009, "退款交易失败", -1, "失败状态");
        resp_code.add(0012, "交易撤销成功", -1, "失败状态");
        resp_code.add(0014, "数据接收成功", 1, "中间状态");
        resp_code.add(0024, "退款撤销成功", -1, "失败状态");
        resp_code.add(0025, "重复交易", -1, "失败状态");
        resp_code.add(0026, "预授权完成处理成功", 0, "成功状态");
        resp_code.add(0029, "预授权撤销成功", -1, "失败状态");
        resp_code.add(0031, "退款撤销审核不通过", -1, "失败状态");
        resp_code.add(0037, "预授权完成撤销成功", -1, "失败状态");
        resp_code.add(1002, "商户审核不通过", -1, "失败状态");
        resp_code.add(1003, "商户已审核", 1, "中间状态");
        resp_code.add(1005, "交易撤销中", 1, "中间状态");
        resp_code.add(1007, "退款撤销中", 1, "中间状态");
        resp_code.add(1008, "预授权完成处理中", 1, "中间状态");
        resp_code.add(1009, "预授权撤销中", 1, "中间状态");
        resp_code.add(1010, "重汇已申请", 1, "中间状态");
        resp_code.add(1011, "重汇审核通过", 1, "中间状态");
        resp_code.add(1012, "重汇审核不通过", -1, "失败状态");
        resp_code.add(1013, "退款成功(重汇)", 0, "成功状态");
        resp_code.add(1014, "预授权完成撤销中", 1, "中间状态");
        resp_code.add(1015, "交易发送成功", 1, "中间状态");
        resp_code.add(1016, "交易发送失败", 1, "中间状态");
        resp_code.add(1017, "初始发送状态", 1, "中间状态");
        resp_code.add(1018, "生成支付账单号成功", 1, "中间状态");
        resp_code.add(1019, "经办成功", 1, "中间状态");
        resp_code.add(1021, "单边账已退款", -1, "失败状态");
        resp_code.add(1022, "商户已申请", 1, "中间状态");
        resp_code.add(1026, "退款调账已申请", 1, "中间状态");
        resp_code.add(1027, "退款调账成功", 0, "成功状态");
        resp_code.add(1028, "重汇文件已下载", 1, "中间状态");
        resp_code.add(1029, "重汇确认失败", -1, "失败状态");
        resp_code.add(1030, "财务已审核", 1, "中间状态");
        resp_code.add(1049, "退款预终止", 1, "中间状态");
        resp_code.add(1050, "退款已终止", -1, "失败状态");
        resp_code.add(1099, "风险交易（渠道应答中部分信息与请求不符）", 1, "中间状态");
        resp_code.add(2001, "报文解析失败", -1, "失败状态");
        resp_code.add(2002, "无效的令牌", -1, "失败状态");
        resp_code.add(2003, "卡已过期", -1, "失败状态");
        resp_code.add(2004, "请求频繁", -1, "失败状态");
        resp_code.add(2006, "交易超时", 1, "中间状态");
        resp_code.add(2007, "获取动态验证码失败", -1, "失败状态");
        resp_code.add(2015, "IP地址非法", -1, "失败状态");
        resp_code.add(2016, "非法服务请求", -1, "失败状态");
        resp_code.add(2017, "平台校验失败", -1, "失败状态");
        resp_code.add(2018, "无效证书", -1, "失败状态");
        resp_code.add(2027, "商户支付机构信息表无记录", -1, "失败状态");
        resp_code.add(2028, "查询系统配置表无记录", -1, "失败状态");
        resp_code.add(2031, "非法商户", -1, "失败状态");
        resp_code.add(2032, "CVN2失效", -1, "失败状态");
        resp_code.add(2034, "重复退款", -1, "失败状态");
        resp_code.add(2036, "交易报文信息不一致", -1, "失败状态");
        resp_code.add(2040, "订单数据已同步，请至新菜单做退款", -1, "失败状态");
        resp_code.add(2042, "格式校验失败", -1, "失败状态");
        resp_code.add(2043, "系统异常，请查询后处理", 1, "中间状态");
        resp_code.add(2044, "基本格式检验失败-字段非空未填", -1, "失败状态");
        resp_code.add(2045, "基本格式检验失败-字段类型错误", -1, "失败状态");
        resp_code.add(2046, "基本格式检验失败-字段长度错误", -1, "失败状态");
        resp_code.add(2049, "风控受限", -1, "失败状态");
        resp_code.add(2061, "银行卡非法", -1, "失败状态");
        resp_code.add(2064, "资金不足", -1, "失败状态");
        resp_code.add(2066, "手机号已失效", -1, "失败状态");
        resp_code.add(2067, "手机号格式错误", -1, "失败状态");
        resp_code.add(2071, "无效短信码", -1, "失败状态");
        resp_code.add(2402, "原交易判定失败", -1, "失败状态");
        resp_code.add(3003, "验签失败", -1, "失败状态");
        resp_code.add(3004, "防钓鱼校验失败", -1, "失败状态");
        resp_code.add(3007, "订单有效期失效", -1, "失败状态");
        resp_code.add(3010, "商户未开通此交易类型", -1, "失败状态");
        resp_code.add(3011, "单笔交易超限", -1, "失败状态");
        resp_code.add(3012, "累计交易笔数超限", -1, "失败状态");
        resp_code.add(3013, "累计交易金额超限", -1, "失败状态");
        resp_code.add(3014, "单笔交易超限-商户", -1, "失败状态");
        resp_code.add(3015, "累计交易笔数超限-商户", -1, "失败状态");
        resp_code.add(3016, "累计交易金额超限-商户", -1, "失败状态");
        resp_code.add(3019, "分账方和订单商户非分账关系", -1, "失败状态");
        resp_code.add(3020, "分账金额与订单金额不符", -1, "失败状态");
        resp_code.add(3022, "该笔订单已经支付成功请查实", -1, "失败状态");
        resp_code.add(3034, "未找到原始交易", -1, "失败状态");
        resp_code.add(3201, "查发卡方", -1, "失败状态");
        resp_code.add(3229, "不正确的PIN", -1, "失败状态");
        resp_code.add(3233, "受限制的卡", -1, "失败状态");
        resp_code.add(3239, "允许的输入PIN次数超限", -1, "失败状态");
        resp_code.add(3249, "交换中心转发了原交易请求，但未收到发卡方应答", 1, "中间状态");
        resp_code.add(3251, "受理方状态非法", -1, "失败状态");
        resp_code.add(3264, "已发送银行", 1, "中间状态");
        resp_code.add(3270, "不支持此类卡交易", -1, "失败状态");
        resp_code.add(3285, "预授权号不匹配", -1, "失败状态");
        resp_code.add(3289, "预授权完成金额不匹配", -1, "失败状态");
        resp_code.add(3290, "预授权取消、完成交易不得超过原交易30天以上", -1, "失败状态");
        resp_code.add(3293, "交易结果未知", 1, "中间状态");
        resp_code.add(3295, "累计退货金额大于原交易金额", -1, "失败状态");
        resp_code.add(3302, "卡号与证件号码不符", -1, "失败状态");
        resp_code.add(3305, "该储种不能办理代收付业务", -1, "失败状态");
        resp_code.add(3308, "账户状态不正常", -1, "失败状态");
        resp_code.add(3309, "帐户已销户", -1, "失败状态");
        resp_code.add(3310, "账户已冻结", -1, "失败状态");
        resp_code.add(3325, "密码错误", -1, "失败状态");
        resp_code.add(3326, "户名不符", -1, "失败状态");
        resp_code.add(3333, "原交易信息记录不存在", -1, "失败状态");
        resp_code.add(3346, "银行分户存款余额不足", -1, "失败状态");
        resp_code.add(3359, "交易提交银行错误请与ChinaPay系统管理员联系", -1, "失败状态");
        resp_code.add(3389, "密码输错次数超限，请联系发卡行", -1, "失败状态");
        resp_code.add(3397, "未开通银联无卡支付业务", -1, "失败状态");
        resp_code.add(3400, "路由失败", -1, "失败状态");
        resp_code.add(3401, "地区信息错误", -1, "失败状态");
        resp_code.add(3417, "签名错误", -1, "失败状态");
        resp_code.add(3422, "撤销交易的日期不是当天的日期", -1, "失败状态");
        resp_code.add(3427, "交易金额超出待支付金额", -1, "失败状态");
        resp_code.add(3432, "处理失败", -1, "失败状态");
        resp_code.add(3441, "无流水", 1, "中间状态");
        resp_code.add(3444, "商户日期与系统日期相差超过一天，拒绝交易", -1, "失败状态");
        resp_code.add(3445, "交易币种非法", -1, "失败状态");
        resp_code.add(3446, "原交易已做过退款，撤销拒绝", -1, "失败状态");
        resp_code.add(3448, "授权码校验不一致", -1, "失败状态");
        resp_code.add(3450, "退款交易状态非失败，退款撤销拒绝", -1, "失败状态");
        resp_code.add(3452, "商户未配置所属收单机构", -1, "失败状态");
        resp_code.add(3455, "商户未关联分账商户", -1, "失败状态");
        resp_code.add(3456, "该商户不是分账商户", -1, "失败状态");
        resp_code.add(3458, "商户费用分账数据未配置", -1, "失败状态");
        resp_code.add(3459, "分账数据错误", -1, "失败状态");
        resp_code.add(3460, "分账订单已分账", -1, "失败状态");
        resp_code.add(3461, "非延时分账交易", -1, "失败状态");
        resp_code.add(3462, "非分账交易", -1, "失败状态");
        resp_code.add(3463, "批量-明细信息不匹配", -1, "失败状态");
        resp_code.add(3468, "商户未关联在该接入机构下", -1, "失败状态");
        resp_code.add(3470, "未查询到签约信息或签约信息已变更", -1, "失败状态");
        resp_code.add(3473, "渠道无流水", -1, "失败状态");
        resp_code.add(4364, "支付机构号不支持前台交易", -1, "失败状态");

        resp_code
    }

    fn add(&mut self, _code: i16, _msg: &str, _status: i16, _desc: &str) {
        self.code_map.insert(_code, _status);
    }

    pub fn get_payment_status(&self, order_status: i16) -> (PaymentStatus, i16) {
        let mut status = PaymentStatus::Processing;
        let opt_value = self.code_map.get(&order_status);
        match opt_value {
            Some(v) => { status = to_payment_status(*v) }
            None => {}
        }

        let value = from_payment_status(&status);
        (status, value)
    }
}