//! 布里斯科
//!
//! 意大利传统吃墩牌戏，以将牌与点数决胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BriscolaRules,
    name: "布里斯科",
    desc: "意大利传统吃墩牌戏，以将牌与点数决胜",
    origin: "意大利",
    tags: ["吃墩", "传统", "意大利"]
}

impl BriscolaRules {
    /// 开局准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用意大利四十张牌",
            "每人发三张牌",
            "翻一张定将牌置于牌堆下",
            "将牌花色为本局王牌",
        ]
    }

    /// 出牌吃墩
    pub fn trick(&self) -> Vec<&'static str> {
        vec![
            "每人出一张牌比大小",
            "跟出将牌可压非将牌",
            "同花色比点数大者得墩",
            "不同花色非将牌较小",
        ]
    }

    /// 补牌顺序
    pub fn drawing(&self) -> Vec<&'static str> {
        vec![
            "得墩者先补牌",
            "依次从牌堆摸牌",
            "最后一张将牌归得墩者",
            "补满手牌继续出牌",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "A计十一分三点数牌",
            "三牌计十分",
            "K计四分Q计三分J计两分",
            "其余牌不计分",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "牌堆与手牌出尽即终局",
            "统计各自赢得的墩内分值",
            "总分达六十一分者胜",
            "可分两队协作对抗",
        ]
    }
}

impl Rule for BriscolaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("briscola")
    }

    fn explain(&self) -> String {
        format!(
            "【布里斯科】\n{}",
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
                    "出牌吃墩：\\n{}",
                    self.trick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "补牌顺序：\\n{}",
                    self.drawing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分方式：\\n{}",
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
    fn test_briscolarules_basic() {
        let rules = BriscolaRules::new();
        assert_eq!(rules.metadata().name, "布里斯科");
        assert!(!rules.setup().is_empty());
        assert!(!rules.trick().is_empty());
        assert!(!rules.drawing().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_briscolarules_validation() {
        let rules = BriscolaRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("briscola"));
    }

    #[test]
    fn test_briscolarules_explain() {
        let rules = BriscolaRules::new();
        let e = rules.explain();
        assert!(e.contains("开局准备"));
        assert!(e.contains("出牌吃墩"));
        assert!(e.contains("补牌顺序"));
    }
}
