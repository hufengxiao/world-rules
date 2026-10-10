//! AED使用指南
//!
//! 自动体外除颤器的使用步骤与注意事项

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AedUseGuideRules,
    name: "AED使用指南",
    desc: "自动体外除颤器的使用步骤与注意事项",
    origin: "中国",
    tags: ["急救", "心搏骤停", "应急"]
}

impl AedUseGuideRules {
    /// 使用时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "患者无意识无呼吸",
            "心搏骤停时使用",
            "与心肺复苏配合",
            "越早使用成功率越高",
        ]
    }

    /// 操作步骤
    pub fn steps(&self) -> Vec<&'static str> {
        vec![
            "开机听语音提示",
            "贴电极片于裸露胸部",
            "分析心律时勿触碰患者",
            "提示放电时按下按钮",
        ]
    }

    /// 电极位置
    pub fn pads(&self) -> Vec<&'static str> {
        vec![
            "右锁骨下与左腋中线",
            "胸部须干燥无遮挡",
            "有胸毛需剃除",
            "有起搏器避开贴片",
        ]
    }

    /// 配合要点
    pub fn coordination(&self) -> Vec<&'static str> {
        vec![
            "放电后立即继续按压",
            "按提示每两分钟分析",
            "持续至救护人员到达",
            "全程不要中断按压",
        ]
    }
}

impl Rule for AedUseGuideRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("aed_use_guide")
    }

    fn explain(&self) -> String {
        format!(
            "【AED使用指南】\n{}",
            [
                format!(
                    "使用时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "操作步骤：\\n{}",
                    self.steps()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "电极位置：\\n{}",
                    self.pads()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配合要点：\\n{}",
                    self.coordination()
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
    fn test_aeduseguiderules_basic() {
        let rules = AedUseGuideRules::new();
        assert_eq!(rules.metadata().name, "AED使用指南");
        assert!(!rules.timing().is_empty());
        assert!(!rules.steps().is_empty());
        assert!(!rules.pads().is_empty());
        assert!(!rules.coordination().is_empty());
    }

    #[test]
    fn test_aeduseguiderules_validation() {
        let rules = AedUseGuideRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("aed_use_guide"));
    }

    #[test]
    fn test_aeduseguiderules_explain() {
        let rules = AedUseGuideRules::new();
        let e = rules.explain();
        assert!(e.contains("使用时机"));
        assert!(e.contains("操作步骤"));
        assert!(e.contains("电极位置"));
    }
}
