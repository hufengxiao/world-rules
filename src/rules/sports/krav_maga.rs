//! 克拉夫马伽规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 克拉夫马伽规则 (以色列自卫术)
pub struct KravMagaRules {
    metadata: RuleMetadata,
}

impl KravMagaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "克拉夫马伽规则",
                "以色列自卫格斗体系规则"
            )
            .with_origin("以色列")
            .with_tags(vec!["体育".into(), "格斗".into(), "自卫".into()]),
        }
    }

    /// 训练原则
    pub fn training_principles(&self) -> Vec<&'static str> {
        vec![
            "避免伤害为先",
            "快速有效地反击",
            "利用本能反应",
            "从简单到复杂",
            "压力下训练",
        ]
    }

    /// 技术体系
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "站立格斗技术",
            "地面格斗技术",
            "防御技术",
            "反击技术",
            "武器防御",
        ]
    }

    /// 武器防御
    pub fn weapon_defense(&self) -> Vec<&'static str> {
        vec![
            "刀具防御",
            "棍棒防御",
            "枪械防御",
            "日常物品防御",
            "多人攻击防御",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "非竞技性为主",
            "实战模拟",
            "场景训练",
            "安全保护",
            "分级训练",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "护具佩戴",
            "控制力度",
            "循序渐进",
            "医疗支持",
            "教练指导",
        ]
    }

    /// 级别体系
    pub fn belt_system(&self) -> Vec<&'static str> {
        vec![
            "学员级别",
            "实践者级别",
            "专家级别",
            "大师级别",
            "级别认证",
        ]
    }

    /// 训练装备
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "防护手套",
            "护具套装",
            "训练靶",
            "模拟武器",
            "训练服装",
        ]
    }
}

impl Default for KravMagaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KravMagaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("krav_maga")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【克拉夫马伽规则】\n\n\
            训练原则:\n{}\n\n\
            技术体系:\n{}\n\n\
            武器防御:\n{}\n\n\
            级别体系:\n{}\n",
            self.training_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.weapon_defense().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.belt_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krav_maga_rules() {
        let rules = KravMagaRules::new();
        assert!(!rules.training_principles().is_empty());
    }
}