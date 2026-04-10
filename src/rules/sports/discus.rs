//! 铁饼规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 铁饼规则
pub struct DiscusRules {
    metadata: RuleMetadata,
}

impl DiscusRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "铁饼规则",
                "铁饼比赛基本规则"
            )
            .with_origin("古代希腊")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 铁饼规格
    pub fn discus_specifications(&self) -> Vec<&'static str> {
        vec![
            "男子铁饼: 重2公斤，直径22厘米",
            "女子铁饼: 重1公斤，直径18厘米",
            "圆形盘状",
            "木质或金属制",
            "边缘厚度规定",
        ]
    }

    /// 投掷区规格
    pub fn circle_specifications(&self) -> Vec<&'static str> {
        vec![
            "投掷圈直径: 2.5米",
            "圈高: 6-10厘米",
            "铁制投掷圈",
            "投掷区平整",
            "有效扇形区34.92度",
        ]
    }

    /// 投掷技术
    pub fn throwing_techniques(&self) -> Vec<&'static str> {
        vec![
            "旋转投掷",
            "1.5-2圈旋转",
            "出手角度约35度",
            "手指握饼",
            "平稳投出",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每人3次试投",
            "取最好成绩",
            "决赛8人",
            "决赛6次试投",
            "最好成绩获胜",
        ]
    }

    /// 有效落地
    pub fn valid_land(&self) -> Vec<&'static str> {
        vec![
            "铁饼在扇形区内落地",
            "落地痕迹完整",
            "铁饼不能破碎",
            "铁饼整体落地",
            "边界线判定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩出投掷圈",
            "投掷时间超时",
            "球落在扇形区外",
            "投掷后从圈前退出",
            "违规投掷方式",
        ]
    }

    /// 测量规则
    pub fn measurement(&self) -> Vec<&'static str> {
        vec![
            "从铁饼落地痕迹测量",
            "测量至投掷圈中心",
            "精确到0.01米",
            "金属卷尺测量",
            "测量员记录",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "投掷区警示",
            "裁判控制比赛",
            "等待安全信号",
            "指定安全区域",
            "人员不得进入投掷区",
        ]
    }
}

impl Default for DiscusRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DiscusRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("discus")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【铁饼规则】\n\n\
            铁饼规格:\n{}\n\n\
            投掷技术:\n{}\n\n\
            犯规规则:\n{}\n\n\
            比赛规则:\n{}\n",
            self.discus_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.throwing_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discus_rules() {
        let rules = DiscusRules::new();
        assert!(!rules.discus_specifications().is_empty());
    }
}