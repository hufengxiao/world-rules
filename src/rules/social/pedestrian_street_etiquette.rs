//! 步行街道礼仪
//!
//! 人行道步行时的让行、并行与行为礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PedestrianStreetEtiquetteRules,
    name: "步行街道礼仪",
    desc: "人行道步行时的让行、并行与行为礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "行人", "步行", "街道"]
}

impl PedestrianStreetEtiquetteRules {
    /// 行走规则
    pub fn walking(&self) -> Vec<&'static str> {
        vec![
            "靠右行走不逆向逆行",
            "红绿灯处依信号通行",
            "人行横道过马路",
            "不闯红灯横穿",
        ]
    }

    /// 让行大局
    pub fn yielding(&self) -> Vec<&'static str> {
        vec![
            "遇迎面让出空间",
            "老人儿童放慢等待",
            "转弯路口留意方向",
            "不带耳机完全隔绝路面",
        ]
    }

    /// 公共空间
    pub fn public(&self) -> Vec<&'static str> {
        vec![
            "不占用盲道停车",
            "不随地吐痰丢屑",
            "雨伞收放在窄处",
            "音响避免干扰他人",
        ]
    }

    /// 安全细节
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "过马路不看手机",
            "夜间浅色衣更安全",
            "孩童拉好过街",
            "紧急时为警车让行",
        ]
    }
}

impl Rule for PedestrianStreetEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("pedestrian")
    }

    fn explain(&self) -> String {
        format!(
            "【步行街道礼仪】\n{}",
            [
                format!(
                    "行走规则：\\n{}",
                    self.walking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "让行大局：\\n{}",
                    self.yielding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共空间：\\n{}",
                    self.public()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全细节：\\n{}",
                    self.safety()
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
    fn test_pedestrianstreetetiquetterules_basic() {
        let rules = PedestrianStreetEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "步行街道礼仪");
        assert!(!rules.walking().is_empty());
        assert!(!rules.yielding().is_empty());
        assert!(!rules.public().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_pedestrianstreetetiquetterules_validation() {
        let rules = PedestrianStreetEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("pedestrian"));
    }

    #[test]
    fn test_pedestrianstreetetiquetterules_explain() {
        let rules = PedestrianStreetEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("行走规则"));
        assert!(e.contains("让行大局"));
        assert!(e.contains("公共空间"));
    }
}
