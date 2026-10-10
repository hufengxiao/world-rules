//! 四川血战到底
//!
//! 四川麻将血战玩法，一家胡牌后其余继续直至三家胡完

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongSichuanXueliuRules,
    name: "四川血战到底",
    desc: "四川麻将血战玩法，一家胡牌后其余继续直至三家胡完",
    origin: "中国",
    tags: ["麻将", "四川", "血战"]
}

impl MahjongSichuanXueliuRules {
    /// 基本规则
    pub fn basics(&self) -> Vec<&'static str> {
        vec![
            "只用万筒索三门",
            "缺一门方可胡牌",
            "必须缺一门才能胡",
            "无字牌风牌",
        ]
    }

    /// 血战特色
    pub fn xueliu(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后牌局继续",
            "未胡者继续摸打",
            "直至三家胡牌或流局",
            "最后一家称大战",
        ]
    }

    /// 番型计分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "平胡基础分",
            "对对胡清一色加番",
            "七对金钩钓等特殊牌型",
            "番数可翻倍累加",
        ]
    }

    /// 结算规则
    pub fn settlement(&self) -> Vec<&'static str> {
        vec![
            "查叫定输赢",
            "未听牌者赔付听牌者",
            "杠牌另计番",
            "番数按牌型核算",
        ]
    }
}

impl Rule for MahjongSichuanXueliuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_sichuan_xueliu")
    }

    fn explain(&self) -> String {
        format!(
            "【四川血战到底】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.basics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "血战特色：\\n{}",
                    self.xueliu()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "番型计分：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结算规则：\\n{}",
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
    fn test_mahjongsichuanxueliurules_basic() {
        let rules = MahjongSichuanXueliuRules::new();
        assert_eq!(rules.metadata().name, "四川血战到底");
        assert!(!rules.basics().is_empty());
        assert!(!rules.xueliu().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.settlement().is_empty());
    }

    #[test]
    fn test_mahjongsichuanxueliurules_validation() {
        let rules = MahjongSichuanXueliuRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::games("mahjong_sichuan_xueliu")
        );
    }

    #[test]
    fn test_mahjongsichuanxueliurules_explain() {
        let rules = MahjongSichuanXueliuRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("血战特色"));
        assert!(e.contains("番型计分"));
    }
}
