//! 痔疮护理
//!
//! 痔疮发作期的缓解与日常预防护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HemorrhoidCareRules,
    name: "痔疮护理",
    desc: "痔疮发作期的缓解与日常预防护理",
    origin: "中国",
    tags: ["健康", "痔疮", "护理", "肠胃"]
}

impl HemorrhoidCareRules {
    /// 识别表现
    pub fn recognize(&self) -> Vec<&'static str> {
        vec!["排便出血", "肛门肿痛瘙痒", "异物脱出感", "注意严重出血"]
    }

    /// 发作缓解
    pub fn relieve(&self) -> Vec<&'static str> {
        vec!["温水坐浴", "勿久蹲用力", "冷敷缓肿痛", "排便勿过久"]
    }

    /// 饮食调理
    pub fn diet(&self) -> Vec<&'static str> {
        vec!["多喝水", "多食蔬果纤维", "避免辛辣刺激", "少食油腻"]
    }

    /// 日常预防
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["规律排便放松", "避免久坐久站", "适度活动", "保持肛周清洁"]
    }
}

impl Rule for HemorrhoidCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hemorrhoid")
    }

    fn explain(&self) -> String {
        format!(
            "【痔疮护理】\n{}",
            [
                format!(
                    "识别表现：\\n{}",
                    self.recognize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发作缓解：\\n{}",
                    self.relieve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调理：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常预防：\\n{}",
                    self.prevent()
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
    fn test_hemorrhoidcarerules_basic() {
        let rules = HemorrhoidCareRules::new();
        assert_eq!(rules.metadata().name, "痔疮护理");
        assert!(!rules.recognize().is_empty());
        assert!(!rules.relieve().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.prevent().is_empty());
    }

    #[test]
    fn test_hemorrhoidcarerules_validation() {
        let rules = HemorrhoidCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hemorrhoid"));
    }

    #[test]
    fn test_hemorrhoidcarerules_explain() {
        let rules = HemorrhoidCareRules::new();
        let e = rules.explain();
        assert!(e.contains("识别表现"));
        assert!(e.contains("发作缓解"));
        assert!(e.contains("饮食调理"));
    }
}
