//! 健身卡合同规则
//!
//! 健身房会员卡消费的法律规则，包括合同签订、预付卡风险、退卡转让与商家违约救济，保护消费者合法权益。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GymMembershipContractRules,
    name: "健身卡合同规则",
    desc: "健身房会员卡消费的法律规则，包括合同签订、预付卡风险、退卡转让与商家违约救济，保护消费者合法权益。",
    origin: "预付式消费与消费者权益保护法规",
    tags: ["法律", "消费", "预付卡", "健身"]
}

impl GymMembershipContractRules {
    /// 合同签订
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "签订书面服务合同",
            "明确服务内容与期限",
            "留意格式条款中的免责内容",
            "保留合同与付款凭证",
        ]
    }

    /// 预付风险
    pub fn prepaid(&self) -> Vec<&'static str> {
        vec![
            "警惕超低价长期卡诱导",
            "预付金额不宜过大",
            "了解退费规则",
            "关注商家经营状况",
        ]
    }

    /// 退卡转让
    pub fn refund(&self) -> Vec<&'static str> {
        vec![
            "合理原因可申请退卡",
            "退费按剩余期限折算",
            "转让须符合约定程序",
            "商家不得设不合理障碍",
        ]
    }

    /// 违约救济
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "商家关店跑路可投诉",
            "可向消协或市场监管举报",
            "协商不成可提起诉讼",
            "涉嫌诈骗可报警处理",
        ]
    }
}

impl Rule for GymMembershipContractRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("gym_membership")
    }

    fn explain(&self) -> String {
        format!(
            "【健身卡合同规则】\n{}",
            [
                format!(
                    "合同签订：\\n{}",
                    self.contract()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预付风险：\\n{}",
                    self.prepaid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退卡转让：\\n{}",
                    self.refund()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "违约救济：\\n{}",
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
    fn test_gymmembershipcontractrules_basic() {
        let rules = GymMembershipContractRules::new();
        assert_eq!(rules.metadata().name, "健身卡合同规则");
        assert!(!rules.contract().is_empty());
        assert!(!rules.prepaid().is_empty());
        assert!(!rules.refund().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_gymmembershipcontractrules_validation() {
        let rules = GymMembershipContractRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("gym_membership"));
    }

    #[test]
    fn test_gymmembershipcontractrules_explain() {
        let rules = GymMembershipContractRules::new();
        let e = rules.explain();
        assert!(e.contains("合同签订"));
        assert!(e.contains("预付风险"));
        assert!(e.contains("退卡转让"));
    }
}
