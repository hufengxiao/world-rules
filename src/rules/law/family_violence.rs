//! 反家暴法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FamilyViolenceRules,
    name: "反家暴法",
    desc: "反家庭暴力法律规则",
    origin: "中国",
    tags: ["法律", "家庭"],
    category: RuleCategory::law("family_violence"),
    sections: [("措施", section_0), ("预防", section_1)]
}

impl FamilyViolenceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人身安全保护令", "告诫书"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["强制报告", "法治宣传"]
    }
}
