//! 波动光学基础
//!
//! 光的干涉、衍射与偏振基础定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaveOpticsBasicsRules,
    name: "波动光学基础",
    desc: "光的干涉、衍射与偏振基础定律",
    origin: "国际",
    tags: ["科学", "光学", "波动", "物理"]
}

impl WaveOpticsBasicsRules {
    /// 光的干涉
    pub fn interference(&self) -> Vec<&'static str> {
        vec![
            "同源相干产生干涉",
            "双缝实验明暗相间",
            "光程差决定相位",
            "相干光源须同频相干",
        ]
    }

    /// 光的衍射
    pub fn diffraction(&self) -> Vec<&'static str> {
        vec![
            "光绕过障碍扩展传播",
            "缝宽与波长相当才明显",
            "夫琅禾费衍射条纹分布",
            "小孔成像含衍射效应",
        ]
    }

    /// 偏振与色散
    pub fn polarization(&self) -> Vec<&'static str> {
        vec![
            "光振动方向聚集为偏振",
            "偏振片使光偏振",
            "色散使白光分解色彩",
            "雨虹与棱镜皆色散",
        ]
    }

    /// 应用观察
    pub fn application(&self) -> Vec<&'static str> {
        vec![
            "衍射光栅用于光谱",
            "薄膜干涉产生彩色",
            "偏振眼镜减眩光",
            "理解光学仪器成像",
        ]
    }
}

impl Rule for WaveOpticsBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("wave_optics")
    }

    fn explain(&self) -> String {
        format!(
            "【波动光学基础】\n{}",
            [
                format!(
                    "光的干涉：\\n{}",
                    self.interference()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "光的衍射：\\n{}",
                    self.diffraction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "偏振与色散：\\n{}",
                    self.polarization()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用观察：\\n{}",
                    self.application()
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
    fn test_waveopticsbasicsrules_basic() {
        let rules = WaveOpticsBasicsRules::new();
        assert_eq!(rules.metadata().name, "波动光学基础");
        assert!(!rules.interference().is_empty());
        assert!(!rules.diffraction().is_empty());
        assert!(!rules.polarization().is_empty());
        assert!(!rules.application().is_empty());
    }

    #[test]
    fn test_waveopticsbasicsrules_validation() {
        let rules = WaveOpticsBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("wave_optics"));
    }

    #[test]
    fn test_waveopticsbasicsrules_explain() {
        let rules = WaveOpticsBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("光的干涉"));
        assert!(e.contains("光的衍射"));
        assert!(e.contains("偏振与色散"));
    }
}
