//! 慈善法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 慈善法规则
pub struct CharityLawRules {
    metadata: RuleMetadata,
}

impl CharityLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "慈善法规则",
                "中国慈善法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "慈善".into()]),
        }
    }

    /// 慈善活动范围
    pub fn charity_scope(&self) -> Vec<&'static str> {
        vec![
            "扶贫济困慈善活动",
            "扶老助残慈善活动",
            "救孤恤病慈善活动",
            "优抚救灾慈善活动",
            "教育科研慈善活动",
            "环境保护慈善活动",
            "文化艺术慈善活动",
            "公益设施建设慈善",
        ]
    }

    /// 慈善组织规则
    pub fn charity_organizations(&self) -> Vec<&'static str> {
        vec![
            "慈善组织设立条件",
            "慈善组织登记管理",
            "慈善组织信息公开",
            "慈善组织年度报告",
            "慈善组织财务管理",
            "慈善组织资产管理",
            "慈善组织终止程序",
            "慈善组织监管机制",
        ]
    }

    /// 慈善募捐规则
    pub fn charity_fundraising(&self) -> Vec<&'static str> {
        vec![
            "公开募捐资格许可",
            "募捐方案备案管理",
            "募捐信息公开要求",
            "募捐活动规范要求",
            "募捐财物使用管理",
            "募捐剩余财物处理",
            "募捐活动监督机制",
            "募捐违法行为查处",
        ]
    }

    /// 慈善捐赠规则
    pub fn charity_donation(&self) -> Vec<&'static str> {
        vec![
            "捐赠权利保障",
            "捐赠自愿原则",
            "捐赠知情权利",
            "捐赠款物使用监督",
            "捐赠票据开具",
            "捐赠税收优惠",
            "捐赠荣誉表彰",
            "捐赠隐私保护",
        ]
    }

    /// 慈善信托规则
    pub fn charity_trust(&self) -> Vec<&'static str> {
        vec![
            "慈善信托设立",
            "慈善信托备案",
            "慈善信托管理",
            "慈善信托监察",
            "慈善信托变更",
            "慈善信托终止",
            "慈善信托税收优惠",
            "慈善信托信息公开",
        ]
    }

    /// 慈善服务规则
    pub fn charity_services(&self) -> Vec<&'static str> {
        vec![
            "慈善服务开展原则",
            "慈善服务质量管理",
            "慈善服务人员培训",
            "慈善服务安全管理",
            "慈善服务信息记录",
            "慈善服务效果评估",
            "慈善服务投诉处理",
            "慈善服务改进机制",
        ]
    }

    /// 慈善信息公开
    pub fn charity_information_disclosure(&self) -> Vec<&'static str> {
        vec![
            "组织基本信息公开",
            "募捐信息公开要求",
            "捐赠款物使用公开",
            "财务信息公开要求",
            "项目实施信息公开",
            "年度工作报告公开",
            "重大事项信息公开",
            "信息公开平台建设",
        ]
    }

    /// 慈善违法行为
    pub fn charity_violations(&self) -> Vec<&'static str> {
        vec![
            "非法募捐行为",
            "欺诈募捐行为",
            "挪用慈善款物行为",
            "信息不公开违法行为",
            "虚假报告违法行为",
            "组织管理违法行为",
            "信托管理违法行为",
            "税收优惠违法行为",
        ]
    }
}

impl Default for CharityLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CharityLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("charity")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【慈善法规则】\n\n慈善活动范围:\n{}\n\n慈善募捐:\n{}\n\n慈善捐赠:\n{}\n",
            self.charity_scope().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.charity_fundraising().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.charity_donation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charity_law_rules() {
        let rules = CharityLawRules::new();
        assert!(!rules.charity_scope().is_empty());
        assert!(!rules.charity_organizations().is_empty());
    }
}
