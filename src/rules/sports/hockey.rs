//! 曲棍球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 曲棍球规则
pub struct HockeyRules {
    metadata: RuleMetadata,
}

impl HockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "曲棍球规则",
                "曲棍球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 场地规格
    pub fn field_dimensions(&self) -> Vec<&'static str> {
        vec![
            "球场: 长91.4米，宽55米",
            "球门: 高2.14米，宽3.66米",
            "射门圈: 半径14.63米的半圆",
            "点球点: 距球门6.4米",
            "人造草皮场地",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 11人(含守门员)",
            "替补队员: 最多5人",
            "换人次数不限",
            "可在比赛进行中换人",
            "守门员有特殊装备",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "四节比赛，每节15分钟",
            "节间休息2分钟",
            "中场休息10分钟",
            "有效时间制",
            "平局后点球决胜",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "只能用球杆击球",
            "不能用身体触球",
            "不能从右侧抢球",
            "球不能高过后角",
            "不能危险挥杆",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线得1分",
            "必须在射门圈内射门",
            "可以用推、击、挑等方式",
            "角球和点球机会",
            "得分多者获胜",
        ]
    }

    /// 处罚规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "自由球: 普通犯规",
            "短角球: 防守犯规",
            "点球: 严重犯规",
            "绿牌: 警告",
            "黄牌: 暂时离场(至少5分钟)",
            "红牌: 驱逐出场",
        ]
    }

    /// 守门员规则
    pub fn goalkeeper_rules(&self) -> Vec<&'static str> {
        vec![
            "可以在射门区内用身体挡球",
            "可以扑球",
            "不能控球超过合理时间",
            "必须佩戴护具",
            "不得故意躺在球上",
        ]
    }
}

impl Default for HockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hockey")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【曲棍球规则】\n\n\
            场地规格:\n{}\n\n\
            基本规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            处罚规则:\n{}\n",
            self.field_dimensions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hockey_rules() {
        let rules = HockeyRules::new();
        assert!(!rules.field_dimensions().is_empty());
    }
}