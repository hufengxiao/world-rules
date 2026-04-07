//! 心理学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 心理学规则
pub struct PsychologyRules {
    metadata: RuleMetadata,
}

impl PsychologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "心理学定律",
                "心理学基本定律和效应"
            )
            .with_origin("心理学")
            .with_tags(vec!["科学".into(), "心理学".into()]),
        }
    }

    /// 常见心理效应
    pub fn psychological_effects(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("首因效应", "第一印象对后续认知的影响"),
            ("近因效应", "最新信息对印象的改变"),
            ("光环效应", "一个优点影响整体评价"),
            ("从众效应", "跟随大众行为的倾向"),
            ("破窗效应", "小问题不管会导致大问题"),
        ]
    }

    /// 认知偏差
    pub fn cognitive_biases(&self) -> Vec<&'static str> {
        vec![
            "确认偏差: 只寻找支持自己观点的证据",
            "锚定效应: 过度依赖第一条信息",
            "幸存者偏差: 只看到成功案例",
            "达克效应: 无知者往往自信",
            "损失厌恶: 损失的痛苦大于获得的快乐",
        ]
    }

    /// 学习理论
    pub fn learning_theories(&self) -> Vec<&'static str> {
        vec![
            "经典条件反射: 巴甫洛夫的狗",
            "操作性条件反射: 行为后果影响行为",
            "社会学习理论: 观察模仿学习",
            "认知学习理论: 理解和思考在学习中的作用",
        ]
    }

    /// 记忆规律
    pub fn memory_laws(&self) -> Vec<&'static str> {
        vec![
            "艾宾浩斯遗忘曲线: 遗忘先快后慢",
            "前摄抑制: 之前的学习干扰新学习",
            "倒摄抑制: 新学习干扰旧记忆",
            "系列位置效应: 首尾记得最牢",
        ]
    }

    /// 动机理论
    pub fn motivation_theories(&self) -> Vec<&'static str> {
        vec![
            "马斯洛需求层次: 生理→安全→社交→尊重→自我实现",
            "赫茨伯格双因素: 保健因素和激励因素",
            "期望理论: 努力导致绩效，绩效导致奖励",
        ]
    }
}

impl Default for PsychologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PsychologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("psychology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let effects = self.psychological_effects();
        format!(
            "【心理学定律】\n\n\
            常见心理效应:\n{}\n\n\
            认知偏差:\n{}\n\n\
            学习理论:\n{}\n\n\
            记忆规律:\n{}\n",
            effects.iter().map(|(n, d)| format!("  • {}: {}", n, d)).collect::<Vec<_>>().join("\n"),
            self.cognitive_biases().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.learning_theories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.memory_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psychology_rules() {
        let rules = PsychologyRules::new();
        assert!(!rules.psychological_effects().is_empty());
    }
}