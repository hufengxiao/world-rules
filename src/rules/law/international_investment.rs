//! 国际投资法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InternationalInvestmentRules,
    name: "国际投资法",
    desc: "国际投资法律规则",
    origin: "国际",
    tags: ["法律", "国际"],
    category: RuleCategory::law("international_investment"),
    sections: [("保护", section_0), ("争端", section_1)]
}

impl InternationalInvestmentRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["公平公正待遇", "征收补偿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["ICSID仲裁"]
    }
}
