//! 门球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 门球规则
pub struct GateballRules {
    metadata: RuleMetadata,
}

impl GateballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "门球规则",
                "门球比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地: 长20-25米，宽15-20米",
            "三个球门",
            "一个终点柱",
            "沙土或人造草皮",
            "场地边界线",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每队5人",
            "比赛时间30分钟",
            "红白两队交替击球",
            "每人一球",
            "教练指挥",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "过一门得1分",
            "过二门得1分",
            "过三门得1分",
            "撞终点柱得2分",
            "满分5分",
        ]
    }

    /// 击球规则
    pub fn hitting_rules(&self) -> Vec<&'static str> {
        vec![
            "球槌击球",
            "必须在10秒内击球",
            "击球方向控制",
            "不能连击",
            "犯规处罚",
        ]
    }

    /// 战术技巧
    pub fn tactics(&self) -> Vec<&'static str> {
        vec![
            "撞击: 击中他球",
            "闪击: 被撞击球送出",
            "接力: 为队友创造机会",
            "阻挡: 干扰对手",
            "过门: 得分手段",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时击球",
            "击球动作违规",
            "触球违规",
            "出界",
            "妨碍他人",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "球槌: T字形",
            "球: 红白两色",
            "号码布",
            "运动服",
            "计分板",
        ]
    }

    /// 裁判职责
    pub fn referee_duties(&self) -> Vec<&'static str> {
        vec![
            "主裁判: 比赛指挥",
            "副裁判: 辅助判定",
            "记录员: 记分计时",
            "巡边员: 出界判定",
            "比赛监督",
        ]
    }
}

impl Default for GateballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GateballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("gateball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【门球规则】\n\n\
            场地规格:\n{}\n\n\
            得分规则:\n{}\n\n\
            战术技巧:\n{}\n\n\
            犯规规则:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tactics().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateball_rules() {
        let rules = GateballRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}