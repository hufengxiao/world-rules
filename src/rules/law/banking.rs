//! 银行法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 银行法规则
pub struct BankingLawRules {
    metadata: RuleMetadata,
}

impl BankingLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "银行法规则",
                "中国银行法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "银行法".into()]),
        }
    }

    /// 银行类型
    pub fn bank_types(&self) -> Vec<&'static str> {
        vec![
            "中央银行: 中国人民银行",
            "商业银行: 国有股份制城商等",
            "政策性银行: 开发性政策性",
            "农村金融机构: 农商行村镇银行",
            "外资银行: 外资银行分行",
            "民营银行: 民营资本银行",
            "互联网银行: 网络银行业务",
            "非银行金融机构: 金融公司等",
        ]
    }

    /// 银行业务范围
    pub fn banking_business(&self) -> Vec<&'static str> {
        vec![
            "吸收公众存款",
            "发放贷款",
            "办理结算",
            "票据承兑贴现",
            "发行金融债券",
            "代理发行兑付债券",
            "买卖政府债券",
            "同业拆借",
            "银行卡业务",
            "信用证服务",
            "担保业务",
            "保管箱服务",
        ]
    }

    /// 存款业务规则
    pub fn deposit_rules(&self) -> Vec<&'static str> {
        vec![
            "存款利率: 央行基准利率",
            "存款期限: 活期定期",
            "存款金额: 最低存款额",
            "存款凭证: 存单存折",
            "存款保险: 50万以内保障",
            "存款实名制: 身份验证",
            "存款继承: 存款继承规则",
            "存款查询: 司法查询冻结",
        ]
    }

    /// 贷款业务规则
    pub fn loan_rules(&self) -> Vec<&'static str> {
        vec![
            "贷款利率: 浮动利率制度",
            "贷款期限: 短中长期",
            "贷款用途: 指定用途",
            "贷款担保: 担保方式",
            "贷款审批: 审批流程",
            "贷款限额: 贷款比例限制",
            "贷款偿还: 按期偿还",
            "逾期处理: 逾期违约责任",
            "贷款展期: 延期申请",
            "不良贷款: 催收处置",
        ]
    }

    /// 银行监管规则
    pub fn banking_regulation(&self) -> Vec<&'static str> {
        vec![
            "银保监会监管",
            "资本充足率监管",
            "流动性监管",
            "风险分类监管",
            "大额风险暴露限制",
            "关联交易监管",
            "信息披露要求",
            "内部控制要求",
        ]
    }

    /// 银行风险管理
    pub fn risk_management(&self) -> Vec<&'static str> {
        vec![
            "信用风险: 借款人违约风险",
            "市场风险: 市场波动风险",
            "操作风险: 操作失误风险",
            "流动性风险: 资金周转风险",
            "法律风险: 法律纠纷风险",
            "声誉风险: 信誉损失风险",
            "信息科技风险: 系统风险",
            "系统性风险: 金融系统风险",
        ]
    }

    /// 消费者保护规则
    pub fn consumer_protection(&self) -> Vec<&'static str> {
        vec![
            "信息披露义务",
            "收费透明规则",
            "投诉处理机制",
            "金融消费者教育",
            "个人金融信息保护",
            "账户安全保障",
            "电子银行安全",
            "投资者适当性管理",
        ]
    }

    /// 反洗钱规则
    pub fn anti_money_laundering(&self) -> Vec<&'static str> {
        vec![
            "客户身份识别",
            "客户身份资料保存",
            "交易记录保存",
            "大额交易报告",
            "可疑交易报告",
            "客户风险分类",
            "反洗钱内控制度",
            "反洗钱培训",
        ]
    }
}

impl Default for BankingLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BankingLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("banking")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【银行法规则】\n\n银行类型:\n{}\n\n存款规则:\n{}\n\n贷款规则:\n{}\n",
            self.bank_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.deposit_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.loan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banking_law_rules() {
        let rules = BankingLawRules::new();
        assert!(!rules.bank_types().is_empty());
        assert!(!rules.deposit_rules().is_empty());
    }
}