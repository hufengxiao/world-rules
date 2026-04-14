//! 保险法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 保险法规则
pub struct InsuranceLawRules {
    metadata: RuleMetadata,
}

impl InsuranceLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "保险法规则",
                "中国保险法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "保险法".into()]),
        }
    }

    /// 保险类型
    pub fn insurance_types(&self) -> Vec<&'static str> {
        vec![
            "人身保险: 寿险健康险意外险",
            "财产保险: 财产损失险",
            "责任保险: 责任损害赔偿",
            "信用保险: 信用风险保障",
            "保证保险: 保证责任保障",
            "农业保险: 农业风险保障",
            "再保险: 保险人之间分保",
            "社会保险: 社保体系",
        ]
    }

    /// 保险合同要素
    pub fn contract_elements(&self) -> Vec<&'static str> {
        vec![
            "保险人: 承担保险责任",
            "投保人: 支付保费",
            "被保险人: 受保障人",
            "受益人: 享有保险金",
            "保险标的: 保险对象",
            "保险责任: 承保范围",
            "保险金额: 最高赔付额",
            "保险期间: 合同有效期",
        ]
    }

    /// 保险合同订立
    pub fn contract_formation(&self) -> Vec<&'static str> {
        vec![
            "投保申请: 提出投保请求",
            "保险人承保: 同意承保",
            "保险单签发: 签发保险单",
            "告知义务: 投保人如实告知",
            "说明义务: 保险人说明条款",
            "保险费支付: 缴纳保费",
            "合同生效: 符合条件生效",
            "犹豫期: 人身保险犹豫期",
        ]
    }

    /// 保险责任规则
    pub fn insurance_liability(&self) -> Vec<&'static str> {
        vec![
            "保险事故发生",
            "保险责任确定",
            "保险金赔付",
            "免赔额/免赔率",
            "责任限额规定",
            "除外责任条款",
            "保险责任免除情形",
            "代位求偿权行使",
        ]
    }

    /// 保险理赔规则
    pub fn claim_rules(&self) -> Vec<&'static str> {
        vec![
            "事故通知义务",
            "索赔申请提交",
            "理赔材料提供",
            "保险人调查核实",
            "理赔决定作出",
            "理赔时限规定",
            "保险金支付",
            "理赔争议解决",
        ]
    }

    /// 保险合同变更终止
    pub fn contract_modification(&self) -> Vec<&'static str> {
        vec![
            "合同变更: 双方协商变更",
            "保单批改: 修改保单内容",
            "保险金额调整",
            "保险费调整",
            "合同解除: 解除保险合同",
            "合同终止: 合同到期终止",
            "退保处理: 退还保险费",
            "复效: 人身保险复效",
        ]
    }

    /// 保险经营规则
    pub fn operation_rules(&self) -> Vec<&'static str> {
        vec![
            "保险公司设立许可",
            "保险经营业务范围",
            "保险资金运用",
            "偿付能力监管",
            "保险准备金提取",
            "保险费率监管",
            "保险条款监管",
            "保险中介管理",
        ]
    }

    /// 保险监管
    pub fn insurance_regulation(&self) -> Vec<&'static str> {
        vec![
            "银保监会监管",
            "保险业自律管理",
            "保险信息披露",
            "保险投诉处理",
            "保险行政处罚",
            "保险市场准入",
            "保险市场退出",
            "消费者权益保护",
        ]
    }
}

impl Default for InsuranceLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InsuranceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("insurance")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【保险法规则】\n\n保险类型:\n{}\n\n保险合同要素:\n{}\n\n保险理赔:\n{}\n",
            self.insurance_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.contract_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.claim_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insurance_law_rules() {
        let rules = InsuranceLawRules::new();
        assert!(!rules.insurance_types().is_empty());
        assert!(!rules.contract_elements().is_empty());
    }
}