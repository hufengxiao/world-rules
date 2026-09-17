//! 攀岩礼仪与安全
//!
//! 攀岩馆攀爬、保护与器材使用的礼仪与安全

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ClimbingEtiquetteRules,
    name: "攀岩礼仪与安全",
    desc: "攀岩馆攀爬、保护与器材使用的礼仪与安全",
    origin: "国际",
    tags: ["体育", "攀岩", "户外", "礼仪", "安全"]
}

impl ClimbingEtiquetteRules {
    /// 入馆须知
    pub fn entry(&self) -> Vec<&'static str> {
        vec![
            "穿合脚的攀岩鞋入馆",
            "遵守场馆与线路规定",
            "新手先接受指导",
            "个人物品妥善放好",
        ]
    }

    /// 保护与搭档
    pub fn belay(&self) -> Vec<&'static str> {
        vec![
            "攀爬前互相检查保护器材",
            "听清保护口令再行动",
            "保护者专注不分散",
            "沟通确认再向下撤",
        ]
    }

    /// 排队轮流
    pub fn queue(&self) -> Vec<&'static str> {
        vec![
            "同一线排队轮流攀",
            "攀完让给下一位",
            "不插队或抢先",
            "休息不在线路下停留",
        ]
    }

    /// 安全互敬
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "遵守岩馆安全守则",
            "不冒险逃规动作",
            "感到不适及时停下",
            "尊重其他攀爬者",
        ]
    }
}

impl Rule for ClimbingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("climbing")
    }

    fn explain(&self) -> String {
        format!(
            "【攀岩礼仪与安全】\n{}",
            [
                format!(
                    "入馆须知：\\n{}",
                    self.entry()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护与搭档：\\n{}",
                    self.belay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "排队轮流：\\n{}",
                    self.queue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全互敬：\\n{}",
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
    fn test_climbingetiquetterules_basic() {
        let rules = ClimbingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "攀岩礼仪与安全");
        assert!(!rules.entry().is_empty());
        assert!(!rules.belay().is_empty());
        assert!(!rules.queue().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_climbingetiquetterules_validation() {
        let rules = ClimbingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("climbing"));
    }

    #[test]
    fn test_climbingetiquetterules_explain() {
        let rules = ClimbingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入馆须知"));
        assert!(e.contains("保护与搭档"));
        assert!(e.contains("排队轮流"));
    }
}
