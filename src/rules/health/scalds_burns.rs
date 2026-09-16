//! 烫烧伤急救与护理
//!
//! 烫伤烧伤的应急处理、防护与后续护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScaldBurnCareRules,
    name: "烫烧伤急救与护理",
    desc: "烫伤烧伤的应急处理、防护与后续护理规则",
    origin: "医学",
    tags: ["健康", "烫伤", "烧伤", "急救", "护理"]
}

impl ScaldBurnCareRules {
    /// 应急处理
    pub fn first_aid(&self) -> Vec<&'static str> {
        vec![
            "立即用流动凉水冲洗烧伤处",
            "持续冲淋或冷敷约20分钟",
            "小心去除衣物并剪除未粘皮肤",
            "不用牙膏酱油等涂抹伤口",
        ]
    }

    /// 烫伤分级
    pub fn severity(&self) -> Vec<&'static str> {
        vec![
            "红斑轻度可用凉水降温观察",
            "起水泡避免自行挑破",
            "大面积或深部烫伤立即就医",
            "面部关节等重要部位涉医",
        ]
    }

    /// 护理要点
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "保持创面清洁避免感染",
            "不随意涂抹止痛或偏方药膏",
            "按时换药观察有无红肿渗液",
            "愈合期间避免抓挠",
        ]
    }

    /// 安全防护
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "厨房热物远离儿童可及处",
            "举热锅时注意避让",
            "给孩子洗澡先试水温",
            "热水壶放在台面稳固处",
        ]
    }
}

impl Rule for ScaldBurnCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("scalds")
    }

    fn explain(&self) -> String {
        format!(
            "【烫烧伤急救与护理】\n{}",
            [
                format!(
                    "应急处理：\\n{}",
                    self.first_aid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "烫伤分级：\\n{}",
                    self.severity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护理要点：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全防护：\\n{}",
                    self.prevention()
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
    fn test_scaldburncarerules_basic() {
        let rules = ScaldBurnCareRules::new();
        assert_eq!(rules.metadata().name, "烫烧伤急救与护理");
        assert!(!rules.first_aid().is_empty());
        assert!(!rules.severity().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.prevention().is_empty());
    }

    #[test]
    fn test_scaldburncarerules_validation() {
        let rules = ScaldBurnCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("scalds"));
    }

    #[test]
    fn test_scaldburncarerules_explain() {
        let rules = ScaldBurnCareRules::new();
        let e = rules.explain();
        assert!(e.contains("应急处理"));
        assert!(e.contains("烫伤分级"));
        assert!(e.contains("护理要点"));
    }
}
