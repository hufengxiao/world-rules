//! 斯科普
//!
//! 意大利扫牌游戏，用手中牌吃走桌面牌并扫清得额外分

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScopaRules,
    name: "斯科普",
    desc: "意大利扫牌游戏，用手中牌吃走桌面牌并扫清得额外分",
    origin: "意大利",
    tags: ["吃牌", "传统", "意大利"]
}

impl ScopaRules {
    /// 开局准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用意大利四十张牌",
            "桌面上摊开四张牌",
            "每人发三张手牌",
            "牌堆剩余作摸牌用",
        ]
    }

    /// 出牌吃牌
    pub fn capturing(&self) -> Vec<&'static str> {
        vec![
            "出一张牌吃桌面同点数牌",
            "或用牌点数之和吃多张",
            "吃不到则牌留在桌上",
            "吃走的牌收归己方",
        ]
    }

    /// 扫清得分
    pub fn sweep(&self) -> Vec<&'static str> {
        vec![
            "一次吃光桌面所有牌称扫清",
            "每次扫清得一分",
            "末张牌吃光不计扫清",
            "扫清为额外奖励分",
        ]
    }

    /// 终局计分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "吃牌最多者得一分",
            "方块最多者得一分",
            "七牌最多者得一分",
            "七分与扫清分相加",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "手牌与牌堆出尽即终局",
            "各分项逐条比对计分",
            "先达十一分者获胜",
            "可两人或四人对战",
        ]
    }
}

impl Rule for ScopaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("scopa")
    }

    fn explain(&self) -> String {
        format!(
            "【斯科普】\n{}",
            [
                format!(
                    "开局准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出牌吃牌：\\n{}",
                    self.capturing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "扫清得分：\\n{}",
                    self.sweep()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "终局计分：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_scoparules_basic() {
        let rules = ScopaRules::new();
        assert_eq!(rules.metadata().name, "斯科普");
        assert!(!rules.setup().is_empty());
        assert!(!rules.capturing().is_empty());
        assert!(!rules.sweep().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_scoparules_validation() {
        let rules = ScopaRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("scopa"));
    }

    #[test]
    fn test_scoparules_explain() {
        let rules = ScopaRules::new();
        let e = rules.explain();
        assert!(e.contains("开局准备"));
        assert!(e.contains("出牌吃牌"));
        assert!(e.contains("扫清得分"));
    }
}
