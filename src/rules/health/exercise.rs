//! 运动规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 运动规则
pub struct ExerciseRules {
    metadata: RuleMetadata,
}

impl ExerciseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "运动规则",
                "健康运动基础知识"
            )
            .with_origin("运动医学")
            .with_tags(vec!["健康".into(), "运动".into()]),
        }
    }

    /// 运动类型
    pub fn exercise_types(&self) -> Vec<&'static str> {
        vec![
            "有氧运动: 跑步、游泳、骑车等",
            "无氧运动: 举重、短跑等",
            "柔韧性训练: 瑜伽、拉伸等",
            "平衡训练: 太极、普拉提等",
            "混合训练: HIIT、CrossFit等",
        ]
    }

    /// 运动频率建议
    pub fn frequency_guidelines(&self) -> Vec<&'static str> {
        vec![
            "有氧运动: 每周至少150分钟中等强度",
            "或每周至少75分钟高强度有氧",
            "力量训练: 每周至少2次",
            "柔韧性训练: 每周至少2-3次",
            "久坐人群每小时起身活动几分钟",
        ]
    }

    /// 运动强度
    pub fn intensity_levels(&self) -> Vec<&'static str> {
        vec![
            "低强度: 可以轻松交谈",
            "中等强度: 能说话但不能唱歌",
            "高强度: 只能说几个字",
            "最大心率 = 220 - 年龄",
            "中等强度为最大心率的50-70%",
            "高强度为最大心率的70-85%",
        ]
    }

    /// 运动前准备
    pub fn warm_up(&self) -> Vec<&'static str> {
        vec![
            "热身5-10分钟",
            "轻度有氧运动提高体温",
            "动态拉伸活动关节",
            "逐渐增加运动强度",
            "避免突然剧烈运动",
        ]
    }

    /// 运动后恢复
    pub fn cool_down(&self) -> Vec<&'static str> {
        vec![
            "放松运动5-10分钟",
            "逐渐降低运动强度",
            "静态拉伸放松肌肉",
            "补充水分和电解质",
            "适当休息和睡眠",
        ]
    }

    /// 运动注意事项
    pub fn safety_tips(&self) -> Vec<&'static str> {
        vec![
            "运动前了解自身健康状况",
            "循序渐进增加运动量",
            "穿着合适的运动装备",
            "注意运动环境安全",
            "不适时立即停止运动",
            "慢性病患者咨询医生",
            "运动前后充分补水",
        ]
    }

    /// 常见运动伤害预防
    pub fn injury_prevention(&self) -> Vec<&'static str> {
        vec![
            "充分热身和放松",
            "使用正确的运动姿势",
            "不要过度训练",
            "加强薄弱肌肉群",
            "使用适当的保护装备",
            "注意休息和恢复",
        ]
    }
}

impl Default for ExerciseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ExerciseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("exercise")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【运动规则】\n\n\
            运动类型:\n{}\n\n\
            运动频率建议:\n{}\n\n\
            运动注意事项:\n{}\n",
            self.exercise_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.frequency_guidelines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_tips().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_rules() {
        let rules = ExerciseRules::new();
        assert!(!rules.exercise_types().is_empty());
    }
}