//! 卡西诺规则 (Cassino)
//!
//! 传统卡牌配对游戏，通过捕获牌堆中的牌来得分。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CassinoRules,
    name: "卡西诺规则",
    desc: "卡西诺(Cassino)传统卡牌配对游戏规则",
    origin: "意大利",
    tags: ["游戏", "卡牌", "配对", "传统"],
}

impl CassinoRules {
    /// 游戏设置
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "2-4人游戏（最佳2人）",
            "使用标准52张牌（不含大小王）",
            "每人发4张牌",
            "桌中央发4张牌面朝上",
            "剩余牌作抽牌堆",
            "发牌轮换，每轮每人再发4张",
        ]
    }

    /// 捕获规则
    pub fn section_capture(&self) -> Vec<&'static str> {
        vec![
            "配对捕获：手牌与桌上同点数牌配对捕获",
            "组合捕获：手牌等于桌上多张牌点数之和",
            "例：手牌9可捕获桌上4+5",
            "构筑(Build)：放牌到桌上，宣布组合点数",
            "例：放5到桌上4旁，宣布\"构筑9\"",
            "只有构筑者能用对应手牌捕获构筑牌",
            "对手可添加牌改变构筑目标",
        ]
    }

    /// 特殊牌规则
    pub fn section_special_cards(&self) -> Vec<&'static str> {
        vec![
            "A有特殊价值：可捕获桌上任意单张牌",
            "A也可作为1点参与组合",
            "J称为\"Big Cassino\"：捕获桌上所有牌",
            "J也可作为11点参与组合",
            "2称为\"Little Cassino\"：可捕获桌上所有A",
            "牌面朝下的牌不能被捕获",
        ]
    }

    /// 计分规则
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "捕获最多牌：3分",
            "捕获最多红心牌：1分",
            "捕获Big Cassino(J)：2分",
            "捕获Little Cassino(2♠)：1分",
            "捕获每个A：1分",
            "游戏结束：用完所有牌，最高分者获胜",
            "总分11分（2人）或21分（多人）",
        ]
    }

    /// 策略建议
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "优先捕获J(Big Cassino)和2♠(Little Cassino)",
            "收集红心牌争夺红心分",
            "谨慎构筑，避免对手拦截",
            "记牌，预判对手手牌",
            "利用A的灵活性",
            "合理分配捕获时机",
        ]
    }
}

impl Rule for CassinoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("cassino")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡西诺规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("捕获规则", &self.section_capture()),
                ("特殊牌", &self.section_special_cards()),
                ("计分规则", &self.section_scoring()),
                ("策略建议", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cassino_rules() {
        let rules = CassinoRules::new();
        assert_eq!(rules.metadata().name, "卡西诺规则");
        assert!(rules.explain().contains("Big Cassino"));
        assert!(rules.explain().contains("构筑"));
        assert!(rules.explain().contains("配对"));
    }

    #[test]
    fn test_cassino_category() {
        let rules = CassinoRules::new();
        assert_eq!(rules.category(), RuleCategory::games("cassino"));
    }
}