//! 桑搏规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 桑搏规则 (俄罗斯摔跤)
pub struct SamboRules {
    metadata: RuleMetadata,
}

impl SamboRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "桑搏规则",
                "俄罗斯武术摔跤规则"
            )
            .with_origin("俄罗斯")
            .with_tags(vec!["体育".into(), "摔跤".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "运动桑搏",
            "格斗桑搏",
            "自由桑搏",
            "特殊桑搏",
            "青少年桑搏",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 6分钟",
            "有效得分判定",
            "犯规扣分",
            "全场获胜",
            "积分获胜",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "摔法技术",
            "投掷技术",
            "地面技术",
            "擒拿技术",
            "打击技术",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完全摔倒: 4分",
            "部分摔倒: 2分",
            "地面控制: 2-4分",
            "擒拿获胜",
            "技术评分",
        ]
    }

    /// 允许动作
    pub fn allowed_actions(&self) -> Vec<&'static str> {
        vec![
            "各种摔法",
            "投掷动作",
            "地面控制",
            "擒拿技术",
            "腿部攻击",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "窒息技术",
            "关节过度扭转",
            "危险投掷",
            "打击头部",
            "违规动作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "桑搏比赛服",
            "摔跤鞋",
            "护具套装",
            "腰带标识",
            "比赛场地",
        ]
    }
}

impl Default for SamboRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SamboRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sambo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【桑搏规则】\n\n\
            比赛类型:\n{}\n\n\
            得分标准:\n{}\n\n\
            装备要求:\n{}\n\n\
            禁止动作:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sambo_rules() {
        let rules = SamboRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}