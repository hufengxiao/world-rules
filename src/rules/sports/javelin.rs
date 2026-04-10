//! 标枪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 标枪规则
pub struct JavelinRules {
    metadata: RuleMetadata,
}

impl JavelinRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "标枪规则",
                "标枪比赛基本规则"
            )
            .with_origin("古代希腊")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 标枪规格
    pub fn javelin_specifications(&self) -> Vec<&'static str> {
        vec![
            "男子标枪: 长2.6-2.7米，重800克",
            "女子标枪: 长2.2-2.3米，重600克",
            "金属或复合材料",
            "枪头必须金属制",
            "重心位置规定",
        ]
    }

    /// 投掷区规格
    pub fn runway_specifications(&self) -> Vec<&'static str> {
        vec![
            "助跑道长: 30-36.5米",
            "助跑道宽: 4米",
            "起掷弧半径: 8米",
            "投掷区扇形角: 29度",
            "两侧边线",
        ]
    }

    /// 投掷技术
    pub fn throwing_techniques(&self) -> Vec<&'static str> {
        vec![
            "助跑投掷",
            "交叉步",
            "出手角度约35度",
            "标枪必须枪头先落地",
            "正确握枪姿势",
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
            "枪头先落地",
            "落地痕迹完整",
            "在扇形区域内",
            "标枪不能破碎",
            "标枪整体落地",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩出助跑道",
            "跨过起掷弧",
            "枪尾先落地",
            "标枪落地后触碰",
            "投掷时间超时",
        ]
    }

    /// 测量规则
    pub fn measurement(&self) -> Vec<&'static str> {
        vec![
            "从枪头痕迹测量",
            "测量至起掷弧中心",
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
            "人员不得进入",
            "标枪回收安全",
        ]
    }
}

impl Default for JavelinRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for JavelinRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("javelin")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【标枪规则】\n\n\
            标枪规格:\n{}\n\n\
            投掷技术:\n{}\n\n\
            犯规规则:\n{}\n\n\
            比赛规则:\n{}\n",
            self.javelin_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_javelin_rules() {
        let rules = JavelinRules::new();
        assert!(!rules.javelin_specifications().is_empty());
    }
}