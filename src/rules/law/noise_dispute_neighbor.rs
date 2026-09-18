//! 邻里噪音纠纷
//!
//! 邻里噪音扰民的协商、证据与处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NoiseDisputeNeighborRules,
    name: "邻里噪音纠纷",
    desc: "邻里噪音扰民的协商、证据与处理",
    origin: "中国",
    tags: ["法律", "噪音", "邻里", "纠纷"]
}

impl NoiseDisputeNeighborRules {
    /// 先沟通
    pub fn talk(&self) -> Vec<&'static str> {
        vec!["先礼貌沟通", "说明噪音困扰", "约定时段", "态度和缓"]
    }

    /// 留存证据
    pub fn evidence(&self) -> Vec<&'static str> {
        vec!["记录噪音时间", "录音录像", "邻居证词", "留物证"]
    }

    /// 求助调解
    pub fn mediation(&self) -> Vec<&'static str> {
        vec!["找物业社区调解", "反映居委", "治安调解", "依程序处理"]
    }

    /// 依法维权
    pub fn legal(&self) -> Vec<&'static str> {
        vec!["严重扰民可投诉", "环保噪音电话", "长期纠纷诉讼", "保留权利"]
    }
}

impl Rule for NoiseDisputeNeighborRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("noise_dispute")
    }

    fn explain(&self) -> String {
        format!(
            "【邻里噪音纠纷】\n{}",
            [
                format!(
                    "先沟通：\\n{}",
                    self.talk()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "留存证据：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求助调解：\\n{}",
                    self.mediation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依法维权：\\n{}",
                    self.legal()
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
    fn test_noisedisputeneighborrules_basic() {
        let rules = NoiseDisputeNeighborRules::new();
        assert_eq!(rules.metadata().name, "邻里噪音纠纷");
        assert!(!rules.talk().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.mediation().is_empty());
        assert!(!rules.legal().is_empty());
    }

    #[test]
    fn test_noisedisputeneighborrules_validation() {
        let rules = NoiseDisputeNeighborRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("noise_dispute"));
    }

    #[test]
    fn test_noisedisputeneighborrules_explain() {
        let rules = NoiseDisputeNeighborRules::new();
        let e = rules.explain();
        assert!(e.contains("先沟通"));
        assert!(e.contains("留存证据"));
        assert!(e.contains("求助调解"));
    }
}
