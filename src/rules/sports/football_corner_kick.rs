//! 角球规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 角球类型
#[derive(Debug, Clone, Copy)]
pub enum CornerKickType {
    /// 左侧角球
    LeftCorner,
    /// 右侧角球
    RightCorner,
    /// 短角球
    ShortCorner,
    /// 远角球
    FarCorner,
}

/// 角球规则详解
pub struct FootballCornerKickRules {
    metadata: RuleMetadata,
}

impl FootballCornerKickRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("角球规则详解", "足球角球判罚和执行的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "角球".into()]),
        }
    }

    /// 角球判罚条件
    pub fn corner_conditions(&self) -> Vec<&'static str> {
        vec![
            "球整体越过球门线",
            "最后触球是防守方",
            "未进球",
            "在角球弧内执行",
            "裁判判定",
        ]
    }

    /// 角球执行位置
    pub fn execution_location(&self) -> Vec<&'static str> {
        vec![
            "角球弧内",
            "距角旗杆1米弧形区域",
            "四个角球弧",
            "球必须完全在弧内",
            "不能移动角旗",
        ]
    }

    /// 角球执行程序
    pub fn execution_procedure(&self) -> Vec<&'static str> {
        vec![
            "球放置在角球弧",
            "确认防守球员距离",
            "一脚完成角球",
            "球移动即视为执行",
            "其他球员可以站位",
        ]
    }

    /// 防守方规定
    pub fn defense_rules(&self) -> Vec<&'static str> {
        vec![
            "距角球弧至少9.15米",
            "不能阻挡角球执行",
            "可以在门线站位",
            "人墙布置",
            "门将防守准备",
        ]
    }

    /// 进攻方策略
    pub fn attack_strategies(&self) -> Vec<&'static str> {
        vec![
            "高空传中",
            "短角球配合",
            "远角球射门",
            "角球战术配合",
            "球员站位安排",
        ]
    }

    /// 角球犯规情况
    pub fn foul_situations(&self) -> Vec<&'static str> {
        vec![
            "防守方距离不足",
            "角球执行者重复触球",
            "角球弧位置不正确",
            "移动角旗杆",
            "危险动作犯规",
        ]
    }

    /// 角球统计
    pub fn corner_statistics(&self) -> Vec<&'static str> {
        vec![
            "角球次数统计",
            "角球进球率约3-5%",
            "角球机会创造",
            "角球防守成功率",
            "角球战术效果",
        ]
    }

    /// 角球后续
    pub fn corner_outcomes(&self) -> Vec<&'static str> {
        vec![
            "进球:比赛继续",
            "防守成功:比赛继续",
            "再次角球:防守方再次触球",
            "门球:进攻方触球出界",
            "犯规:重新判罚",
        ]
    }

    /// 计算角球进球概率(模拟)
    pub fn calculate_goal_probability(&self) -> f32 {
        0.03 // 角球直接进球概率约3%
    }
}

impl Default for FootballCornerKickRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballCornerKickRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_corner")
    }

    fn explain(&self) -> String {
        format!(
            "【角球规则详解】\n\n\
            角球判罚条件:\n{}\n\n\
            执行位置:\n{}\n\n\
            执行程序:\n{}\n\n\
            防守方规定:\n{}\n\n\
            进攻方策略:\n{}\n",
            self.corner_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution_location()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution_procedure()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.defense_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.attack_strategies()
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
    fn test_corner_conditions() {
        let rules = FootballCornerKickRules::new();
        let conditions = rules.corner_conditions();
        assert!(conditions.contains(&"球整体越过球门线"));
        assert!(conditions.contains(&"最后触球是防守方"));
    }

    #[test]
    fn test_execution_location() {
        let rules = FootballCornerKickRules::new();
        assert_eq!(rules.execution_location().len(), 5);
        assert!(rules.execution_location().contains(&"角球弧内"));
    }

    #[test]
    fn test_goal_probability() {
        let rules = FootballCornerKickRules::new();
        let prob = rules.calculate_goal_probability();
        assert!(prob > 0.0 && prob < 0.1);
    }

    #[test]
    fn test_foul_situations() {
        let rules = FootballCornerKickRules::new();
        let fouls = rules.foul_situations();
        assert!(fouls.contains(&"防守方距离不足"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballCornerKickRules::new();
        assert_eq!(rules.metadata().name, "角球规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_corner"));
    }
}
