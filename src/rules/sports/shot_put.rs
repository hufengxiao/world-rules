//! 铅球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 铅球规则
pub struct ShotPutRules {
    metadata: RuleMetadata,
}

impl ShotPutRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "铅球规则",
                "铅球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 铅球重量
    pub fn shot_weights(&self) -> Vec<&'static str> {
        vec![
            "男子: 7.26公斤",
            "女子: 4公斤",
            "青年男子: 6公斤",
            "青年女子: 4公斤",
            "材质: 铁或铜制",
        ]
    }

    /// 投掷区规格
    pub fn circle_specifications(&self) -> Vec<&'static str> {
        vec![
            "投掷圈直径: 2.135米",
            "圈高: 6-10厘米",
            "抵趾板长度: 1.22米",
            "抵趾板高度: 10厘米",
            "投掷区平整",
        ]
    }

    /// 有效区域
    pub fn landing_sector(&self) -> Vec<&'static str> {
        vec![
            "有效区域: 40度扇形",
            "扇形区标记清晰",
            "测量距离",
            "边界线",
            "落地判定",
        ]
    }

    /// 投掷技术
    pub fn throwing_techniques(&self) -> Vec<&'static str> {
        vec![
            "推掷: 直线推球",
            "旋转掷: 旋转后推球",
            "背向投掷",
            "侧向投掷",
            "必须在圈内完成",
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

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩出投掷圈",
            "球落在扇形区外",
            "触碰抵趾板外侧",
            "投掷时间超时",
            "违规投掷方式",
        ]
    }

    /// 测量规则
    pub fn measurement(&self) -> Vec<&'static str> {
        vec![
            "从球落地痕迹测量",
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

impl Default for ShotPutRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShotPutRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shot_put")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【铅球规则】\n\n\
            铅球重量:\n{}\n\n\
            投掷技术:\n{}\n\n\
            犯规规则:\n{}\n\n\
            比赛规则:\n{}\n",
            self.shot_weights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_shot_put_rules() {
        let rules = ShotPutRules::new();
        assert!(!rules.shot_weights().is_empty());
    }
}