//! 日常舒展拉伸
//!
//! 缓解肌肉紧张、改善柔韧的日常拉伸与活动规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MuscleStretchRoutineRules,
    name: "日常舒展拉伸",
    desc: "缓解肌肉紧张、改善柔韧的日常拉伸与活动规则",
    origin: "国际",
    tags: ["健康", "拉伸", "柔韧", "久坐", "活动"]
}

impl MuscleStretchRoutineRules {
    /// 久坐活动
    pub fn sedentary(&self) -> Vec<&'static str> {
        vec![
            "每静坐约一小时起身活动",
            "伸展颈肩与腰背",
            "站坐交替减少久坐",
            "利用走动缓解僵硬",
        ]
    }

    /// 正确拉伸
    pub fn stretch(&self) -> Vec<&'static str> {
        vec![
            "循序渐进缓慢拉伸",
            "感到牵拉而非剧痛",
            "保持均匀呼吸",
            "勿过度弹震用力",
        ]
    }

    /// 部位要点
    pub fn areas(&self) -> Vec<&'static str> {
        vec![
            "缓解肩颈可做转肩仰头",
            "久站可活动脚踝小腿",
            "腰背用温和屈伸",
            "针对长时间保持的姿势舒展",
        ]
    }

    /// 习惯养成
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "把拉伸融入日常如离坐时",
            "结合深呼吸放松",
            "运动后留出舒展时间",
            "持续较明显酸痛及时就医",
        ]
    }
}

impl Rule for MuscleStretchRoutineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("muscle_stretch")
    }

    fn explain(&self) -> String {
        format!(
            "【日常舒展拉伸】\n{}",
            [
                format!(
                    "久坐活动：\\n{}",
                    self.sedentary()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确拉伸：\\n{}",
                    self.stretch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "部位要点：\\n{}",
                    self.areas()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "习惯养成：\\n{}",
                    self.routine()
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
    fn test_musclestretchroutinerules_basic() {
        let rules = MuscleStretchRoutineRules::new();
        assert_eq!(rules.metadata().name, "日常舒展拉伸");
        assert!(!rules.sedentary().is_empty());
        assert!(!rules.stretch().is_empty());
        assert!(!rules.areas().is_empty());
        assert!(!rules.routine().is_empty());
    }

    #[test]
    fn test_musclestretchroutinerules_validation() {
        let rules = MuscleStretchRoutineRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("muscle_stretch"));
    }

    #[test]
    fn test_musclestretchroutinerules_explain() {
        let rules = MuscleStretchRoutineRules::new();
        let e = rules.explain();
        assert!(e.contains("久坐活动"));
        assert!(e.contains("正确拉伸"));
        assert!(e.contains("部位要点"));
    }
}
