//! 尤克牌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: EuchreRules,
    name: "尤克牌规则",
    desc: "尤克牌(Euchre)卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌"]
}

impl EuchreRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "4人分2队，对家为队友",
            "使用24张牌(9-A各4花色)",
            "每人发5张牌，3-2-3-2或2-3-2-3顺序",
            "翻一张牌决定王牌花色",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "王牌花色中J为右王牌(Bower)，同色J为左王牌",
            "王牌大小: 右J > 左J > A > K > Q > 10 > 9",
            "非王牌大小: A > K > Q > J > 10 > 9",
            "叫牌阶段可选择接受或放弃",
            "无人叫牌可由发牌人指定花色或重新发牌",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "出牌规则: 必须跟首攻花色，无则可出王牌或垫牌",
            "王牌能赢非王牌，同花色比大小",
            "一队叫牌并赢3-4墩得1分",
            "一队叫牌并赢全部5墩得2分",
            "防守方赢3墩以上(Euchre)得2分",
            "先到10分的队伍获胜",
        ]
    }
}

impl Rule for EuchreRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("euchre")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "尤克牌规则",
            &[
                ("基本设置", &self.section_0()),
                ("王牌与叫牌", &self.section_1()),
                ("出牌与计分", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EuchreRules::new();
        assert!(!r.explain().is_empty());
        assert!(r.explain().contains("右王牌"));
    }
}
