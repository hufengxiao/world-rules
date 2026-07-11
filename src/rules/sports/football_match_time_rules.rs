//! 比赛时间规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 比赛阶段类型
#[derive(Debug, Clone, Copy)]
pub enum MatchPhase {
    /// 上半场
    FirstHalf,
    /// 下半场
    SecondHalf,
    /// 加时赛上半场
    ExtraTimeFirstHalf,
    /// 加时赛下半场
    ExtraTimeSecondHalf,
    /// 点球大战
    PenaltyShootout,
}

/// 比赛时间规则详解
pub struct FootballMatchTimeRules {
    metadata: RuleMetadata,
}

impl FootballMatchTimeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("比赛时间规则详解", "足球比赛时间和阶段的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "时间".into()]),
        }
    }

    /// 常规比赛时间
    pub fn regular_match_time(&self) -> Vec<&'static str> {
        vec![
            "上下半场各45分钟",
            "总时长90分钟",
            "中场休息15分钟",
            "时间由裁判控制",
            "比赛开始哨响",
            "比赛结束哨响",
        ]
    }

    /// 补时规定
    pub fn additional_time_rules(&self) -> Vec<&'static str> {
        vec![
            "补时弥补时间损失",
            "替换球员时间",
            "受伤处理时间",
            "其他中断时间",
            "裁判决定补时",
            "通常补时1-5分钟",
        ]
    }

    /// 加时赛规定
    pub fn extra_time_rules(&self) -> Vec<&'static str> {
        vec![
            "淘汰赛平局后加时",
            "上下半场各15分钟",
            "加时赛总时长30分钟",
            "加时赛间隔休息",
            "先进球获胜(金球制已取消)",
            "加时赛平局点球决胜",
        ]
    }

    /// 点球大战时间
    pub fn penalty_time_rules(&self) -> Vec<&'static str> {
        vec![
            "加时赛后进行",
            "每队5轮轮流",
            "单轮约2分钟",
            "总时长不定",
            "直到分出胜负",
            "可能超过10轮",
        ]
    }

    /// 比赛中断时间
    pub fn interruption_handling(&self) -> Vec<&'static str> {
        vec![
            "受伤中断",
            "犯规处理",
            "换人中断",
            "VAR审查",
            "天气中断",
            "计入补时",
        ]
    }

    /// 比赛暂停规定
    pub fn suspension_rules(&self) -> Vec<&'static str> {
        vec![
            "严重天气暂停",
            "安全原因暂停",
            "观众干扰暂停",
            "设备故障暂停",
            "裁判决定暂停",
            "暂停后继续比赛",
        ]
    }

    /// 比赛中止规定
    pub fn abandonment_rules(&self) -> Vec<&'static str> {
        vec![
            "极端天气中止",
            "安全威胁中止",
            "观众暴力中止",
            "球员数量不足",
            "裁判中止比赛",
            "中止后重新安排",
        ]
    }

    /// 时间记录方式
    pub fn time_recording(&self) -> Vec<&'static str> {
        vec![
            "裁判手表计时",
            "电子计时器",
            "电视显示时间",
            "补时显示",
            "比赛记录",
            "赛后时间统计",
        ]
    }

    /// 计算比赛总时长(分钟)
    pub fn calculate_total_duration(&self, include_extra_time: bool, include_penalty: bool) -> u16 {
        let base_time = 90 + 15; // 90分钟比赛 + 15分钟中场休息

        let extra_time = if include_extra_time {
            30 + 5 // 30分钟加时赛 + 5分钟休息
        } else {
            0
        };

        let penalty_time = if include_penalty {
            15 // 估计点球大战15分钟
        } else {
            0
        };

        base_time + extra_time + penalty_time
    }

    /// 计算补时(模拟)
    pub fn estimate_additional_time(&self, interruptions: u8) -> u8 {
        // 每次中断约增加30秒补时
        interruptions * 30 / 60 + 1 // 转换为分钟，最低1分钟
    }

    /// 判定比赛是否进入下一阶段
    pub fn should_advance_phase(&self, current_minutes: u16, phase: MatchPhase) -> bool {
        match phase {
            MatchPhase::FirstHalf => current_minutes >= 45,
            MatchPhase::SecondHalf => current_minutes >= 90,
            MatchPhase::ExtraTimeFirstHalf => current_minutes >= 105,
            MatchPhase::ExtraTimeSecondHalf => current_minutes >= 120,
            MatchPhase::PenaltyShootout => false, // 点球大战不按时间判定
        }
    }
}

impl Default for FootballMatchTimeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballMatchTimeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_match_time")
    }

    fn explain(&self) -> String {
        format!(
            "【比赛时间规则详解】\n\n\
            常规比赛时间:\n{}\n\n\
            补时规定:\n{}\n\n\
            加时赛规定:\n{}\n\n\
            点球大战时间:\n{}\n\n\
            比赛中断处理:\n{}\n",
            self.regular_match_time()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.additional_time_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.extra_time_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalty_time_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interruption_handling()
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
    fn test_regular_match_time() {
        let rules = FootballMatchTimeRules::new();
        let time = rules.regular_match_time();
        assert!(time.contains(&"上下半场各45分钟"));
        assert!(time.contains(&"中场休息15分钟"));
    }

    #[test]
    fn test_additional_time_rules() {
        let rules = FootballMatchTimeRules::new();
        let add_time = rules.additional_time_rules();
        assert!(add_time.contains(&"补时弥补时间损失"));
        assert!(add_time.contains(&"裁判决定补时"));
    }

    #[test]
    fn test_extra_time_rules() {
        let rules = FootballMatchTimeRules::new();
        let extra = rules.extra_time_rules();
        assert!(extra.contains(&"上下半场各15分钟"));
        assert!(extra.contains(&"加时赛总时长30分钟"));
    }

    #[test]
    fn test_calculate_total_duration() {
        let rules = FootballMatchTimeRules::new();

        // 常规比赛时间
        let regular = rules.calculate_total_duration(false, false);
        assert_eq!(regular, 105); // 90 + 15中场休息

        // 包含加时赛
        let with_extra = rules.calculate_total_duration(true, false);
        assert_eq!(with_extra, 140); // 90 + 15 + 30 + 5

        // 包含加时赛和点球
        let full = rules.calculate_total_duration(true, true);
        assert_eq!(full, 155); // 90 + 15 + 30 + 5 + 15
    }

    #[test]
    fn test_estimate_additional_time() {
        let rules = FootballMatchTimeRules::new();

        // 2次中断
        let few = rules.estimate_additional_time(2);
        assert!(few >= 1 && few <= 3);

        // 5次中断
        let many = rules.estimate_additional_time(5);
        assert!(many >= 2 && many <= 5);
    }

    #[test]
    fn test_should_advance_phase() {
        let rules = FootballMatchTimeRules::new();

        // 上半场45分钟时应进入下半场
        assert!(rules.should_advance_phase(45, MatchPhase::FirstHalf));

        // 上半场40分钟时不应进入下半场
        assert!(!rules.should_advance_phase(40, MatchPhase::FirstHalf));

        // 下半场90分钟时应结束
        assert!(rules.should_advance_phase(90, MatchPhase::SecondHalf));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballMatchTimeRules::new();
        assert_eq!(rules.metadata().name, "比赛时间规则详解");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_match_time")
        );
    }
}
