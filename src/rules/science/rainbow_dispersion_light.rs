//! 彩虹与色散
//!
//! 太阳光经水珠折射反射产生彩虹的色散原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RainbowDispersionLightRules,
    name: "彩虹与色散",
    desc: "太阳光经水珠折射反射产生彩虹的色散原理",
    origin: "中国",
    tags: ["科学", "光学", "彩虹", "自然"]
}

impl RainbowDispersionLightRules {
    /// 成因原理
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "阳光为七色复合",
            "水珠折射分离颜色",
            "不同波长折射不同",
            "常见的分光现象",
        ]
    }

    /// 观察要点
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "雨后天晴常见",
            "背对太阳看彩虹",
            "清晨傍晚更清晰",
            "喷泉水滴亦可现",
        ]
    }

    /// 颜色顺序
    pub fn colors(&self) -> Vec<&'static str> {
        vec![
            "外红内紫排列",
            "红橙黄绿蓝靛紫",
            "每色波长相异",
            "双彩虹顺序相反",
        ]
    }

    /// 生活应用
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "三棱镜分解日光",
            "肥皂泡显色",
            "油膜五彩斑斓",
            "色散原理利用",
        ]
    }
}

impl Rule for RainbowDispersionLightRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("rainbow_dispersion")
    }

    fn explain(&self) -> String {
        format!(
            "【彩虹与色散】\n{}",
            [
                format!(
                    "成因原理：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观察要点：\\n{}",
                    self.observe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "颜色顺序：\\n{}",
                    self.colors()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活应用：\\n{}",
                    self.apply()
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
    fn test_rainbowdispersionlightrules_basic() {
        let rules = RainbowDispersionLightRules::new();
        assert_eq!(rules.metadata().name, "彩虹与色散");
        assert!(!rules.cause().is_empty());
        assert!(!rules.observe().is_empty());
        assert!(!rules.colors().is_empty());
        assert!(!rules.apply().is_empty());
    }

    #[test]
    fn test_rainbowdispersionlightrules_validation() {
        let rules = RainbowDispersionLightRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("rainbow_dispersion")
        );
    }

    #[test]
    fn test_rainbowdispersionlightrules_explain() {
        let rules = RainbowDispersionLightRules::new();
        let e = rules.explain();
        assert!(e.contains("成因原理"));
        assert!(e.contains("观察要点"));
        assert!(e.contains("颜色顺序"));
    }
}
