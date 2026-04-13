//! 少林功夫规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 少林功夫规则
pub struct ShaolinKungFuRules {
    metadata: RuleMetadata,
}

impl ShaolinKungFuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "少林功夫规则",
                "少林功夫比赛基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "套路表演赛",
            "散打比赛",
            "器械比赛",
            "功夫表演",
            "团体表演",
        ]
    }

    /// 基本套路
    pub fn routine_forms(&self) -> Vec<&'static str> {
        vec![
            "少林拳基础套路",
            "罗汉拳",
            "小洪拳",
            "大洪拳",
            "长拳套路",
        ]
    }

    /// 器械类型
    pub fn weapon_forms(&self) -> Vec<&'static str> {
        vec![
            "棍术",
            "刀术",
            "剑术",
            "枪术",
            "双器械",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "动作规范: 5分",
            "演练水平: 3分",
            "难度动作: 2分",
            "创新加分",
            "扣分项目",
        ]
    }

    /// 技术要求
    pub fn technique_requirements(&self) -> Vec<&'static str> {
        vec![
            "姿势正确",
            "动作连贯",
            "力度适中",
            "节奏把握",
            "精神饱满",
        ]
    }

    /// 功夫特点
    pub fn characteristics(&self) -> Vec<&'static str> {
        vec![
            "刚柔相济",
            "内外兼修",
            "攻防结合",
            "禅武合一",
            "传统传承",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "武术服装",
            "武术器械",
            "比赛场地",
            "安全保护",
            "传统装束",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "器械安全检查",
            "场地安全",
            "医疗支持",
            "控制力度",
            "裁判监督",
        ]
    }
}

impl Default for ShaolinKungFuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShaolinKungFuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shaolin_kung_fu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【少林功夫规则】\n\n\
            基本套路:\n{}\n\n\
            评分标准:\n{}\n\n\
            技术要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.routine_forms().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shaolin_kung_fu_rules() {
        let rules = ShaolinKungFuRules::new();
        assert!(!rules.routine_forms().is_empty());
    }
}