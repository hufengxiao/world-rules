//! 三包保修索赔
//!
//! 家用电器等商品的七日、十五日与三包修理退还

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WarrantyClaimsRules,
    name: "三包保修索赔",
    desc: "家用电器等商品的七日、十五日与三包修理退还",
    origin: "中国",
    tags: ["法律", "三包", "保修", "商品"]
}

impl WarrantyClaimsRules {
    /// 三包保修
    pub fn warranty(&self) -> Vec<&'static str> {
        vec![
            "了解商品三包范围",
            "确认保修卡与凭证",
            "查明保修有效期限",
            "保留购买与维修记录",
        ]
    }

    /// 报修处理
    pub fn service(&self) -> Vec<&'static str> {
        vec![
            "出现质量故障及时申报",
            "按厂家指定渠道报修",
            "如实描述故障",
            "妥善保管维修回执",
        ]
    }

    /// 换货退货
    pub fn replace(&self) -> Vec<&'static str> {
        vec![
            "符合条件可依规换新",
            "严重质量可主张退货",
            "依规定承担时限责任",
            "不与商家虚耗推扯",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "经销商商家先行沟通",
            "符条件可向监管投诉",
            "依消法主张权利",
            "证据齐全利于维权",
        ]
    }
}

impl Rule for WarrantyClaimsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("warranty")
    }

    fn explain(&self) -> String {
        format!(
            "【三包保修索赔】\n{}",
            [
                format!(
                    "三包保修：\\n{}",
                    self.warranty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "报修处理：\\n{}",
                    self.service()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "换货退货：\\n{}",
                    self.replace()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
                    self.remedy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_warrantyclaimsrules_basic() {
        let rules = WarrantyClaimsRules::new();
        assert_eq!(rules.metadata().name, "三包保修索赔");
        assert!(!rules.warranty().is_empty());
        assert!(!rules.service().is_empty());
        assert!(!rules.replace().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_warrantyclaimsrules_validation() {
        let rules = WarrantyClaimsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("warranty"));
    }

    #[test]
    fn test_warrantyclaimsrules_explain() {
        let rules = WarrantyClaimsRules::new();
        let e = rules.explain();
        assert!(e.contains("三包保修"));
        assert!(e.contains("报修处理"));
        assert!(e.contains("换货退货"));
    }
}
