//! 妇女权益保障法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 妇女权益保障法规则
pub struct WomenProtectionLawRules {
    metadata: RuleMetadata,
}

impl WomenProtectionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "妇女权益保障法规则",
                "中国妇女权益保障法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "妇女权益".into()]),
        }
    }

    /// 妇女权益原则
    pub fn rights_principles(&self) -> Vec<&'static str> {
        vec![
            "男女平等原则",
            "特殊保护原则",
            "全面保障原则",
            "权益维护原则",
            "歧视禁止原则",
            "公平待遇原则",
            "权益救济原则",
            "社会参与原则",
        ]
    }

    /// 政治权利
    pub fn political_rights(&self) -> Vec<&'static str> {
        vec![
            "选举权保障",
            "被选举权保障",
            "参政议政权利",
            "公务员录用平等",
            "干部选拔平等",
            "人大代表保障",
            "政协委员保障",
            "基层自治参与",
        ]
    }

    /// 文化教育权益
    pub fn education_rights(&self) -> Vec<&'static str> {
        vec![
            "受教育权平等",
            "入学升学平等",
            "学业评价平等",
            "学历学位平等",
            "接受教育保障",
            "文化参与平等",
            "科技参与平等",
            "教育歧视禁止",
        ]
    }

    /// 劳动权益
    pub fn labor_rights(&self) -> Vec<&'static str> {
        vec![
            "就业权利平等",
            "招聘录用平等",
            "工资待遇平等",
            "晋升发展平等",
            "劳动保护特殊",
            "产假保护保障",
            "哺乳期保护",
            "经期保护措施",
        ]
    }

    /// 财产权益
    pub fn property_rights(&self) -> Vec<&'static str> {
        vec![
            "财产所有权平等",
            "继承权平等保障",
            "婚姻财产保护",
            "农村土地权益",
            "宅基地使用权",
            "承包经营权保护",
            "征地补偿平等",
            "财产分割公平",
        ]
    }

    /// 人身权利
    pub fn personal_rights(&self) -> Vec<&'static str> {
        vec![
            "人身自由保护",
            "生命健康保护",
            "人格尊严保护",
            "名誉荣誉保护",
            "隐私权保护",
            "肖像权保护",
            "性骚扰防治",
            "暴力侵害防治",
        ]
    }

    /// 婚姻家庭权益
    pub fn marriage_family_rights(&self) -> Vec<&'static str> {
        vec![
            "婚姻自主权利",
            "婚姻平等地位",
            "家庭事务平等",
            "生育权利保障",
            "子女抚养平等",
            "家庭暴力防治",
            "离婚权益保障",
            "监护权保障",
        ]
    }

    /// 违法行为
    pub fn protection_violations(&self) -> Vec<&'static str> {
        vec![
            "性别歧视行为",
            "就业歧视行为",
            "教育歧视行为",
            "性骚扰行为",
            "性侵害行为",
            "家庭暴力行为",
            "财产权益侵害",
            "人身权益侵害",
        ]
    }
}

impl Default for WomenProtectionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WomenProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("women_protection")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【妇女权益保障法规则】\n\n权益原则:\n{}\n\n劳动权益:\n{}\n\n人身权利:\n{}\n",
            self.rights_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.labor_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.personal_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_women_protection_law_rules() {
        let rules = WomenProtectionLawRules::new();
        assert!(!rules.rights_principles().is_empty());
        assert!(!rules.labor_rights().is_empty());
    }
}