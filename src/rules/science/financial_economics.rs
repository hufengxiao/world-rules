//! 金融经济学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FinancialEconomicsRules,
    name: "金融经济学定律",
    desc: "金融经济学定律",
    origin: "国际",
    tags: ["科学", "经济"],
    category: RuleCategory::science("financial_economics"),
    sections: [("定价", section_0), ("风险", section_1)]
}

impl FinancialEconomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CAPM模型", "期权定价"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有效市场假说", "投资组合"]
    }
}
