//! 美式麻将
//!
//! 以牌型卡为核心的美式麻将，配百搭牌与年度牌谱

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongAmericanRules,
    name: "美式麻将",
    desc: "以牌型卡为核心的美式麻将，配百搭牌与年度牌谱",
    origin: "美国",
    tags: ["麻将", "变体", "美国"]
}

impl MahjongAmericanRules {
    /// 牌张特色
    pub fn tiles(&self) -> Vec<&'static str> {
        vec![
            "含八张百搭牌",
            "牌面有数字与花色",
            "使用年度牌型卡",
            "含特殊风牌花牌",
        ]
    }

    /// 牌型卡
    pub fn card(&self) -> Vec<&'static str> {
        vec![
            "每年更新规定牌型",
            "须依卡上组合胡牌",
            "牌型分值各不相同",
            "胡牌须严格匹配",
        ]
    }

    /// 百搭使用
    pub fn joker(&self) -> Vec<&'static str> {
        vec![
            "百搭可代任意牌",
            "须配合牌型卡使用",
            "不可单独成对",
            "部分牌型禁用百搭",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "按牌型卡分数计",
            "自摸与点炮分不同",
            "累计局数定胜负",
            "四风与花牌可加分",
        ]
    }
}

impl Rule for MahjongAmericanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_american")
    }

    fn explain(&self) -> String {
        format!(
            "【美式麻将】\n{}",
            [
                format!(
                    "牌张特色：\\n{}",
                    self.tiles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牌型卡：\\n{}",
                    self.card()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "百搭使用：\\n{}",
                    self.joker()
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
    fn test_mahjongamericanrules_basic() {
        let rules = MahjongAmericanRules::new();
        assert_eq!(rules.metadata().name, "美式麻将");
        assert!(!rules.tiles().is_empty());
        assert!(!rules.card().is_empty());
        assert!(!rules.joker().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_mahjongamericanrules_validation() {
        let rules = MahjongAmericanRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("mahjong_american"));
    }

    #[test]
    fn test_mahjongamericanrules_explain() {
        let rules = MahjongAmericanRules::new();
        let e = rules.explain();
        assert!(e.contains("牌张特色"));
        assert!(e.contains("牌型卡"));
        assert!(e.contains("百搭使用"));
    }
}
