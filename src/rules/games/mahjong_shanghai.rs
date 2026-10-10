//! 上海麻将
//!
//! 上海地区麻将玩法，含花牌与百搭的计番规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongShanghaiRules,
    name: "上海麻将",
    desc: "上海地区麻将玩法，含花牌与百搭的计番规则",
    origin: "中国",
    tags: ["麻将", "上海", "变体"]
}

impl MahjongShanghaiRules {
    /// 牌张配置
    pub fn tiles(&self) -> Vec<&'static str> {
        vec![
            "一百四十四张牌",
            "含春夏秋冬梅兰竹菊花牌",
            "使用全部万筒索风箭",
            "花牌摸到即补牌",
        ]
    }

    /// 胡牌牌型
    pub fn patterns(&self) -> Vec<&'static str> {
        vec![
            "基本四组一对",
            "碰碰胡清一色",
            "混一色与清混碰",
            "须达起胡番数",
        ]
    }

    /// 花牌计分
    pub fn flower(&self) -> Vec<&'static str> {
        vec![
            "摸花牌计一番",
            "正花加倍计分",
            "花牌须明示亮出",
            "花牌数影响番型",
        ]
    }

    /// 结算方式
    pub fn settlement(&self) -> Vec<&'static str> {
        vec!["按番数计分", "自摸三家付", "点炮一家付", "荒庄不输不赢"]
    }
}

impl Rule for MahjongShanghaiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_shanghai")
    }

    fn explain(&self) -> String {
        format!(
            "【上海麻将】\n{}",
            [
                format!(
                    "牌张配置：\\n{}",
                    self.tiles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胡牌牌型：\\n{}",
                    self.patterns()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "花牌计分：\\n{}",
                    self.flower()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结算方式：\\n{}",
                    self.settlement()
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
    fn test_mahjongshanghairules_basic() {
        let rules = MahjongShanghaiRules::new();
        assert_eq!(rules.metadata().name, "上海麻将");
        assert!(!rules.tiles().is_empty());
        assert!(!rules.patterns().is_empty());
        assert!(!rules.flower().is_empty());
        assert!(!rules.settlement().is_empty());
    }

    #[test]
    fn test_mahjongshanghairules_validation() {
        let rules = MahjongShanghaiRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("mahjong_shanghai"));
    }

    #[test]
    fn test_mahjongshanghairules_explain() {
        let rules = MahjongShanghaiRules::new();
        let e = rules.explain();
        assert!(e.contains("牌张配置"));
        assert!(e.contains("胡牌牌型"));
        assert!(e.contains("花牌计分"));
    }
}
