//! 物业管理条例基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 物业管理条例规则
pub struct PropertyManagementLawRules {
    metadata: RuleMetadata,
}

impl PropertyManagementLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "物业管理条例规则",
                "中国物业管理条例基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "物业管理".into()]),
        }
    }

    /// 物业管理概念
    pub fn property_management_concepts(&self) -> Vec<&'static str> {
        vec![
            "物业管理定义范围",
            "业主权利义务",
            "业主大会组成职权",
            "业主委员会职责",
            "物业服务企业资质",
            "物业管理区域划分",
            "物业管理用房配置",
            "物业管理费用构成",
        ]
    }

    /// 业主权利
    pub fn owner_rights(&self) -> Vec<&'static str> {
        vec![
            "业主共有权保障",
            "业主专有权保护",
            "业主知情权保障",
            "业主决策权行使",
            "业主监督权保障",
            "业主投诉申诉权",
            "业主收益分配权",
            "业主选举被选举权",
        ]
    }

    /// 业主大会规则
    pub fn owners_meeting(&self) -> Vec<&'static str> {
        vec![
            "业主大会成立程序",
            "业主大会召开规则",
            "业主大会议事规则",
            "业主大会表决规则",
            "业主大会决议效力",
            "业主大会信息公开",
            "业主大会变更程序",
            "业主大会监督机制",
        ]
    }

    /// 物业服务规则
    pub fn property_services(&self) -> Vec<&'static str> {
        vec![
            "物业服务合同签订",
            "物业服务内容范围",
            "物业服务标准规范",
            "物业服务质量管理",
            "物业服务费用收取",
            "物业服务信息公示",
            "物业服务投诉处理",
            "物业服务监督检查",
        ]
    }

    /// 物业维修养护
    pub fn property_maintenance(&self) -> Vec<&'static str> {
        vec![
            "物业共用部位维修",
            "物业共用设施维护",
            "物业专项维修资金",
            "物业维修资金使用",
            "物业维修资金管理",
            "物业维修养护费用",
            "物业维修责任划分",
            "物业维修监督机制",
        ]
    }

    /// 物业安全管理
    pub fn property_safety(&self) -> Vec<&'static str> {
        vec![
            "物业安全管理制度",
            "物业安全防范措施",
            "物业消防安全管理",
            "物业安全应急处理",
            "物业安全检查制度",
            "物业安全责任追究",
            "物业安全投诉处理",
            "物业安全监督检查",
        ]
    }

    /// 物业违法行为
    pub fn property_violations(&self) -> Vec<&'static str> {
        vec![
            "业主违规装修行为",
            "业主违规停车行为",
            "业主违规饲养宠物",
            "业主违规高空抛物",
            "物业服务违规行为",
            "物业收费违规行为",
            "物业维修资金违规",
            "物业安全管理违规",
        ]
    }

    /// 物业争议处理
    pub fn property_dispute_resolution(&self) -> Vec<&'static str> {
        vec![
            "物业争议协商调解",
            "物业争议仲裁处理",
            "物业争议诉讼处理",
            "物业争议行政处理",
            "物业损害赔偿规则",
            "物业责任划分认定",
            "物业证据收集保全",
            "物业救济途径保障",
        ]
    }
}

impl Default for PropertyManagementLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PropertyManagementLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("property_management")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【物业管理条例规则】\n\n业主权利:\n{}\n\n物业服务:\n{}\n\n物业维修:\n{}\n",
            self.owner_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.property_services().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.property_maintenance().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_management_law_rules() {
        let rules = PropertyManagementLawRules::new();
        assert!(!rules.owner_rights().is_empty());
        assert!(!rules.property_services().is_empty());
    }
}
