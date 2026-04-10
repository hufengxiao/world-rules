//! 飞镖规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 飞镖规则
pub struct DartsRules {
    metadata: RuleMetadata,
}

impl DartsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "飞镖规则",
                "飞镖比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "室内".into()]),
        }
    }

    /// 镖盘规格
    pub fn board_specifications(&self) -> Vec<&'static str> {
        vec![
            "镖盘直径: 45.1厘米",
            "投掷距离: 2.37米",
            "靶心: 红色中心(50分)",
            "外靶心: 绿色圆环(25分)",
            "20个扇区交替红绿",
        ]
    }

    /// 得分区域
    pub fn scoring_areas(&self) -> Vec<&'static str> {
        vec![
            "靶心(牛眼): 50分",
            "外靶心: 25分",
            "三倍环: 分值×3",
            "双倍环: 分值×2",
            "单倍区: 面积分值",
        ]
    }

    /// 比赛形式
    pub fn game_formats(&self) -> Vec<&'static str> {
        vec![
            "501/301: 从指定分数减到0",
            "板球: 击中特定数字",
            "围绕钟: 顺序击中1-20",
            "团体赛和个人赛",
            "每轮投3镖",
        ]
    }

    /// 501规则
    pub fn rules_501(&self) -> Vec<&'static str> {
        vec![
            "起始分数501分",
            "每镖减去命中分数",
            "必须精确归零",
            "最后一镖必须双倍区结束",
            "爆分: 超过剩余分数",
        ]
    }

    /// 板球规则
    pub fn cricket_rules(&self) -> Vec<&'static str> {
        vec![
            "目标数字: 20,19,18,17,16,15,靶心",
            "每个数字需命中3次",
            "命中三倍算3次",
            "命中双倍算2次",
            "全部关闭后获胜",
        ]
    }

    /// 常见术语
    pub fn terminology(&self) -> Vec<&'static str> {
        vec![
            "180分: 单轮最高分",
            "九镖完赛: 完美比赛",
            "三倍20: 最高单镖60分",
            " checkout: 结束比赛",
            "爆分: 超过剩余分数",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越过投掷线",
            "投掷超过规定时间",
            "干扰对手",
            "不当行为",
            "取消该轮得分",
        ]
    }
}

impl Default for DartsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DartsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("darts")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【飞镖规则】\n\n\
            镖盘规格:\n{}\n\n\
            得分区域:\n{}\n\n\
            501规则:\n{}\n\n\
            常见术语:\n{}\n",
            self.board_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_areas().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rules_501().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.terminology().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_darts_rules() {
        let rules = DartsRules::new();
        assert!(!rules.board_specifications().is_empty());
    }
}