//! 儿童权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChildrenRightsRules, name: "儿童权利法", desc: "儿童权利保障法", origin: "国际", tags: ["法律", "儿童"] }
impl ChildrenRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "生存权:有权获得基本生活保障",
            "发展权:有权获得教育和发展机会",
            "受保护权:有权免受暴力和剥削",
            "参与权:有权表达意见和参与决策",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "家庭保护:父母有抚养教育义务",
            "学校保护:学校有安全教育义务",
            "社会保护:禁止使用童工",
            "网络保护:限制未成年人网络游戏时间",
            "司法保护:少年法庭/教育为主惩罚为辅",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "联合国儿童权利公约:最全面的儿童权利国际文件",
            "中国1992年加入该公约",
            "核心原则:儿童最佳利益原则",
            "禁止歧视:不因种族性别等受歧视",
        ]
    }
}
impl Rule for ChildrenRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("children_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儿童权利法",
            &[
                ("基本权利", &self.section_0()),
                ("保护措施", &self.section_1()),
                ("国际公约", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChildrenRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
