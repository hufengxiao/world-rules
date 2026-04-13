//! 激流回旋皮划艇规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 激流回旋皮划艇规则
pub struct SlalomKayakingRules {
    metadata: RuleMetadata,
}

impl SlalomKayakingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "激流回旋皮划艇规则",
                "激流回旋皮划艇比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 船艇类型
    pub fn boat_types(&self) -> Vec<&'static str> {
        vec![
            "K1: 单人皮艇",
            "C1: 单人划艇",
            "C2: 双人划艇",
            "不同长度规定",
            "重量限制",
        ]
    }

    /// 比赛场地
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "人工或天然激流",
            "长度约300米",
            "落差要求",
            "水流速度",
            "障碍设置",
        ]
    }

    /// 门杆规则
    pub fn gate_rules(&self) -> Vec<&'static str> {
        vec![
            "绿门: 顺流通过",
            "红门: 逆流通过",
            "18-25个门",
            "门杆悬挂高度",
            "门杆颜色标识",
        ]
    }

    /// 犯规罚分
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "碰门杆: 罚2秒",
            "漏门: 罚50秒",
            "门杆颜色错误通过: 罚50秒",
            "逆向通过: 罚50秒",
            "罚分计入成绩",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛两轮",
            "决赛一轮",
            "取最好成绩",
            "时间加罚分",
            "最快者获胜",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "皮划艇",
            "桨",
            "头盔必须佩戴",
            "救生衣必须",
            "防水裙",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔和救生衣必须",
            "翻艇自救能力",
            "安全艇在场",
            "医疗支持",
            "水位监控",
        ]
    }

    /// 技术要点
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "门杆通过技术",
            "水流利用",
            "转向技术",
            "速度控制",
            "翻艇自救",
        ]
    }
}

impl Default for SlalomKayakingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SlalomKayakingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("slalom_kayaking")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【激流回旋皮划艇规则】\n\n\
            门杆规则:\n{}\n\n\
            犯规罚分:\n{}\n\n\
            安全规则:\n{}\n\n\
            技术要点:\n{}\n",
            self.gate_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slalom_kayaking_rules() {
        let rules = SlalomKayakingRules::new();
        assert!(!rules.boat_types().is_empty());
    }
}