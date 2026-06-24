//! 民法典总则详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilCodeGeneralRules, name: "民法典总则详解", desc: "民法典总则详解", origin: "中国", tags: ["法律", "民法"] }
impl CivilCodeGeneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "平等原则:民事主体法律地位一律平等",
            "自愿原则:按照自己意愿设立变更终止民事法律关系",
            "公平原则:合理确定各方权利义务",
            "诚信原则:秉持诚实恪守承诺",
            "守法与公序良俗原则",
            "绿色原则:节约资源保护生态环境",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "自然人:从出生到死亡享有民事权利能力",
            "法人:营利法人/非营利法人/特别法人",
            "非法人组织:个人独资企业/合伙企业等",
            "民事行为能力:完全/限制/无民事行为能力",
            "8周岁以上18周岁以下为限制民事行为能力人",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "有效条件:行为人有相应行为能力/意思表示真实/不违反法律",
            "无效情形:违反强制性规定/违背公序良俗/恶意串通",
            "可撤销:重大误解/欺诈/胁迫/显失公平",
            "效力待定:限制行为能力人超出能力范围的行为",
        ]
    }
}
impl Rule for CivilCodeGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_general")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典总则详解",
            &[
                ("基本原则", &self.section_0()),
                ("民事主体", &self.section_1()),
                ("民事法律行为", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilCodeGeneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
