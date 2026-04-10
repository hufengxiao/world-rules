//! 短道速滑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 短道速滑规则
pub struct ShortTrackRules {
    metadata: RuleMetadata,
}

impl ShortTrackRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "短道速滑规则",
                "短道速滑比赛基本规则"
            )
            .with_origin("加拿大")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 场地规格
    pub fn rink_specifications(&self) -> Vec<&'static str> {
        vec![
            "冰场: 30米×60米",
            "跑道: 111.12米一圈",
            "弯道半径: 8米",
            "直道长度: 28.85米",
            "弯道无道线",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "500米: 4.5圈",
            "1000米: 9圈",
            "1500米: 13.5圈",
            "3000米: 27圈",
            "接力: 5000米(女子)、5000米(男子)",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛: 分组晋级",
            "复赛: 中间轮次",
            "半决赛: 决定决赛",
            "决赛A: 争夺奖牌",
            "决赛B: 排名赛",
        ]
    }

    /// 起跑规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "起跑位置: 内侧向外排列",
            "随机分配起跑位置",
            "预备口令: Ready",
            "起跑口令: Go",
            "抢跑警告后取消资格",
        ]
    }

    /// 交接规则
    pub fn relay_rules(&self) -> Vec<&'static str> {
        vec![
            "接力区: 全场任何位置",
            "身体接触交接",
            "队友助推",
            "每队4人",
            "每人滑行圈数不限",
        ]
    }

    /// 领先权规则
    pub fn passing_rules(&self) -> Vec<&'static str> {
        vec![
            "内侧选手有领先权",
            "超越需确保安全",
            "碰撞责任判定",
            "不得阻碍他人",
            "不得横穿跑道",
        ]
    }

    /// 犯规处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "阻挡犯规: 取消资格",
            "碰撞犯规: 取消资格",
            "协助犯规: 取消资格",
            "危险动作: 取消资格",
            "未完成比赛: 无成绩",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "冰刀: 左弯设计",
            "手套: 安全保护",
            "头盔: 必须佩戴",
            "护颈: 安全保护",
            "护膝和护腿",
        ]
    }
}

impl Default for ShortTrackRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShortTrackRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("short_track")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【短道速滑规则】\n\n\
            比赛距离:\n{}\n\n\
            起跑规则:\n{}\n\n\
            领先权规则:\n{}\n\n\
            犯规处罚:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.start_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.passing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_track_rules() {
        let rules = ShortTrackRules::new();
        assert!(!rules.distances().is_empty());
    }
}