//! 游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 泳姿类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwimmingStyle {
    /// 自由泳
    Freestyle,
    /// 蛙泳
    Breaststroke,
    /// 蝶泳
    Butterfly,
    /// 仰泳
    Backstroke,
    /// 混合泳
    Medley,
}

impl SwimmingStyle {
    pub fn name(&self) -> &'static str {
        match self {
            SwimmingStyle::Freestyle => "自由泳",
            SwimmingStyle::Breaststroke => "蛙泳",
            SwimmingStyle::Butterfly => "蝶泳",
            SwimmingStyle::Backstroke => "仰泳",
            SwimmingStyle::Medley => "混合泳",
        }
    }

    pub fn english_name(&self) -> &'static str {
        match self {
            SwimmingStyle::Freestyle => "Freestyle",
            SwimmingStyle::Breaststroke => "Breaststroke",
            SwimmingStyle::Butterfly => "Butterfly",
            SwimmingStyle::Backstroke => "Backstroke",
            SwimmingStyle::Medley => "Individual Medley",
        }
    }
}

/// 游泳规则
pub struct SwimmingRules {
    metadata: RuleMetadata,
}

impl SwimmingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "游泳规则",
                "FINA 国际泳联标准规则"
            )
            .with_origin("FINA")
            .with_tags(vec!["体育".into(), "游泳".into()]),
        }
    }

    /// 标准比赛距离
    pub fn standard_distances(&self) -> Vec<u16> {
        vec![50, 100, 200, 400, 800, 1500]
    }

    /// 泳池规格
    pub fn pool_specifications(&self) -> Vec<&'static str> {
        vec![
            "奥运会标准池: 50米长",
            "短池: 25米长",
            "宽度: 至少21米 (8条泳道)",
            "深度: 至少1.35米",
            "泳道宽度: 2.5米",
            "水温: 25-28°C",
        ]
    }

    /// 出发规则
    pub fn starting_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳、蛙泳、蝶泳: 跳台出发",
            "仰泳: 水中出发",
            "听到哨声后上跳台",
            "枪响后跳入水中",
            "抢跳: 取消比赛资格",
        ]
    }

    /// 转身规则
    pub fn turn_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳: 可滚翻转身",
            "蛙泳: 必须双手触壁",
            "蝶泳: 必须双手触壁",
            "仰泳: 可滚翻转身",
            "混合泳: 按各泳姿规则",
        ]
    }

    /// 各泳姿技术规则
    pub fn style_rules(&self, style: SwimmingStyle) -> Vec<&'static str> {
        match style {
            SwimmingStyle::Freestyle => vec![
                "可使用任何泳姿",
                "通常采用爬泳",
                "转身和终点可触壁任意部位",
            ],
            SwimmingStyle::Breaststroke => vec![
                "双手必须同时划水",
                "双腿必须同时蹬腿",
                "禁止蝶泳腿",
                "每次划水后头须露出水面",
                "转身和终点双手同时触壁",
            ],
            SwimmingStyle::Butterfly => vec![
                "双臂必须同时向前挥动",
                "双腿必须同时上下打水",
                "身体保持俯卧",
                "转身和终点双手同时触壁",
            ],
            SwimmingStyle::Backstroke => vec![
                "全程保持仰卧",
                "禁止翻转超过90°",
                "转身时可翻转",
                "终点必须仰卧触壁",
            ],
            SwimmingStyle::Medley => vec![
                "顺序: 蝶泳→仰泳→蛙泳→自由泳",
                "每种泳姿游1/4距离",
                "按各泳姿规则执行",
            ],
        }
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抢跳",
            "越线进入他人泳道",
            "使用 illegal 技术动作",
            "转身或终点触壁不合规",
            "仰泳站立或行走",
        ]
    }

    /// 世界纪录级别
    pub fn record_types(&self) -> Vec<&'static str> {
        vec![
            "世界纪录 (World Record)",
            "奥运会纪录 (Olympic Record)",
            "洲际纪录 (Continental Record)",
            "国家纪录 (National Record)",
        ]
    }
}

impl Default for SwimmingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【游泳规则】\n\n\
            泳池规格:\n{}\n\n\
            标准距离: {}米\n\n\
            出发规则:\n{}\n\n\
            转身规则:\n{}\n\n\
            犯规行为:\n{}\n",
            self.pool_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.standard_distances().iter().map(|d| d.to_string()).collect::<Vec<_>>().join("/"),
            self.starting_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.turn_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swimming_rules() {
        let rules = SwimmingRules::new();
        assert!(rules.standard_distances().contains(&100));
    }
}