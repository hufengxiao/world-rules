//! 运动精神与风度
//!
//! 体育竞技中胜不骄败不馁、尊重对手的风度

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SportsmanshipRules,
    name: "运动精神与风度",
    desc: "体育竞技中胜不骄败不馁、尊重对手的风度",
    origin: "国际",
    tags: ["体育", "运动精神", "风度", "竞技", "体育精神"]
}

impl SportsmanshipRules {
    /// 胜败态度
    pub fn win_loss(&self) -> Vec<&'static str> {
        vec![
            "赢球不炫耀奚落",
            "输球不找借口抱怨",
            "尊重对手努力",
            "庆祝有节制有分寸",
        ]
    }

    /// 尊重对手
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "赛前赛后握手致意",
            "不与对手发生肢体冲突",
            "承认对方优势与发挥",
            "有风度地赏识精彩的对抗",
        ]
    }

    /// 公平竞争
    pub fn fairness(&self) -> Vec<&'static str> {
        vec![
            "遵守赛规不自认占便宜",
            "不使用违规辅助或手段",
            "诚实面对判罚",
            "维护比赛公平公正",
        ]
    }

    /// 团队情谊
    pub fn team(&self) -> Vec<&'static str> {
        vec![
            "鼓励发挥欠佳的同伴",
            "失利不指责队友",
            "共享荣誉也共担责任",
            "尊重教练与团队",
        ]
    }
}

impl Rule for SportsmanshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sportsmanship")
    }

    fn explain(&self) -> String {
        format!(
            "【运动精神与风度】\n{}",
            [
                format!(
                    "胜败态度：\\n{}",
                    self.win_loss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重对手：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平竞争：\\n{}",
                    self.fairness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队情谊：\\n{}",
                    self.team()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_sportsmanshiprules_basic() {
        let rules = SportsmanshipRules::new();
        assert_eq!(rules.metadata().name, "运动精神与风度");
        assert!(!rules.win_loss().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.fairness().is_empty());
        assert!(!rules.team().is_empty());
    }

    #[test]
    fn test_sportsmanshiprules_validation() {
        let rules = SportsmanshipRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("sportsmanship"));
    }

    #[test]
    fn test_sportsmanshiprules_explain() {
        let rules = SportsmanshipRules::new();
        let e = rules.explain();
        assert!(e.contains("胜败态度"));
        assert!(e.contains("尊重对手"));
        assert!(e.contains("公平竞争"));
    }
}
