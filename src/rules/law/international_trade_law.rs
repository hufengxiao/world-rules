//! 国际贸易法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InternationalTradeLawRules,
    name: "国际贸易法",
    desc: "国际贸易法律规则",
    origin: "国际",
    tags: ["法律", "国际"],
    category: RuleCategory::law("international_trade_law"),
    sections: [("WTO", section_0), ("贸易救济", section_1)]
}

impl InternationalTradeLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最惠国待遇", "争端解决"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反倾销", "反补贴"]
    }
}
