//! 电力法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电力法规则
pub struct ElectricityLawRules {
    metadata: RuleMetadata,
}

impl ElectricityLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电力法规则",
                "中国电力法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "电力".into()]),
        }
    }

    /// 电力建设规则
    pub fn power_construction(&self) -> Vec<&'static str> {
        vec![
            "电力建设规划",
            "电力建设项目审批",
            "电力建设安全管理",
            "电力建设环境保护",
            "电力建设征地拆迁",
            "电力建设质量控制",
            "电力建设验收程序",
            "电力建设监管机制",
        ]
    }

    /// 电力生产规则
    pub fn power_production(&self) -> Vec<&'static str> {
        vec![
            "电力生产许可",
            "电力生产安全管理",
            "电力生产质量控制",
            "电力生产环境保护",
            "电力生产技术标准",
            "电力生产节能减排",
            "电力生产应急管理",
            "电力生产监督检查",
        ]
    }

    /// 电力供应规则
    pub fn power_supply(&self) -> Vec<&'static str> {
        vec![
            "电力供应服务规范",
            "供电营业许可",
            "供电质量标准",
            "供电服务标准",
            "供电中断管理",
            "供电信息公布",
            "供电投诉处理",
            "供电安全保障",
        ]
    }

    /// 电力使用规则
    pub fn power_use(&self) -> Vec<&'static str> {
        vec![
            "用电申请程序",
            "用电合同签订",
            "用电计量管理",
            "电费缴纳规则",
            "用电安全管理",
            "节约用电要求",
            "用电违规处理",
            "用电服务权利",
        ]
    }

    /// 电力设施保护
    pub fn power_facility_protection(&self) -> Vec<&'static str> {
        vec![
            "电力设施保护区划定",
            "电力设施保护措施",
            "电力设施巡视检查",
            "电力设施隐患治理",
            "电力设施维护管理",
            "电力设施安全标识",
            "电力设施保护宣传",
            "电力设施违法查处",
        ]
    }

    /// 电力价格管理
    pub fn electricity_price(&self) -> Vec<&'static str> {
        vec![
            "电价制定原则",
            "电价分类管理",
            "电价调整程序",
            "电价信息公开",
            "电价监督检查",
            "电费收缴管理",
            "电费减免政策",
            "电价违法行为查处",
        ]
    }

    /// 电力监督管理
    pub fn electricity_supervision(&self) -> Vec<&'static str> {
        vec![
            "电力监督检查",
            "电力市场监管",
            "电力安全监管",
            "电力质量监管",
            "电力服务监管",
            "电力价格监管",
            "电力环保监管",
            "电力违法处罚",
        ]
    }

    /// 电力违法行为
    pub fn electricity_violations(&self) -> Vec<&'static str> {
        vec![
            "破坏电力设施行为",
            "盗窃电能行为",
            "危害电力安全行为",
            "扰乱供电秩序行为",
            "违法用电行为",
            "电力价格违法行为",
            "拒绝电力检查行为",
            "妨碍电力建设行为",
        ]
    }
}

impl Default for ElectricityLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ElectricityLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("electricity")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电力法规则】\n\n电力建设:\n{}\n\n电力供应:\n{}\n\n电力设施保护:\n{}\n",
            self.power_construction().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.power_supply().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.power_facility_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electricity_law_rules() {
        let rules = ElectricityLawRules::new();
        assert!(!rules.power_construction().is_empty());
        assert!(!rules.power_supply().is_empty());
    }
}
