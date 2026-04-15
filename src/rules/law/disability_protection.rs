//! 残疾人保障法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 残疾人保障法规则
pub struct DisabilityProtectionLawRules {
    metadata: RuleMetadata,
}

impl DisabilityProtectionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "残疾人保障法规则",
                "中国残疾人保障法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "残疾人保障".into()]),
        }
    }

    /// 残疾人权利
    pub fn disability_rights(&self) -> Vec<&'static str> {
        vec![
            "平等权利: 平等参与权利",
            "就业权利: 就业工作权利",
            "教育权利: 受教育权利",
            "康复权利: 康复服务权利",
            "文化权利: 文化参与权利",
            "社会保障权利",
            "无障碍权利: 无障碍环境",
            "信息获取权利",
        ]
    }

    /// 康复服务
    pub fn rehabilitation_services(&self) -> Vec<&'static str> {
        vec![
            "医疗康复服务",
            "教育康复服务",
            "职业康复服务",
            "社会康复服务",
            "康复训练指导",
            "康复辅助器具",
            "康复机构建设",
            "康复人才培养",
        ]
    }

    /// 教育保障
    pub fn education_support(&self) -> Vec<&'static str> {
        vec![
            "义务教育保障",
            "特殊教育学校",
            "随班就读支持",
            "教育资助政策",
            "职业教育培训",
            "高等教育支持",
            "终身教育服务",
            "教育无障碍支持",
        ]
    }

    /// 就业保障
    pub fn employment_support(&self) -> Vec<&'static str> {
        vec![
            "就业歧视禁止",
            "就业配额制度",
            "就业培训服务",
            "创业扶持政策",
            "就业服务机构",
            "工作环境改善",
            "岗位适配支持",
            "劳动权益保护",
        ]
    }

    /// 社会保障
    pub fn social_security(&self) -> Vec<&'static str> {
        vec![
            "社会保险保障",
            "社会救助服务",
            "社会福利政策",
            "护理补贴制度",
            "生活补助政策",
            "医疗费用减免",
            "住房保障支持",
            "养老服务体系",
        ]
    }

    /// 无障碍环境
    pub fn accessible_environment(&self) -> Vec<&'static str> {
        vec![
            "无障碍设施建设",
            "无障碍交通服务",
            "无障碍信息交流",
            "公共建筑无障碍",
            "住宅无障碍改造",
            "信息无障碍服务",
            "公共服务无障碍",
            "应急服务无障碍",
        ]
    }

    /// 文化体育
    pub fn culture_sports(&self) -> Vec<&'static str> {
        vec![
            "文化活动支持",
            "体育健身服务",
            "艺术创作扶持",
            "残奥会支持",
            "文化设施优惠",
            "体育设施优惠",
            "文化体育补贴",
            "残疾人艺术团",
        ]
    }

    /// 违法行为
    pub fn protection_violations(&self) -> Vec<&'static str> {
        vec![
            "歧视残疾人行为",
            "拒绝提供无障碍",
            "侵害就业权益",
            "侵害教育权益",
            "侵害康复权益",
            "侵害社会保障权益",
            "侵害人身权益",
            "侵害财产权益",
        ]
    }
}

impl Default for DisabilityProtectionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DisabilityProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("disability_protection")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人保障法规则】\n\n残疾人权利:\n{}\n\n康复服务:\n{}\n\n就业保障:\n{}\n",
            self.disability_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rehabilitation_services().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.employment_support().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disability_protection_law_rules() {
        let rules = DisabilityProtectionLawRules::new();
        assert!(!rules.disability_rights().is_empty());
        assert!(!rules.rehabilitation_services().is_empty());
    }
}
