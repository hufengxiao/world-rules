//! 缅甸拳击规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 缅甸拳击规则 (Lethwei)
pub struct LethweiRules {
    metadata: RuleMetadata,
}

impl LethweiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "缅甸拳击规则",
                "缅甸传统徒手拳击规则"
            )
            .with_origin("缅甸")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 5回合",
            "每回合3分钟",
            "无积分制",
            "KO或TKO获胜",
            "无手套比赛",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法技术",
            "肘击技术",
            "膝击技术",
            "踢法技术",
            "头撞技术",
        ]
    }

    /// 允许动作
    pub fn allowed_actions(&self) -> Vec<&'static str> {
        vec![
            "徒手拳击",
            "肘击攻击",
            "膝击攻击",
            "踢腿攻击",
            "头撞攻击",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "咬击",
            "挖眼",
            "攻击后脑",
            "地面攻击",
            "违规动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "医疗检查",
            "裁判监督",
            "比赛控制",
            "伤势处理",
            "保护规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "无拳击手套",
            "绑带缠绕",
            "比赛服装",
            "护齿",
            "传统装饰",
        ]
    }

    /// 比赛判定
    pub fn outcome(&self) -> Vec<&'static str> {
        vec![
            "KO获胜",
            "TKO获胜",
            "对手弃权",
            "裁判判定",
            "平局规则",
        ]
    }
}

impl Default for LethweiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LethweiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lethwei")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【缅甸拳击规则】\n\n\
            比赛规则:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n\n\
            比赛判定:\n{}\n",
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.outcome().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lethwei_rules() {
        let rules = LethweiRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}