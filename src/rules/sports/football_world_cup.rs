//! 世界杯足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 世界杯足球规则
pub struct FootballWorldCupRules {
    metadata: RuleMetadata,
}

impl FootballWorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界杯足球规则", "FIFA世界杯比赛特殊规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "世界杯".into()]),
        }
    }

    /// 世界杯参赛资格
    pub fn qualification_rules(&self) -> Vec<&'static str> {
        vec![
            "各大洲预选赛",
            "东道主自动晋级",
            "32支球队参赛",
            "分组抽签决定",
            "种子队保护机制",
        ]
    }

    /// 小组赛规则
    pub fn group_stage_rules(&self) -> Vec<&'static str> {
        vec![
            "8个小组每组4队",
            "小组内循环赛",
            "每队3场比赛",
            "积分制排名",
            "胜3分平1分负0分",
            "同分比较规则:净胜球>进球>相互战绩",
        ]
    }

    /// 淘汰赛规则
    pub fn knockout_rules(&self) -> Vec<&'static str> {
        vec![
            "16强单场淘汰",
            "8强赛后淘汰",
            "半决赛两场",
            "决赛一场定胜负",
            "90分钟平局加时30分钟",
            "加时平局点球决胜",
        ]
    }

    /// 点球决胜规则
    pub fn penalty_rules(&self) -> Vec<&'static str> {
        vec![
            "每队5轮轮流踢",
            "5轮后分出胜负即结束",
            "5轮平局继续轮流",
            "单轮决胜制",
            "门将必须留在门线",
            "裁判监督执行",
        ]
    }

    /// 世界杯特殊规定
    pub fn special_regulations(&self) -> Vec<&'static str> {
        vec![
            "VAR技术应用",
            "替补人数增加(5人)",
            " concussion protocol (脑震荡协议)",
            "补水暂停",
            "视频助理裁判审核",
        ]
    }

    /// 冠军奖励
    pub fn championship_rewards(&self) -> Vec<&'static str> {
        vec![
            "世界杯奖杯",
            "金牌",
            "国际声誉提升",
            "FIFA排名加分",
            "下届世界杯种子队资格",
        ]
    }

    /// 纪录与统计
    pub fn records_tracking(&self) -> Vec<&'static str> {
        vec![
            "进球纪录",
            "助攻纪录",
            "出场纪录",
            "冠军次数",
            "最佳射手金靴奖",
        ]
    }
}

impl Default for FootballWorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【世界杯足球规则】\n\n\
            参赛资格:\n{}\n\n\
            小组赛规则:\n{}\n\n\
            淘汰赛规则:\n{}\n\n\
            点球决胜:\n{}\n\n\
            特殊规定:\n{}\n",
            self.qualification_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.group_stage_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.knockout_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalty_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.special_regulations()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_cup_rules() {
        let rules = FootballWorldCupRules::new();
        assert!(!rules.qualification_rules().is_empty());
        assert_eq!(rules.group_stage_rules().len(), 6);
        assert!(rules.explain().contains("世界杯"));
    }

    #[test]
    fn test_knockout_rules() {
        let rules = FootballWorldCupRules::new();
        let knockout = rules.knockout_rules();
        assert!(knockout.contains(&"加时平局点球决胜"));
    }

    #[test]
    fn test_penalty_rules() {
        let rules = FootballWorldCupRules::new();
        assert_eq!(rules.penalty_rules().len(), 6);
    }

    #[test]
    fn test_metadata() {
        let rules = FootballWorldCupRules::new();
        assert_eq!(rules.metadata().name, "世界杯足球规则");
        assert_eq!(rules.category(), RuleCategory::sports("football_world_cup"));
    }
}
