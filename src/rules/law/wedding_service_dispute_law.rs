//! 婚庆服务纠纷
//!
//! 婚庆摄影婚宴等服务违约的维权要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeddingServiceDisputeLawRules,
    name: "婚庆服务纠纷",
    desc: "婚庆摄影婚宴等服务违约的维权要点",
    origin: "中国",
    tags: ["法律", "消费", "服务"]
}

impl WeddingServiceDisputeLawRules {
    /// 合同签订
    pub fn contract(&self) -> Vec<&'static str> {
        vec![
            "签订书面服务合同",
            "明确服务项目与标准",
            "约定价款与付款方式",
            "写明违约责任",
        ]
    }

    /// 常见纠纷
    pub fn disputes(&self) -> Vec<&'static str> {
        vec![
            "服务缩水与货不对板",
            "照片丢失或延迟交付",
            "临时加价",
            "婚宴菜品数量不符",
        ]
    }

    /// 举证要点
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留合同与付款凭证",
            "保存沟通记录",
            "拍摄现场实际情况",
            "留存服务瑕疵证据",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先与服务方协商",
            "可要求重作或退款",
            "向消协投诉",
            "违约严重可诉讼索赔",
        ]
    }
}

impl Rule for WeddingServiceDisputeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("wedding_service_dispute_law")
    }

    fn explain(&self) -> String {
        format!(
            "【婚庆服务纠纷】\n{}",
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
                    "常见纠纷：\\n{}",
                    self.disputes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "举证要点：\\n{}",
                    self.evidence()
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
    fn test_weddingservicedisputelawrules_basic() {
        let rules = WeddingServiceDisputeLawRules::new();
        assert_eq!(rules.metadata().name, "婚庆服务纠纷");
        assert!(!rules.contract().is_empty());
        assert!(!rules.disputes().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_weddingservicedisputelawrules_validation() {
        let rules = WeddingServiceDisputeLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("wedding_service_dispute_law")
        );
    }

    #[test]
    fn test_weddingservicedisputelawrules_explain() {
        let rules = WeddingServiceDisputeLawRules::new();
        let e = rules.explain();
        assert!(e.contains("合同签订"));
        assert!(e.contains("常见纠纷"));
        assert!(e.contains("举证要点"));
    }
}
