//! 手球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 手球规则
pub struct HandballRules {
    metadata: RuleMetadata,
}

impl HandballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "手球规则",
                "手球比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 场地规格
    pub fn court_dimensions(&self) -> Vec<&'static str> {
        vec![
            "球场: 长40米，宽20米",
            "球门区: 半径6米的半圆",
            "球门: 高2米，宽3米",
            "七米线: 点球位置",
            "自由掷球线: 9米虚线",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 7人(含守门员)",
            "替补队员: 最多7人",
            "换人次数不限",
            "必须随时可以换人",
            "守门员可以换成场上球员",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "上下半场各30分钟",
            "中场休息15分钟",
            "青少年比赛时间较短",
            "平局时加时赛(上下半场各5分钟)",
            "加时后平局则点球决胜",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "用手传球和射门",
            "持球最多3秒",
            "最多走3步",
            "可以运球",
            "不能进入球门区(守门员除外)",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线得1分",
            "可在任何位置射门",
            "球门区外射门",
            "快攻是重要得分方式",
            "得分多者获胜",
        ]
    }

    /// 犯规与处罚
    pub fn fouls_penalties(&self) -> Vec<&'static str> {
        vec![
            "普通犯规: 自由掷球",
            "严重犯规: 七米球(点球)",
            "黄牌: 警告",
            "红牌: 驱逐出场",
            "两分钟罚下: 临时减员",
        ]
    }

    /// 守门员规则
    pub fn goalkeeper_rules(&self) -> Vec<&'static str> {
        vec![
            "守门员可在球门区内触球",
            "可用身体任何部位挡球",
            "离开球门区后等同普通球员",
            "不能持球出球门区",
            "不能将球传回球门区",
        ]
    }
}

impl Default for HandballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HandballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("handball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【手球规则】\n\n\
            场地规格:\n{}\n\n\
            基本规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            犯规与处罚:\n{}\n",
            self.court_dimensions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls_penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handball_rules() {
        let rules = HandballRules::new();
        assert!(!rules.court_dimensions().is_empty());
    }
}