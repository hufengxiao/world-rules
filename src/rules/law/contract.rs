//! 合同法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 合同类型
#[derive(Debug, Clone)]
pub enum ContractType {
    /// 买卖合同
    Sales,
    /// 租赁合同
    Lease,
    /// 劳动合同
    Employment,
    /// 服务合同
    Service,
    /// 借款合同
    Loan,
}

impl ContractType {
    pub fn name(&self) -> &'static str {
        match self {
            ContractType::Sales => "买卖合同",
            ContractType::Lease => "租赁合同",
            ContractType::Employment => "劳动合同",
            ContractType::Service => "服务合同",
            ContractType::Loan => "借款合同",
        }
    }
}

/// 合同法规则
pub struct ContractRules {
    metadata: RuleMetadata,
}

impl ContractRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "合同法规则",
                "合同订立与履行基本规则"
            )
            .with_origin("中国合同法")
            .with_tags(vec!["法律".into(), "合同".into()]),
        }
    }

    /// 合同必备条款
    pub fn essential_terms(&self) -> Vec<&'static str> {
        vec![
            "当事人名称或姓名",
            "标的 (合同对象)",
            "数量",
            "质量",
            "价款或报酬",
            "履行期限、地点和方式",
            "违约责任",
            "争议解决方式",
        ]
    }

    /// 合同生效条件
    pub fn validity_conditions(&self) -> Vec<&'static str> {
        vec![
            "当事人具有相应民事行为能力",
            "意思表示真实",
            "内容不违反法律强制性规定",
            "不损害公共利益",
        ]
    }

    /// 无效合同情形
    pub fn invalid_situations(&self) -> Vec<&'static str> {
        vec![
            "一方以欺诈、胁迫手段订立",
            "恶意串通损害他人利益",
            "以合法形式掩盖非法目的",
            "损害社会公共利益",
            "违反法律强制性规定",
        ]
    }
}

impl Default for ContractRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ContractRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("contract")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【合同法规则】\n\n\
            合同必备条款:\n{}\n\n\
            合同生效条件:\n{}\n\n\
            无效合同情形:\n{}\n",
            self.essential_terms().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.validity_conditions().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.invalid_situations().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}