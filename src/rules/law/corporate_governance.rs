//! 公司治理法规
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CorporateGovernanceRules,
    name: "公司治理法规",
    desc: "公司治理法律规则",
    origin: "中国",
    tags: ["法律", "公司"],
    category: RuleCategory::law("corporate_governance"),
    sections: [("股东会", section_0), ("董事会", section_1)]
}

impl CorporateGovernanceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["决议程序", "股东权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["董事义务", "独立董事"]
    }
}
