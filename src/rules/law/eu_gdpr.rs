//! GDPR规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EuGdprRules, name: "GDPR规则", desc: "欧盟通用数据保护规则", origin: "欧盟", tags: ["法律", "数据"] }
impl EuGdprRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "合法性公平性透明性:数据处理必须有合法基础",
            "目的限制:数据只能用于收集时声明的目的",
            "数据最小化:只收集必要的数据",
            "准确性:数据必须准确且及时更新",
            "存储限制:数据保留不超过必要时间",
            "完整性和保密性:确保数据安全",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "知情权:有权知道数据如何被处理",
            "访问权:有权获取自己的数据副本",
            "更正权:有权要求更正不准确的数据",
            "删除权:有权要求删除数据",
            "限制处理权:有权限制数据处理",
            "数据可携带权:有权以通用格式获取数据",
            "反对权:有权反对数据处理",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "严重违规:最高2000万欧元或全球营业额4%",
            "一般违规:最高1000万欧元或全球营业额2%",
            "数据泄露:必须在72小时内通知监管机构",
        ]
    }
}
impl Rule for EuGdprRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("eu_gdpr")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "GDPR规则",
            &[
                ("基本原则", &self.section_0()),
                ("数据主体权利", &self.section_1()),
                ("处罚", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EuGdprRules::new();
        assert!(!r.explain().is_empty());
    }
}
