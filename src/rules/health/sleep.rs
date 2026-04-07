//! 睡眠规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 睡眠规则
pub struct SleepRules {
    metadata: RuleMetadata,
}

impl SleepRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "睡眠规则",
                "健康睡眠基础知识"
            )
            .with_origin("睡眠医学")
            .with_tags(vec!["健康".into(), "睡眠".into()]),
        }
    }

    /// 睡眠时长建议
    pub fn sleep_duration(&self) -> Vec<&'static str> {
        vec![
            "新生儿: 14-17小时",
            "婴儿: 12-15小时",
            "幼儿: 11-14小时",
            "学龄儿童: 9-11小时",
            "青少年: 8-10小时",
            "成年人: 7-9小时",
            "老年人: 7-8小时",
        ]
    }

    /// 睡眠周期
    pub fn sleep_cycles(&self) -> Vec<&'static str> {
        vec![
            "一个完整周期约90分钟",
            "浅睡眠(N1): 入睡阶段",
            "浅睡眠(N2): 睡眠加深",
            "深睡眠(N3): 恢复性睡眠",
            "快速眼动期(REM): 做梦阶段",
            "每晚会经历4-6个周期",
        ]
    }

    /// 良好睡眠习惯
    pub fn good_sleep_habits(&self) -> Vec<&'static str> {
        vec![
            "保持规律作息时间",
            "创造舒适睡眠环境",
            "睡前避免剧烈运动",
            "睡前避免咖啡因和酒精",
            "睡前避免使用电子设备",
            "睡前放松活动",
            "保持卧室安静黑暗",
        ]
    }

    /// 睡眠障碍
    pub fn sleep_disorders(&self) -> Vec<&'static str> {
        vec![
            "失眠: 难以入睡或维持睡眠",
            "睡眠呼吸暂停: 睡眠中呼吸中断",
            "不宁腿综合征: 腿部不适难以入睡",
            "睡眠过多: 日间过度嗜睡",
            "昼夜节律障碍: 睡眠时间错位",
            "夜惊和梦游: 睡眠中异常行为",
        ]
    }

    /// 改善睡眠建议
    pub fn improve_sleep(&self) -> Vec<&'static str> {
        vec![
            "固定就寝和起床时间",
            "睡前1小时避免屏幕",
            "保持卧室温度适宜(18-22度)",
            "使用舒适床垫和枕头",
            "白天适量运动",
            "睡前避免大量饮水",
            "尝试冥想或深呼吸",
        ]
    }

    /// 睡眠不足危害
    pub fn sleep_deprivation_effects(&self) -> Vec<&'static str> {
        vec![
            "注意力下降",
            "记忆力减退",
            "免疫力下降",
            "情绪波动",
            "增加肥胖风险",
            "增加心血管疾病风险",
            "影响学习和工作表现",
        ]
    }
}

impl Default for SleepRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SleepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sleep")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【睡眠规则】\n\n\
            睡眠时长建议:\n{}\n\n\
            良好睡眠习惯:\n{}\n\n\
            改善睡眠建议:\n{}\n",
            self.sleep_duration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.good_sleep_habits().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.improve_sleep().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_rules() {
        let rules = SleepRules::new();
        assert!(!rules.sleep_duration().is_empty());
    }
}