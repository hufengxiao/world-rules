//! 点球规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 点球执行状态
#[derive(Debug, Clone)]
pub struct PenaltyKickState {
    /// 罚球球员
    pub kicker_number: u8,
    /// 门将
    pub goalkeeper_number: u8,
    /// 是否进球
    pub scored: bool,
    /// 罚球类型
    pub kick_type: PenaltyKickType,
}

/// 点球类型
#[derive(Debug, Clone, Copy)]
pub enum PenaltyKickType {
    /// 常规比赛点球
    RegularMatch,
    /// 点球大战
    Shootout,
    /// 误判重罚
    Retake,
}

/// 点球规则详解
pub struct FootballPenaltyRules {
    metadata: RuleMetadata,
}

impl FootballPenaltyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("点球规则详解", "足球点球判罚和执行的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "点球".into()]),
        }
    }

    /// 点球判罚条件
    pub fn penalty_conditions(&self) -> Vec<&'static str> {
        vec![
            "禁区内犯规",
            "防守方犯规",
            "可判罚任意球的犯规",
            "犯规阻止明显进球机会",
            "裁判判定",
        ]
    }

    /// 点球执行程序
    pub fn execution_procedure(&self) -> Vec<&'static str> {
        vec![
            "球放置在点球点(11米)",
            "罚球球员确认",
            "门将必须留在门线上",
            "其他球员退至禁区外",
            "裁判哨响后执行",
            "一脚完成罚球",
        ]
    }

    /// 门将行为规则
    pub fn goalkeeper_rules(&self) -> Vec<&'static str> {
        vec![
            "必须站在门线上",
            "面向罚球球员",
            "哨响前不能移动",
            "可以横向移动",
            "扑救后可以继续行动",
            "违规重罚点球",
        ]
    }

    /// 罚球球员规则
    pub fn kicker_rules(&self) -> Vec<&'static str> {
        vec![
            "必须确认身份",
            "哨响前不能触球",
            "一脚完成罚球",
            "不能假动作欺骗门将",
            "罚球后不能再次触球",
            "违规判罚无效",
        ]
    }

    /// 其他球员位置
    pub fn other_players_position(&self) -> Vec<&'static str> {
        vec![
            "至少距点球点9.15米",
            "在禁区外",
            "在罚球弧后",
            "哨响后才能进入",
            "违规可能重罚",
        ]
    }

    /// 点球大战规则
    pub fn shootout_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛平局后进行",
            "每队5轮轮流",
            "5轮后分出胜负结束",
            "平局继续轮流",
            "全员必须罚过才能重复",
            "门将可以罚点球",
        ]
    }

    /// 重罚情况
    pub fn retake_situations(&self) -> Vec<&'static str> {
        vec![
            "门将提前移动",
            "其他球员违规进入",
            "罚球球员假动作",
            "外力干扰",
            "裁判中止罚球",
        ]
    }

    /// 点球结果处理
    pub fn result_handling(&self) -> Vec<&'static str> {
        vec![
            "进球:比赛继续",
            "未进球:比赛继续(常规点球)",
            "未进球:下一轮(点球大战)",
            "扑救:门将可继续防守",
            "反弹:罚球者不能补射",
        ]
    }

    /// 计算点球成功率(模拟)
    pub fn calculate_success_rate(&self, kick_type: PenaltyKickType) -> f32 {
        match kick_type {
            PenaltyKickType::RegularMatch => 0.75, // 常规比赛点球成功率较高
            PenaltyKickType::Shootout => 0.70,     // 点球大战心理压力大
            PenaltyKickType::Retake => 0.60,       // 重罚成功率较低
        }
    }
}

impl Default for FootballPenaltyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballPenaltyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_penalty")
    }

    fn explain(&self) -> String {
        format!(
            "【点球规则详解】\n\n\
            点球判罚条件:\n{}\n\n\
            执行程序:\n{}\n\n\
            门将规则:\n{}\n\n\
            罚球球员规则:\n{}\n\n\
            点球大战规则:\n{}\n",
            self.penalty_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution_procedure()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.goalkeeper_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kicker_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.shootout_rules()
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
    fn test_penalty_conditions() {
        let rules = FootballPenaltyRules::new();
        let conditions = rules.penalty_conditions();
        assert!(conditions.contains(&"禁区内犯规"));
        assert!(conditions.contains(&"防守方犯规"));
    }

    #[test]
    fn test_goalkeeper_rules() {
        let rules = FootballPenaltyRules::new();
        assert_eq!(rules.goalkeeper_rules().len(), 6);
        assert!(rules.goalkeeper_rules().contains(&"必须站在门线上"));
    }

    #[test]
    fn test_success_rate() {
        let rules = FootballPenaltyRules::new();
        let regular_rate = rules.calculate_success_rate(PenaltyKickType::RegularMatch);
        let shootout_rate = rules.calculate_success_rate(PenaltyKickType::Shootout);

        // 常规比赛点球成功率高于点球大战
        assert!(regular_rate > shootout_rate);
        assert!(regular_rate > 0.7 && regular_rate < 0.8);
    }

    #[test]
    fn test_shootout_rules() {
        let rules = FootballPenaltyRules::new();
        let shootout = rules.shootout_rules();
        assert!(shootout.contains(&"每队5轮轮流"));
        assert!(shootout.contains(&"门将可以罚点球"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballPenaltyRules::new();
        assert_eq!(rules.metadata().name, "点球规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_penalty"));
    }
}
