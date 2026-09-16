//! 接听电话的礼仪
//!
//! 接打工作与私人电话时的应答、措辞与时机礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PhoneAnsweringEtiquetteRules,
    name: "接听电话的礼仪",
    desc: "接打工作与私人电话时的应答、措辞与时机礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "电话", "沟通"]
}

impl PhoneAnsweringEtiquetteRules {
    /// 接听时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "在适宜场所接听电话",
            "公共场合尽量简短",
            "重要会议用餐时先示意可否接",
            "无人接听时及时回电",
        ]
    }

    /// 应答措辞
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "接起先自报身份或问候",
            "语速清晰表达友善",
            "公共场合压低音量",
            "使用敬语与收尾礼貌",
        ]
    }

    /// 转接与留言
    pub fn handoff(&self) -> Vec<&'static str> {
        vec![
            "转接时说明转接原因",
            "代接记录并准确转达",
            "对方不在时询问是否留言",
            "按时回拨约好的来电",
        ]
    }

    /// 结束通话
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "提及重点后示意道别",
            "确认重要信息电话复述",
            "礼貌告别再挂断",
            "事后及时跟进未尽事宜",
        ]
    }
}

impl Rule for PhoneAnsweringEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("phone_answering")
    }

    fn explain(&self) -> String {
        format!(
            "【接听电话的礼仪】\n{}",
            [
                format!(
                    "接听时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应答措辞：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "转接与留言：\\n{}",
                    self.handoff()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结束通话：\\n{}",
                    self.ending()
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
    fn test_phoneansweringetiquetterules_basic() {
        let rules = PhoneAnsweringEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "接听电话的礼仪");
        assert!(!rules.timing().is_empty());
        assert!(!rules.greeting().is_empty());
        assert!(!rules.handoff().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_phoneansweringetiquetterules_validation() {
        let rules = PhoneAnsweringEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("phone_answering"));
    }

    #[test]
    fn test_phoneansweringetiquetterules_explain() {
        let rules = PhoneAnsweringEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("接听时机"));
        assert!(e.contains("应答措辞"));
        assert!(e.contains("转接与留言"));
    }
}
