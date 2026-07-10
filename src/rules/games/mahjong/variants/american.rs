//! 美国麻将规则
//!
//! 美国麻将使用独特的西方计分表和 Joker 牌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 美国麻将规则
pub struct AmericanMahjongRules {
    metadata: RuleMetadata,
}

impl AmericanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("美国麻将规则", "美国麻将联合会标准规则")
                .with_origin("美国")
                .with_tags(vec!["游戏".into(), "麻将".into(), "美国".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用152张牌(含8张Joker牌)",
            "每人起手13张牌",
            "庄家起手14张",
            "东南西北四方",
            "使用西方标准计分卡",
        ]
    }

    /// Joker牌规则
    pub fn joker_rules(&self) -> Vec<&'static str> {
        vec![
            "8张Joker牌可替代任何牌",
            "Joker不能单独成对(将)",
            "Joker可在暴露组中使用",
            "Joker不可用于暗组",
            "Joker可被替换回收",
        ]
    }

    /// Charleston规则
    pub fn charleston_rules(&self) -> Vec<&'static str> {
        vec![
            "第一轮Charleston: 右-过-左各3张",
            "第二轮Charleston: 右-过-左各3张",
            "可选第三轮Charleston(对面3张)",
            "必须传递至少1张牌",
            "盲传不能看对方传递的牌",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<(&'static str, u16)> {
        vec![
            // 基本牌型
            ("基础牌型", 0),
            ("清一色", 25),
            ("混一色", 10),
            ("全风牌", 25),
            // 特殊牌型
            ("七对子", 25),
            ("龙七对", 50),
            ("天和", 50),
            ("地和", 50),
            // 高分牌型
            ("清一色七对", 50),
            ("混一色七对", 35),
            ("四风齐", 50),
            ("三风齐", 25),
            // 计分倍数
            ("自摸双倍", 2),
            ("Joker使用加5分", 5),
            ("无Joker加5分", 5),
            ("空手(无暴露组)加5分", 5),
        ]
    }

    /// 暴露规则
    pub fn exposure_rules(&self) -> Vec<&'static str> {
        vec![
            "可以暴露一组(明组)",
            "暴露后必须轮到你时才能继续",
            "暴露组可使用Joker",
            "暴露后可声明Mahjong(胡牌)",
            "暴露时必须显示完整组",
        ]
    }

    /// 禁止规则
    pub fn forbidden_rules(&self) -> Vec<&'static str> {
        vec![
            "不能吃牌",
            "必须完整牌型才能胡",
            "Joker不能作将",
            "必须使用标准计分卡",
            "流局无人胡牌重新开始",
        ]
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "东南西北顺序",
            "东家为庄开始",
            "Charleston传递阶段",
            "出牌、碰、杠阶段",
            "Mahjong(胡牌)结束",
        ]
    }

    /// 碰杠规则
    pub fn pung_kong_rules(&self) -> Vec<&'static str> {
        vec![
            "可以碰(Pung)任意家的牌",
            "碰后暴露三张相同牌",
            "可以杠(Kong)暴露四张",
            "杠后摸一张牌",
            "可以使用Joker补杠",
        ]
    }
}

impl Default for AmericanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AmericanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_american")
    }

    fn explain(&self) -> String {
        let scoring_list: String = self
            .scoring_rules()
            .iter()
            .map(|(name, score)| format!("  • {}: {}分", name, score))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【美国麻将规则】\n\n\
            基本设置:\n{}\n\n\
            Joker牌规则:\n{}\n\n\
            Charleston规则:\n{}\n\n\
            计分规则:\n{}\n\n\
            暴露规则:\n{}\n\n\
            禁止规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.joker_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.charleston_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            scoring_list,
            self.exposure_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.forbidden_rules()
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
    fn test_american_mahjong_rules() {
        let rules = AmericanMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.joker_rules().is_empty());
        assert!(rules.scoring_rules().len() > 0);
    }

    #[test]
    fn test_american_joker_rules() {
        let rules = AmericanMahjongRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Joker"));
        assert!(explanation.contains("152张"));
    }
}
