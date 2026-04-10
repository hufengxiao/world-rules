//! 皮划艇规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 皮划艇规则
pub struct KayakingRules {
    metadata: RuleMetadata,
}

impl KayakingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "皮划艇规则",
                "皮划艇比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 船艇类型
    pub fn boat_types(&self) -> Vec<&'static str> {
        vec![
            "皮艇(K): 双桨、坐姿",
            "划艇(C): 单桨、跪姿",
            "单人艇: K1, C1",
            "双人艇: K2, C2",
            "四人艇: K4, C4",
        ]
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "静水竞速: 直线比赛",
            "激流回旋: 障碍赛",
            "马拉松: 长距离",
            "激流速降: 速度赛",
            "花样皮划艇",
        ]
    }

    /// 静水竞速
    pub fn sprint_rules(&self) -> Vec<&'static str> {
        vec![
            "标准距离: 200米、500米、1000米",
            "直线航道",
            "9条航道",
            "固定起航系统",
            "计时精确到0.01秒",
        ]
    }

    /// 激流回旋
    pub fn slalom_rules(&self) -> Vec<&'static str> {
        vec![
            "人工或天然激流",
            "门杆悬挂: 红门逆流、绿门顺流",
            "18-25个门",
            "漏门罚50秒",
            "碰门罚2秒",
            "两轮成绩相加",
        ]
    }

    /// 马拉松比赛
    pub fn marathon_rules(&self) -> Vec<&'static str> {
        vec![
            "长距离比赛",
            "包含转弯和换艇区",
            "策略性比赛",
            "补给站",
            "团队战术",
        ]
    }

    /// 技术要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "正确的划桨技术",
            "平衡控制",
            "转弯技术",
            "翻艇自救",
            "安全意识",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "必须佩戴救生衣",
            "头盔: 激流比赛必须",
            "防水裙: 防止进水",
            "熟悉水域情况",
            "紧急信号",
        ]
    }

    /// 裁判职责
    pub fn official_duties(&self) -> Vec<&'static str> {
        vec![
            "发令员: 负责起航",
            "航道裁判: 监督航道",
            "终点裁判: 判定名次",
            "门裁判: 激流回旋",
            "安全监督",
        ]
    }
}

impl Default for KayakingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KayakingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kayaking")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【皮划艇规则】\n\n\
            船艇类型:\n{}\n\n\
            激流回旋:\n{}\n\n\
            技术要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.boat_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.slalom_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kayaking_rules() {
        let rules = KayakingRules::new();
        assert!(!rules.boat_types().is_empty());
    }
}