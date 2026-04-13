//! 法式踢拳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 法式踢拳规则 (Savate)
pub struct SavateRules {
    metadata: RuleMetadata,
}

impl SavateRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "法式踢拳规则",
                "法国传统踢拳规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 3-5回合",
            "每回合2分钟",
            "有效得分判定",
            "犯规扣分",
            "裁判评分",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法技术",
            "踢法技术",
            "步法移动",
            "防守技术",
            "组合技法",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效拳法: 1分",
            "有效踢法: 2分",
            "高踢得分: 4分",
            "扫腿得分: 2分",
            "技术扣分",
        ]
    }

    /// 允许动作
    pub fn allowed_actions(&self) -> Vec<&'static str> {
        vec![
            "拳法打击",
            "踢腿攻击",
            "扫腿技术",
            "闪避防守",
            "步法移动",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "肘击",
            "膝击",
            "头撞",
            "摔法",
            "地面攻击",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "法式踢拳鞋",
            "拳击手套",
            "防护服装",
            "头盔护具",
            "护齿护具",
        ]
    }

    /// 级别体系
    pub fn glove_system(&self) -> Vec<&'static str> {
        vec![
            "蓝手套: 初级",
            "绿手套: 中级",
            "红手套: 高级",
            "白手套: 专家",
            "黄手套: 大师",
        ]
    }
}

impl Default for SavateRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SavateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("savate")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【法式踢拳规则】\n\n\
            比赛规则:\n{}\n\n\
            得分标准:\n{}\n\n\
            装备要求:\n{}\n\n\
            级别体系:\n{}\n",
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.glove_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_savate_rules() {
        let rules = SavateRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}