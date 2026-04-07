//! 举重规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 举重规则
pub struct WeightliftingRules {
    metadata: RuleMetadata,
}

impl WeightliftingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "举重规则",
                "IWF 国际举重联合会标准规则"
            )
            .with_origin("IWF")
            .with_tags(vec!["体育".into(), "举重".into()]),
        }
    }

    /// 比赛动作
    pub fn lifts(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("抓举 (Snatch)", "一次性将杠铃举过头顶"),
            ("挺举 (Clean & Jerk)", "先提拉至肩，再推举过头顶"),
        ]
    }

    /// 男子重量级别
    pub fn men_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "55公斤级",
            "61公斤级",
            "67公斤级",
            "73公斤级",
            "81公斤级",
            "89公斤级",
            "96公斤级",
            "102公斤级",
            "109公斤级",
            "+109公斤级",
        ]
    }

    /// 女子重量级别
    pub fn women_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "45公斤级",
            "49公斤级",
            "55公斤级",
            "59公斤级",
            "64公斤级",
            "71公斤级",
            "76公斤级",
            "81公斤级",
            "87公斤级",
            "+87公斤级",
        ]
    }

    /// 试举规则
    pub fn attempt_rules(&self) -> Vec<&'static str> {
        vec![
            "每种动作各3次试举机会",
            "必须成功完成动作",
            "每次试举有时间限制(1分钟)",
            "可更改试举重量",
            "重量只能增加不能减少",
        ]
    }

    /// 判定标准
    pub fn judgment_criteria(&self) -> Vec<&'static str> {
        vec![
            "双臂伸直",
            "杠铃静止在头顶",
            "双脚并拢站立",
            "等待裁判信号放下",
            "动作必须连续",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "举起后屈肘",
            "试举时间结束未完成",
            "杠铃掉落",
            "双脚踏出举重台",
            "放下杠铃时手未松开",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "总成绩 = 抓举最好成绩 + 挺举最好成绩",
            "成绩相同时体重轻者排名靠前",
            "世界纪录需超过1公斤",
        ]
    }
}

impl Default for WeightliftingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WeightliftingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("weightlifting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let lifts = self.lifts();
        format!(
            "【举重规则】\n\n\
            比赛动作:\n{}\n\n\
            男子重量级别:\n{}\n\n\
            女子重量级别:\n{}\n\n\
            试举规则:\n{}\n\n\
            判定标准:\n{}\n\n\
            计分方式:\n{}\n",
            lifts.iter().map(|(n, d)| format!("  • {}: {}", n, d)).collect::<Vec<_>>().join("\n"),
            self.men_weight_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.women_weight_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.attempt_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.judgment_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weightlifting_rules() {
        let rules = WeightliftingRules::new();
        assert_eq!(rules.lifts().len(), 2);
    }
}