//! 基因编辑法规
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GeneEditingLawRules,
    name: "基因编辑法规",
    desc: "基因编辑技术法律规则",
    origin: "国际",
    tags: ["法律", "生物"],
    category: RuleCategory::law("gene_editing_law"),
    sections: [("研究限制", section_0), ("应用监管", section_1)]
}

impl GeneEditingLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["人类胚胎编辑禁止", "知情同意", "伦理审查"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因治疗审批", "转基因监管", "基因检测规范"]
    }
}
