//! 邻里噪音纠纷
//!
//! 处理邻里噪音干扰、沟通与依法维权的要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NeighborhoodNoiseDisputeRules,
    name: "邻里噪音纠纷",
    desc: "处理邻里噪音干扰、沟通与依法维权的要点",
    origin: "中国",
    tags: ["法律", "邻里", "噪音", "纠纷"]
}

impl NeighborhoodNoiseDisputeRules {
    /// 理性沟通
    pub fn communicate(&self) -> Vec<&'static str> {
        vec![
            "先心平气和协商",
            "说明受扰与希望",
            "尊重对方合理作息",
            "互谅避免激化",
        ]
    }

    /// 记录证据
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "记录噪音时间与影响",
            "保留相关沟通记录",
            "必要时记录证据",
            "不虚构夸大情况",
        ]
    }

    /// 依规处理
    pub fn channel(&self) -> Vec<&'static str> {
        vec![
            "物业或社区调解先行",
            "了解噪音管理规定",
            "必要时向部门反映",
            "在合理范围内主张休息权",
        ]
    }

    /// 依法维权
    pub fn legal(&self) -> Vec<&'static str> {
        vec![
            "影响严重可依法主张",
            "避免私报复对骂",
            "依据证据走正当程序",
            "尊重处理结果",
        ]
    }
}

impl Rule for NeighborhoodNoiseDisputeRules {
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
                    "理性沟通：\\n{}",
                    self.communicate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "记录证据：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依规处理：\\n{}",
                    self.channel()
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
    fn test_neighborhoodnoisedisputerules_basic() {
        let rules = NeighborhoodNoiseDisputeRules::new();
        assert_eq!(rules.metadata().name, "邻里噪音纠纷");
        assert!(!rules.communicate().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.channel().is_empty());
        assert!(!rules.legal().is_empty());
    }

    #[test]
    fn test_neighborhoodnoisedisputerules_validation() {
        let rules = NeighborhoodNoiseDisputeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("noise_dispute"));
    }

    #[test]
    fn test_neighborhoodnoisedisputerules_explain() {
        let rules = NeighborhoodNoiseDisputeRules::new();
        let e = rules.explain();
        assert!(e.contains("理性沟通"));
        assert!(e.contains("记录证据"));
        assert!(e.contains("依规处理"));
    }
}
