//! MotoGP详细
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MotogpDetailedRules,
    name: "MotoGP详细",
    desc: "MotoGP详细规则",
    origin: "FIM",
    tags: ["体育", "赛车"],
    category: RuleCategory::sports("motogp_detailed"),
    sections: [("组别", section_0), ("规则", section_1)]
}

impl MotogpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["MotoGP", "Moto2"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["积分", "排位赛"]
    }
}
