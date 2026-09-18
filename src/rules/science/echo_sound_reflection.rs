//! 回声与声反射
//!
//! 声音遇到障碍反射形成回声的现象与成因

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EchoSoundReflectionRules,
    name: "回声与声反射",
    desc: "声音遇到障碍反射形成回声的现象与成因",
    origin: "中国",
    tags: ["科学", "回声", "声学", "自然"]
}

impl EchoSoundReflectionRules {
    /// 现象成因
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "声波遇墙反射",
            "反射声原声回来",
            "时间差大于感知",
            "听成两个声音",
        ]
    }

    /// 回声条件
    pub fn condition(&self) -> Vec<&'static str> {
        vec!["障碍较远", "距离差够远", "山谷楼道明显", "空旷地方回声清"]
    }

    /// 应用知识
    pub fn apply(&self) -> Vec<&'static str> {
        vec!["测量距离", "声呐探海底", "超声回声定位", "蝙蝠靠回声导航"]
    }

    /// 消除混响
    pub fn reduce(&self) -> Vec<&'static str> {
        vec![
            "室内吸音材料",
            "软布罩反射弱",
            "墙壁不平减少回",
            "影厅吸音设计",
        ]
    }
}

impl Rule for EchoSoundReflectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("echo_sound")
    }

    fn explain(&self) -> String {
        format!(
            "【回声与声反射】\n{}",
            [
                format!(
                    "现象成因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回声条件：\\n{}",
                    self.condition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用知识：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消除混响：\\n{}",
                    self.reduce()
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
    fn test_echosoundreflectionrules_basic() {
        let rules = EchoSoundReflectionRules::new();
        assert_eq!(rules.metadata().name, "回声与声反射");
        assert!(!rules.cause().is_empty());
        assert!(!rules.condition().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.reduce().is_empty());
    }

    #[test]
    fn test_echosoundreflectionrules_validation() {
        let rules = EchoSoundReflectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("echo_sound"));
    }

    #[test]
    fn test_echosoundreflectionrules_explain() {
        let rules = EchoSoundReflectionRules::new();
        let e = rules.explain();
        assert!(e.contains("现象成因"));
        assert!(e.contains("回声条件"));
        assert!(e.contains("应用知识"));
    }
}
