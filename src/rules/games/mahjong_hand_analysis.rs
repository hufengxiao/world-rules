//! 麻将手牌判断基础
//!
//! 麻将听牌、番数计算与手牌分析的基础

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongHandAnalysisRules,
    name: "麻将手牌判断基础",
    desc: "麻将听牌、番数计算与手牌分析的基础",
    origin: "中国",
    tags: ["游戏", "麻将", "手牌", "听牌"]
}

impl MahjongHandAnalysisRules {
    /// 牌组构成
    pub fn structure(&self) -> Vec<&'static str> {
        vec![
            "顺子是相邻三张",
            "刻子是相同三张",
            "将牌是成对两张",
            "和牌需基本牌型",
        ]
    }

    /// 听牌判断
    pub fn waiting(&self) -> Vec<&'static str> {
        vec![
            "寻找可和牌张",
            "计算听牌数量",
            "权衡安全牌选择",
            "统筹取舍听口",
        ]
    }

    /// 番数计算
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "牌型决定番种",
            "累计番数定分数",
            "役种各不相同",
            "依规则计分算番",
        ]
    }

    /// 和牌流程
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "正确报听喊和",
            "亮牌供验牌",
            "点炮与否定胜负",
            "遵守顺位规则",
        ]
    }
}

impl Rule for MahjongHandAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hand")
    }

    fn explain(&self) -> String {
        format!(
            "【麻将手牌判断基础】\n{}",
            [
                format!(
                    "牌组构成：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "听牌判断：\\n{}",
                    self.waiting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "番数计算：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "和牌流程：\\n{}",
                    self.winning()
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
    fn test_mahjonghandanalysisrules_basic() {
        let rules = MahjongHandAnalysisRules::new();
        assert_eq!(rules.metadata().name, "麻将手牌判断基础");
        assert!(!rules.structure().is_empty());
        assert!(!rules.waiting().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_mahjonghandanalysisrules_validation() {
        let rules = MahjongHandAnalysisRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("mahjong_hand"));
    }

    #[test]
    fn test_mahjonghandanalysisrules_explain() {
        let rules = MahjongHandAnalysisRules::new();
        let e = rules.explain();
        assert!(e.contains("牌组构成"));
        assert!(e.contains("听牌判断"));
        assert!(e.contains("番数计算"));
    }
}
