//! 现代五项规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 现代五项规则
pub struct ModernPentathlonRules {
    metadata: RuleMetadata,
}

impl ModernPentathlonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "现代五项规则",
                "现代五项比赛基本规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "综合".into()]),
        }
    }

    /// 五项项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "马术: 盛装舞步障碍赛",
            "击剑: 重剑比赛",
            "游泳: 200米自由泳",
            "射击: 气枪射击",
            "跑步: 跑步比赛",
        ]
    }

    /// 比赛顺序
    pub fn competition_order(&self) -> Vec<&'static str> {
        vec![
            "传统顺序: 马术、击剑、游泳、射击、跑步",
            "新顺序: 击剑、游泳、射击跑步结合",
            "一天内完成",
            "积分累计制",
            "射击跑步激光跑",
        ]
    }

    /// 马术规则
    pub fn riding_rules(&self) -> Vec<&'static str> {
        vec![
            "随机分配马匹",
            "350-450米路线",
            "12-15个障碍",
            "限时完成",
            "障碍扣分",
        ]
    }

    /// 击剑规则
    pub fn fencing_rules(&self) -> Vec<&'static str> {
        vec![
            "重剑比赛",
            "一对一循环赛",
            "一分钟对决",
            "击中即得分",
            "积分制",
        ]
    }

    /// 游泳规则
    pub fn swimming_rules(&self) -> Vec<&'static str> {
        vec![
            "200米自由泳",
            "泳池比赛",
            "计时制",
            "成绩转积分",
            "标准出发",
        ]
    }

    /// 射击规则
    pub fn shooting_rules(&self) -> Vec<&'static str> {
        vec![
            "10米气手枪",
            "靶心10环",
            "限时射击",
            "激光枪系统",
            "成绩转积分",
        ]
    }

    /// 跑步规则
    pub fn running_rules(&self) -> Vec<&'static str> {
        vec![
            "3公里越野跑",
            "激光跑: 与射击结合",
            "射击跑步交替",
            "出发顺序按积分",
            "最后到达时间决定名次",
        ]
    }

    /// 积分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "基准成绩1000分",
            "每项独立积分",
            "优劣加减分",
            "总积分决定名次",
            "激光跑出发顺序",
        ]
    }
}

impl Default for ModernPentathlonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ModernPentathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("modern_pentathlon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【现代五项规则】\n\n\
            五项项目:\n{}\n\n\
            比赛顺序:\n{}\n\n\
            积分系统:\n{}\n\n\
            马术规则:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_order().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.riding_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_pentathlon_rules() {
        let rules = ModernPentathlonRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}