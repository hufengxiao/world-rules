//! 未成年人保护法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 未成年人保护法规则
pub struct ChildrenProtectionLawRules {
    metadata: RuleMetadata,
}

impl ChildrenProtectionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "未成年人保护法规则",
                "中国未成年人保护法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "未成年人保护".into()]),
        }
    }

    /// 未成年人权利
    pub fn children_rights(&self) -> Vec<&'static str> {
        vec![
            "生存权: 生命生存权利",
            "发展权: 发展成长权利",
            "受保护权: 保护权利",
            "参与权: 参与表达权利",
            "受教育权: 教育权利",
            "健康权: 健康保护权利",
            "人身权利保护",
            "隐私权利保护",
        ]
    }

    /// 家庭保护
    pub fn family_protection(&self) -> Vec<&'static str> {
        vec![
            "监护职责: 家长监护义务",
            "抚养义务: 抚养养育责任",
            "教育引导义务",
            "尊重未成年人意见",
            "禁止家庭暴力",
            "禁止虐待遗弃",
            "家庭教育指导",
            "家庭监护监督",
        ]
    }

    /// 学校保护
    pub fn school_protection(&self) -> Vec<&'static str> {
        vec![
            "安全教育责任",
            "防欺凌措施",
            "心理健康教育",
            "生命安全教育",
            "性教育教育",
            "校园安全管理",
            "教职工行为规范",
            "学生权益保护",
        ]
    }

    /// 社会保护
    pub fn social_protection(&self) -> Vec<&'static str> {
        vec![
            "公共场所保护",
            "网络保护措施",
            "文化产品保护",
            "娱乐场所限制",
            "烟酒销售禁止",
            "用工保护限制",
            "社会福利保障",
            "社会救助服务",
        ]
    }

    /// 网络保护
    pub fn network_protection(&self) -> Vec<&'static str> {
        vec![
            "网络内容管理",
            "网络游戏限制",
            "网络直播限制",
            "网络社交保护",
            "个人信息保护",
            "网络沉迷预防",
            "网络欺凌防治",
            "网络投诉处理",
        ]
    }

    /// 政府保护
    pub fn government_protection(&self) -> Vec<&'static str> {
        vec![
            "未成年人工作机制",
            "未成年人发展规划",
            "未成年人福利政策",
            "未成年人救助保护",
            "未成年人安置措施",
            "未成年人监护干预",
            "未成年人安全保护",
            "未成年人权益维护",
        ]
    }

    /// 司法保护
    pub fn judicial_protection(&self) -> Vec<&'static str> {
        vec![
            "未成年人案件审理",
            "未成年人司法程序",
            "未成年人法律援助",
            "未成年人隐私保护",
            "未成年人犯罪预防",
            "未成年人矫正教育",
            "未成年人犯罪记录封存",
            "未成年人安置帮教",
        ]
    }

    /// 违法行为
    pub fn protection_violations(&self) -> Vec<&'static str> {
        vec![
            "虐待未成年人行为",
            "遗弃未成年人行为",
            "拐卖未成年人行为",
            "性侵害未成年人",
            "校园欺凌行为",
            "雇佣童工行为",
            "向未成年人售烟酒",
            "网络侵害未成年人",
        ]
    }
}

impl Default for ChildrenProtectionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChildrenProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("children_protection")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【未成年人保护法规则】\n\n未成年人权利:\n{}\n\n家庭保护:\n{}\n\n学校保护:\n{}\n",
            self.children_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.family_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.school_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_children_protection_law_rules() {
        let rules = ChildrenProtectionLawRules::new();
        assert!(!rules.children_rights().is_empty());
        assert!(!rules.family_protection().is_empty());
    }
}