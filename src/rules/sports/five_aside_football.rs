//! 五人制足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 五人制足球规则
pub struct FiveAsideFootballRules {
    metadata: RuleMetadata,
}

impl FiveAsideFootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "五人制足球规则",
                "五人制足球比赛基本规则"
            )
            .with_origin("巴西")
            .with_tags(vec!["体育".into(), "足球".into()]),
        }
    }

    /// 球场规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 25-42米",
            "宽度: 15-25米",
            "禁区半径6米",
            "罚球点距门6米",
            "门宽: 3米×高2米",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队5人上场",
            "1名守门员",
            "4名场上球员",
            "替补最多7人",
            "换人次数不限",
        ]
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "两半场各20分钟",
            "半场休息15分钟",
            "有效时间制",
            "裁判控制时间",
            "加时赛可选",
        ]
    }

    /// 球规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "4号球",
            "重量较轻",
            "材质要求",
            "弹性适中",
            "比赛专用球",
        ]
    }

    /// 换人规则
    pub fn substitution_rules(&self) -> Vec<&'static str> {
        vec![
            "换人次数不限",
            "随时换人",
            "换下可换上",
            "指定换人区",
            "无需裁判暂停",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "无越位规则",
            "累积犯规制",
            "每半场5次后罚球",
            "黄牌红牌",
            "直接罚球",
        ]
    }

    /// 守门员规则
    pub fn goalkeeper_rules(&self) -> Vec<&'static str> {
        vec![
            "手球限制4秒",
            "回传球限制",
            "禁区规则",
            "发球规则",
            "控球时间限制",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "室内比赛",
            "室外比赛",
            "世界杯",
            "欧洲杯",
            "国内联赛",
        ]
    }
}

impl Default for FiveAsideFootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FiveAsideFootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("five_aside_football")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【五人制足球规则】\n\n\
            球场规格:\n{}\n\n\
            换人规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            守门员规则:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.substitution_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.goalkeeper_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_aside_football_rules() {
        let rules = FiveAsideFootballRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}