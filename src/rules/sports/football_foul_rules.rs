//! 犯规与处罚规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 犯规严重程度
#[derive(Debug, Clone, Copy)]
pub enum FoulSeverity {
    /// 轻微犯规
    Minor,
    /// 一般犯规
    Normal,
    /// 严重犯规
    Serious,
    /// 暴力犯规
    Violent,
}

/// 纪律处罚类型
#[derive(Debug, Clone, Copy)]
pub enum DisciplinaryAction {
    /// 口头警告
    Warning,
    /// 黄牌警告
    YellowCard,
    /// 两黄变一红
    SecondYellowRed,
    /// 直接红牌
    DirectRedCard,
}

/// 犯规与处罚规则详解
pub struct FootballFoulRules {
    metadata: RuleMetadata,
}

impl FootballFoulRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("犯规与处罚规则详解", "足球犯规判罚和纪律处罚的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "犯规".into()]),
        }
    }

    /// 技术犯规类型
    pub fn technical_fouls(&self) -> Vec<&'static str> {
        vec![
            "踢人",
            "绊摔",
            "跳向对方",
            "冲撞",
            "打人",
            "推人",
            "抢截犯规",
            "拉扯",
            "吐唾沫",
            "手球",
        ]
    }

    /// 犯规判罚标准
    pub fn foul_punishment_standards(&self) -> Vec<&'static str> {
        vec![
            "轻微犯规:口头警告",
            "一般犯规:任意球",
            "严重犯规:黄牌+任意球",
            "暴力犯规:红牌+任意球",
            "禁区内犯规:点球",
            "累计犯规处罚",
        ]
    }

    /// 黄牌判罚条件
    pub fn yellow_card_conditions(&self) -> Vec<&'static str> {
        vec![
            "非体育行为",
            "用语言或行动表示异议",
            "连续违反规则",
            "延误比赛重新开始",
            "不遵守距离规定",
            "未经许可进入/离开场地",
            "危险动作",
            "假装受伤",
        ]
    }

    /// 红牌判罚条件
    pub fn red_card_conditions(&self) -> Vec<&'static str> {
        vec![
            "严重犯规",
            "暴力行为",
            "向他人吐唾沫",
            "故意手球阻止进球",
            "犯规阻止明显进球机会",
            "使用攻击性语言或动作",
            "同一比赛两张黄牌",
        ]
    }

    /// 累计犯规处理
    pub fn accumulated_fouls(&self) -> Vec<&'static str> {
        vec![
            "比赛记录犯规次数",
            "累计黄牌禁赛",
            "累计红牌处罚",
            "联赛累计制度",
            "杯赛累计制度",
        ]
    }

    /// 犯规后果
    pub fn foul_consequences(&self) -> Vec<&'static str> {
        vec![
            "任意球恢复比赛",
            "点球(禁区内犯规)",
            "黄牌警告记录",
            "红牌罚下",
            "禁赛处罚",
            "罚款处罚",
        ]
    }

    /// 争议犯规判定
    pub fn controversial_fouls(&self) -> Vec<&'static str> {
        vec![
            "手球判定",
            "越位判定",
            "犯规意图判定",
            "冲撞程度判定",
            "VAR辅助判定",
        ]
    }

    /// 判定犯规处罚
    pub fn determine_disciplinary_action(&self, severity: FoulSeverity) -> DisciplinaryAction {
        match severity {
            FoulSeverity::Minor => DisciplinaryAction::Warning,
            FoulSeverity::Normal => DisciplinaryAction::Warning,
            FoulSeverity::Serious => DisciplinaryAction::YellowCard,
            FoulSeverity::Violent => DisciplinaryAction::DirectRedCard,
        }
    }

    /// 计算禁赛场次(模拟)
    pub fn calculate_suspension_matches(&self, action: DisciplinaryAction) -> u8 {
        match action {
            DisciplinaryAction::Warning => 0,
            DisciplinaryAction::YellowCard => 0, // 单场黄牌不禁赛
            DisciplinaryAction::SecondYellowRed => 1, // 两黄变一红禁赛1场
            DisciplinaryAction::DirectRedCard => 3, // 直接红牌禁赛3场
        }
    }
}

impl Default for FootballFoulRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballFoulRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_foul")
    }

    fn explain(&self) -> String {
        format!(
            "【犯规与处罚规则详解】\n\n\
            技术犯规类型:\n{}\n\n\
            犯规判罚标准:\n{}\n\n\
            黄牌判罚条件:\n{}\n\n\
            红牌判罚条件:\n{}\n\n\
            犯规后果:\n{}\n",
            self.technical_fouls()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.foul_punishment_standards()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.yellow_card_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.red_card_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.foul_consequences()
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
    fn test_technical_fouls() {
        let rules = FootballFoulRules::new();
        let fouls = rules.technical_fouls();
        assert!(fouls.contains(&"踢人"));
        assert!(fouls.contains(&"手球"));
    }

    #[test]
    fn test_yellow_card_conditions() {
        let rules = FootballFoulRules::new();
        let conditions = rules.yellow_card_conditions();
        assert!(conditions.contains(&"非体育行为"));
        assert!(conditions.contains(&"延误比赛重新开始"));
    }

    #[test]
    fn test_red_card_conditions() {
        let rules = FootballFoulRules::new();
        let red = rules.red_card_conditions();
        assert!(red.contains(&"严重犯规"));
        assert!(red.contains(&"暴力行为"));
    }

    #[test]
    fn test_disciplinary_action() {
        let rules = FootballFoulRules::new();

        // 轻微犯规:警告
        assert_eq!(
            rules.determine_disciplinary_action(FoulSeverity::Minor),
            DisciplinaryAction::Warning
        );

        // 严重犯规:黄牌
        assert_eq!(
            rules.determine_disciplinary_action(FoulSeverity::Serious),
            DisciplinaryAction::YellowCard
        );

        // 暴力犯规:红牌
        assert_eq!(
            rules.determine_disciplinary_action(FoulSeverity::Violent),
            DisciplinaryAction::DirectRedCard
        );
    }

    #[test]
    fn test_suspension_matches() {
        let rules = FootballFoulRules::new();

        // 黄牌不禁赛
        assert_eq!(
            rules.calculate_suspension_matches(DisciplinaryAction::YellowCard),
            0
        );

        // 两黄变一红禁赛1场
        assert_eq!(
            rules.calculate_suspension_matches(DisciplinaryAction::SecondYellowRed),
            1
        );

        // 直接红牌禁赛3场
        assert_eq!(
            rules.calculate_suspension_matches(DisciplinaryAction::DirectRedCard),
            3
        );
    }

    #[test]
    fn test_metadata() {
        let rules = FootballFoulRules::new();
        assert_eq!(rules.metadata().name, "犯规与处罚规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_foul"));
    }
}
