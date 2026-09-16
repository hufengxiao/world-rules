//! 房屋租赁合同要点
//!
//! 租房签约、押金、维修与退租中应知晓的法律要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RentalLeaseBasicsRules,
    name: "房屋租赁合同要点",
    desc: "租房签约、押金、维修与退租中应知晓的法律要点",
    origin: "中国",
    tags: ["法律", "租房", "租赁", "合同", "权益"]
}

impl RentalLeaseBasicsRules {
    /// 签约审阅
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "明确租金、押金与期限",
            "看清违约与续约条款",
            "核对房屋权属与承租人身份",
            "拒绝口头约定的模糊表述",
        ]
    }

    /// 交接查验
    pub fn move_in(&self) -> Vec<&'static str> {
        vec![
            "入住前房屋现状拍照留证",
            "核对家具家电清单与状况",
            "测试水电燃气等设施",
            "用电用气安全注意事项留存",
        ]
    }

    /// 租期维护
    pub fn maintenance(&self) -> Vec<&'static str> {
        vec![
            "明确维修责任归属",
            "重大维修及时告知房东",
            "正常损耗不应一味由租客承担",
            "保留往来书面记录",
        ]
    }

    /// 退租结清
    pub fn move_out(&self) -> Vec<&'static str> {
        vec![
            "按约提前通知退租",
            "归还房屋达到合理清洁",
            "核对押金退还与扣费依据",
            "结清水电燃气等费用",
        ]
    }
}

impl Rule for RentalLeaseBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("rental_lease")
    }

    fn explain(&self) -> String {
        format!(
            "【房屋租赁合同要点】\n{}",
            [
                format!(
                    "签约审阅：\\n{}",
                    self.contract()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交接查验：\\n{}",
                    self.move_in()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "租期维护：\\n{}",
                    self.maintenance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退租结清：\\n{}",
                    self.move_out()
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
    fn test_rentalleasebasicsrules_basic() {
        let rules = RentalLeaseBasicsRules::new();
        assert_eq!(rules.metadata().name, "房屋租赁合同要点");
        assert!(!rules.contract().is_empty());
        assert!(!rules.move_in().is_empty());
        assert!(!rules.maintenance().is_empty());
        assert!(!rules.move_out().is_empty());
    }

    #[test]
    fn test_rentalleasebasicsrules_validation() {
        let rules = RentalLeaseBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("rental_lease"));
    }

    #[test]
    fn test_rentalleasebasicsrules_explain() {
        let rules = RentalLeaseBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("签约审阅"));
        assert!(e.contains("交接查验"));
        assert!(e.contains("租期维护"));
    }
}
