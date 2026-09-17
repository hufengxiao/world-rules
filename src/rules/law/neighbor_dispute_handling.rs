//! 邻里纠纷处理
//!
//! 住宅邻里纠纷的沟通、取证与依法处理原则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NeighborDisputeHandlingRules,
    name: "邻里纠纷处理",
    desc: "住宅邻里纠纷的沟通、取证与依法处理原则",
    origin: "中国",
    tags: ["法律", "邻里", "纠纷", "处理", "维权"]
}

impl NeighborDisputeHandlingRules {
    /// 优先协商
    pub fn negotiate(&self) -> Vec<&'static str> {
        vec![
            "先冷静理性沟通",
            "就事论事不上升人身",
            "约定合理解决方式",
            "保留沟通记录",
        ]
    }

    /// 取证留存
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留时间地点人证",
            "拍照录像记录侵害",
            "保存书面往来与回执",
            "记录物业或居委介入情况",
        ]
    }

    /// 寻求调解
    pub fn mediation(&self) -> Vec<&'static str> {
        vec![
            "请物业居委会参与调解",
            "向人民调解组织申请",
            "依法就噪音等扰民事宜反馈",
            "调解不成再依法处理",
        ]
    }

    /// 依法处理
    pub fn law(&self) -> Vec<&'static str> {
        vec![
            "情节严重可依法投诉",
            "涉及违法及时报警",
            "必要时通过诉讼解决",
            "不采取打架或报复方式",
        ]
    }
}

impl Rule for NeighborDisputeHandlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("neighbor_dispute")
    }

    fn explain(&self) -> String {
        format!(
            "【邻里纠纷处理】\n{}",
            [
                format!(
                    "优先协商：\\n{}",
                    self.negotiate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "取证留存：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻求调解：\\n{}",
                    self.mediation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依法处理：\\n{}",
                    self.law()
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
    fn test_neighbordisputehandlingrules_basic() {
        let rules = NeighborDisputeHandlingRules::new();
        assert_eq!(rules.metadata().name, "邻里纠纷处理");
        assert!(!rules.negotiate().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.mediation().is_empty());
        assert!(!rules.law().is_empty());
    }

    #[test]
    fn test_neighbordisputehandlingrules_validation() {
        let rules = NeighborDisputeHandlingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("neighbor_dispute"));
    }

    #[test]
    fn test_neighbordisputehandlingrules_explain() {
        let rules = NeighborDisputeHandlingRules::new();
        let e = rules.explain();
        assert!(e.contains("优先协商"));
        assert!(e.contains("取证留存"));
        assert!(e.contains("寻求调解"));
    }
}
