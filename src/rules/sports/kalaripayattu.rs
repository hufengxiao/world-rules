//! 卡拉里帕亚特规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 卡拉里帕亚特规则 (印度传统武术)
pub struct KalaripayattuRules {
    metadata: RuleMetadata,
}

impl KalaripayattuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "卡拉里帕亚特规则",
                "印度传统武术规则"
            )
            .with_origin("印度")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 武术流派
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "北方流派",
            "南方流派",
            "中央流派",
            "各邦流派",
            "家族传承",
        ]
    }

    /// 训练阶段
    pub fn training_levels(&self) -> Vec<&'static str> {
        vec![
            "身体训练",
            "空手技术",
            "武器训练",
            "医学知识",
            "精神修养",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法踢法",
            "跳跃技巧",
            "摔法擒拿",
            "压力点技术",
            "武器技法",
        ]
    }

    /// 武器体系
    pub fn weapons(&self) -> Vec<&'static str> {
        vec![
            "短棍",
            "长棍",
            "弯刀",
            "盾牌组合",
            "双手武器",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "表演比赛",
            "对练比赛",
            "武器表演",
            "评分标准",
            "安全规则",
        ]
    }

    /// 医学结合
    pub fn medical_system(&self) -> Vec<&'static str> {
        vec![
            "按摩技术",
            "草药疗法",
            "伤势处理",
            "体能恢复",
            "保健知识",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "传统服装",
            "训练武器",
            "护具套装",
            "训练场地",
            "油脂涂抹",
        ]
    }
}

impl Default for KalaripayattuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KalaripayattuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kalaripayattu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【卡拉里帕亚特规则】\n\n\
            武术流派:\n{}\n\n\
            训练阶段:\n{}\n\n\
            武器体系:\n{}\n\n\
            医学结合:\n{}\n",
            self.styles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.training_levels().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.weapons().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.medical_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kalaripayattu_rules() {
        let rules = KalaripayattuRules::new();
        assert!(!rules.styles().is_empty());
    }
}