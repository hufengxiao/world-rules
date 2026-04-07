//! 自行车比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 自行车比赛类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CyclingType {
    /// 公路自行车
    Road,
    /// 场地自行车
    Track,
    /// 山地自行车
    MountainBike,
    /// BMX
    BMX,
}

impl CyclingType {
    pub fn name(&self) -> &'static str {
        match self {
            CyclingType::Road => "公路自行车",
            CyclingType::Track => "场地自行车",
            CyclingType::MountainBike => "山地自行车",
            CyclingType::BMX => "小轮车(BMX)",
        }
    }
}

/// 自行车比赛规则
pub struct CyclingRules {
    metadata: RuleMetadata,
}

impl CyclingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "自行车比赛规则",
                "UCI 国际自行车联盟标准规则"
            )
            .with_origin("UCI")
            .with_tags(vec!["体育".into(), "自行车".into()]),
        }
    }

    /// 公路赛类型
    pub fn road_race_types(&self) -> Vec<&'static str> {
        vec![
            "大组赛: 同时出发，最先到达者胜",
            "个人计时赛: 间隔出发，用时最少者胜",
            "团体计时赛: 团队计时",
            "多日赛: 总成绩计分(如环法)",
        ]
    }

    /// 场地自行车项目
    pub fn track_events(&self) -> Vec<&'static str> {
        vec![
            "争先赛: 短距离对抗",
            "竞速赛: 三圈竞速",
            "追逐赛: 4公里追逐",
            "记分赛: 积分累计",
            "凯林赛: 摩托领骑后冲刺",
            "团体追逐赛: 4人团体",
        ]
    }

    /// 山地车项目
    pub fn mtb_events(&self) -> Vec<&'static str> {
        vec![
            "越野赛(XC): 山地越野",
            "速降赛(DH): 下坡计时",
            "耐力赛(ENDURO): 多段计时",
        ]
    }

    /// 装备要求
    pub fn equipment_rules(&self) -> Vec<&'static str> {
        vec![
            "必须佩戴头盔",
            "自行车需符合UCI标准",
            "禁止使用无线电通讯",
            "号码牌佩戴规范",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险骑行",
            "阻挡对手",
            "违规超车",
            "接受非规定帮助",
            "使用违禁物质",
        ]
    }

    /// 环法自行车赛
    pub fn tour_de_france(&self) -> Vec<&'static str> {
        vec![
            "总里程: 约3500公里",
            "赛程: 21个赛段",
            "休息日: 2天",
            "黄衫: 总成绩领先",
            "绿衫: 冲刺积分领先",
            "圆点衫: 爬坡积分领先",
            "白衫: 最佳年轻车手",
        ]
    }
}

impl Default for CyclingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CyclingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cycling")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【自行车比赛规则】\n\n\
            公路赛类型:\n{}\n\n\
            场地自行车项目:\n{}\n\n\
            山地车项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            环法自行车赛:\n{}\n",
            self.road_race_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.track_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mtb_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tour_de_france().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycling_rules() {
        let rules = CyclingRules::new();
        assert!(!rules.road_race_types().is_empty());
    }
}