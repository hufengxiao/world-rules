//! 证据保留要点
//!
//! 纠纷维权中保存证据、依法举证的要领

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EvidencePreservationRules,
    name: "证据保留要点",
    desc: "纠纷维权中保存证据、依法举证的要领",
    origin: "中国",
    tags: ["法律", "证据", "举证"]
}

impl EvidencePreservationRules {
    /// 常见证据
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "合同协议书面文件",
            "转账记录发票",
            "聊天与通话记录",
            "现场照片视频",
        ]
    }

    /// 及时保存
    pub fn save(&self) -> Vec<&'static str> {
        vec![
            "纠纷发生及时固定",
            "原件妥存备份",
            "记录时间与情况",
            "不删改证据",
        ]
    }

    /// 有效举证
    pub fn valid(&self) -> Vec<&'static str> {
        vec![
            "证据真实合法关联",
            "保留原始载体",
            "确认真实来源",
            "必要时公证保全",
        ]
    }

    /// 维权应用
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "依证据主张权利",
            "不伪造变造证据",
            "举证责任依规",
            "疑难可咨询专业",
        ]
    }
}

impl Rule for EvidencePreservationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("evidence")
    }

    fn explain(&self) -> String {
        format!(
            "【证据保留要点】\n{}",
            [
                format!(
                    "常见证据：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "及时保存：\\n{}",
                    self.save()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "有效举证：\\n{}",
                    self.valid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权应用：\\n{}",
                    self.usage()
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
    fn test_evidencepreservationrules_basic() {
        let rules = EvidencePreservationRules::new();
        assert_eq!(rules.metadata().name, "证据保留要点");
        assert!(!rules.types().is_empty());
        assert!(!rules.save().is_empty());
        assert!(!rules.valid().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_evidencepreservationrules_validation() {
        let rules = EvidencePreservationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("evidence"));
    }

    #[test]
    fn test_evidencepreservationrules_explain() {
        let rules = EvidencePreservationRules::new();
        let e = rules.explain();
        assert!(e.contains("常见证据"));
        assert!(e.contains("及时保存"));
        assert!(e.contains("有效举证"));
    }
}
