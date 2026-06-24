//! WTO法律规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WtoLawRules, name: "WTO法律规则", desc: "WTO国际贸易规则", origin: "国际", tags: ["法律", "国际"] }
impl WtoLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "最惠国待遇:给予一国的优惠必须无条件给予所有WTO成员",
            "国民待遇:进口商品与国内商品同等待遇",
            "透明度原则:贸易政策法规必须公开",
            "自由贸易原则:通过谈判降低关税和贸易壁垒",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "磋商:争端双方首先尝试协商解决",
            "专家组:磋商失败后设立专家组审理",
            "上诉机构:对专家组报告可以上诉",
            "执行:败诉方必须执行裁决否则面临报复",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "GATT:关税与贸易总协定(货物贸易)",
            "GATS:服务贸易总协定",
            "TRIPS:与贸易有关的知识产权协定",
            "SPS:卫生与植物卫生措施协定",
            "TBT:技术性贸易壁垒协定",
        ]
    }
}
impl Rule for WtoLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("wto_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WTO法律规则",
            &[
                ("基本原则", &self.section_0()),
                ("争端解决", &self.section_1()),
                ("主要协定", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WtoLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
