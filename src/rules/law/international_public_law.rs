//! 国际公法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InternationalPublicLawRules,
    name: "国际公法",
    desc: "国际公法基本规则",
    origin: "国际",
    tags: ["法律", "国际"],
    category: RuleCategory::law("international_public_law"),
    sections: [("基本原则", section_0), ("主体", section_1)]
}

impl InternationalPublicLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主权平等", "不干涉内政", "和平解决争端"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国家", "国际组织", "个人"]
    }
}
