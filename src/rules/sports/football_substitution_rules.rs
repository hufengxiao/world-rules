//! 换人规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 换人类型
#[derive(Debug, Clone, Copy)]
pub enum SubstitutionType {
    /// 常规换人
    Normal,
    /// 因伤换人
    Injury,
    /// 门将换人
    Goalkeeper,
    /// 红牌后换人
    AfterRedCard,
    /// 加时赛换人
    ExtraTime,
}

/// 换人规则详解
pub struct FootballSubstitutionRules {
    metadata: RuleMetadata,
}

impl FootballSubstitutionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("换人规则详解", "足球换人的完整规则和限制")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "换人".into()]),
        }
    }

    /// 基本换人规定
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "最多3名替补(正式比赛)",
            "世界杯等可增加到5名",
            "必须在替补名单中",
            "裁判许可后执行",
            "在指定区域换人",
            "被换下球员不能再上场",
        ]
    }

    /// 换人程序
    pub fn substitution_procedure(&self) -> Vec<&'static str> {
        vec![
            "先通知裁判",
            "替补球员等待",
            "被换下球员离场",
            "替补球员入场",
            "在边线中线附近",
            "裁判确认后生效",
        ]
    }

    /// 换人时机
    pub fn substitution_timing(&self) -> Vec<&'static str> {
        vec![
            "任何时候都可以换人",
            "暂停时换人",
            "球在边界时换人",
            "受伤时换人",
            "战术换人",
            "最后时刻换人",
        ]
    }

    /// 因伤换人规定
    pub fn injury_substitution(&self) -> Vec<&'static str> {
        vec![
            "受伤球员必须离场",
            "裁判评估伤情",
            "医疗人员进入",
            "尽快完成换人",
            "消耗换人名额",
            "严重伤情可额外换人",
        ]
    }

    /// 门将换人规定
    pub fn goalkeeper_substitution(&self) -> Vec<&'static str> {
        vec![
            "可以换门将",
            "消耗换人名额",
            "场上球员可以换门将",
            "更换球衣颜色",
            "通知裁判",
            "特殊装备确认",
        ]
    }

    /// 加时赛换人
    pub fn extra_time_substitution(&self) -> Vec<&'static str> {
        vec![
            "加时赛额外换人名额",
            "世界杯等赛事+1名额",
            "常规换人名额优先",
            "加时赛开始前换人",
            "加时赛期间换人",
            "不增加总名额",
        ]
    }

    /// 红牌后换人
    pub fn after_red_card(&self) -> Vec<&'static str> {
        vec![
            "红牌罚下后可换人",
            "替补替补球员",
            "保持11人上场",
            "消耗换人名额",
            "战术调整",
            "保持比赛进行",
        ]
    }

    /// 换人违规处罚
    pub fn substitution_violations(&self) -> Vec<&'static str> {
        vec![
            "未经裁判许可换人",
            "换人超时",
            "不在替补名单中",
            "被换下球员再次上场",
            "换人程序错误",
            "黄牌警告",
        ]
    }

    /// 比赛开始换人
    pub fn match_start_rules(&self) -> Vec<&'static str> {
        vec![
            "开赛前可以换人",
            "替换首发球员",
            "被替换球员进入替补",
            "不影响换人名额",
            "必须在名单中",
            "裁判确认",
        ]
    }

    /// 计算剩余换人名额
    pub fn remaining_substitutions(&self, total_allowed: u8, used: u8) -> u8 {
        total_allowed.saturating_sub(used)
    }

    /// 判定是否可以换人
    pub fn can_substitute(&self, remaining: u8, is_in_list: bool) -> bool {
        remaining > 0 && is_in_list
    }
}

impl Default for FootballSubstitutionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballSubstitutionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_substitution")
    }

    fn explain(&self) -> String {
        format!(
            "【换人规则详解】\n\n\
            基本规定:\n{}\n\n\
            换人程序:\n{}\n\n\
            换人时机:\n{}\n\n\
            因伤换人:\n{}\n\n\
            门将换人:\n{}\n",
            self.basic_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.substitution_procedure()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.substitution_timing()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.injury_substitution()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.goalkeeper_substitution()
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
    fn test_basic_rules() {
        let rules = FootballSubstitutionRules::new();
        let basics = rules.basic_rules();
        assert!(basics.contains(&"最多3名替补(正式比赛)"));
        assert!(basics.contains(&"必须在替补名单中"));
    }

    #[test]
    fn test_substitution_procedure() {
        let rules = FootballSubstitutionRules::new();
        let procedure = rules.substitution_procedure();
        assert!(procedure.contains(&"先通知裁判"));
        assert!(procedure.contains(&"裁判确认后生效"));
    }

    #[test]
    fn test_remaining_substitutions() {
        let rules = FootballSubstitutionRules::new();

        // 3换人名额，已用1个，剩余2个
        assert_eq!(rules.remaining_substitutions(3, 1), 2);

        // 5换人名额，已用5个，剩余0个
        assert_eq!(rules.remaining_substitutions(5, 5), 0);

        // 已用超过名额
        assert_eq!(rules.remaining_substitutions(3, 5), 0);
    }

    #[test]
    fn test_can_substitute() {
        let rules = FootballSubstitutionRules::new();

        // 有名额且在名单中:可以换人
        assert!(rules.can_substitute(2, true));

        // 无名额:不能换人
        assert!(!rules.can_substitute(0, true));

        // 不在名单中:不能换人
        assert!(!rules.can_substitute(2, false));
    }

    #[test]
    fn test_goalkeeper_substitution() {
        let rules = FootballSubstitutionRules::new();
        let gk_sub = rules.goalkeeper_substitution();
        assert!(gk_sub.contains(&"场上球员可以换门将"));
        assert!(gk_sub.contains(&"更换球衣颜色"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballSubstitutionRules::new();
        assert_eq!(rules.metadata().name, "换人规则详解");
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_substitution")
        );
    }
}
