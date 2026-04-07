//! 田径规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 田径项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AthleticsEvent {
    /// 短跑
    Sprint,
    /// 中长跑
    MiddleDistance,
    /// 长跑
    LongDistance,
    /// 跨栏
    Hurdles,
    /// 接力
    Relay,
    /// 跳高
    HighJump,
    /// 跳远
    LongJump,
    /// 三级跳远
    TripleJump,
    /// 撑杆跳高
    PoleVault,
    /// 铅球
    ShotPut,
    /// 铁饼
    Discus,
    /// 标枪
    Javelin,
    /// 链球
    Hammer,
}

impl AthleticsEvent {
    pub fn name(&self) -> &'static str {
        match self {
            AthleticsEvent::Sprint => "短跑",
            AthleticsEvent::MiddleDistance => "中长跑",
            AthleticsEvent::LongDistance => "长跑",
            AthleticsEvent::Hurdles => "跨栏",
            AthleticsEvent::Relay => "接力",
            AthleticsEvent::HighJump => "跳高",
            AthleticsEvent::LongJump => "跳远",
            AthleticsEvent::TripleJump => "三级跳远",
            AthleticsEvent::PoleVault => "撑杆跳高",
            AthleticsEvent::ShotPut => "铅球",
            AthleticsEvent::Discus => "铁饼",
            AthleticsEvent::Javelin => "标枪",
            AthleticsEvent::Hammer => "链球",
        }
    }
}

/// 田径规则
pub struct AthleticsRules {
    metadata: RuleMetadata,
}

impl AthleticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "田径规则",
                "WA 世界田联标准规则"
            )
            .with_origin("WA")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 跑道规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准跑道: 400米一圈",
            "8条跑道",
            "跑道宽度: 1.22米",
            "弯道半径: 36.5米",
            "跑道材质: 合成橡胶",
        ]
    }

    /// 短跑项目距离
    pub fn sprint_distances(&self) -> Vec<u16> {
        vec![60, 100, 200, 400]
    }

    /// 中长跑项目距离
    pub fn middle_distance_events(&self) -> Vec<u16> {
        vec![800, 1500]
    }

    /// 长跑项目距离
    pub fn long_distance_events(&self) -> Vec<u16> {
        vec![3000, 5000, 10000]
    }

    /// 跨栏规格
    pub fn hurdles_specifications(&self) -> Vec<&'static str> {
        vec![
            "男子110米栏: 栏高1.067米, 10个栏",
            "女子100米栏: 栏高0.84米, 10个栏",
            "男子400米栏: 栏高0.914米, 10个栏",
            "女子400米栏: 栏高0.762米, 10个栏",
        ]
    }

    /// 起跑规则
    pub fn starting_rules(&self) -> Vec<&'static str> {
        vec![
            "短跑: 使用起跑器",
            "各就位 → 预备 → 鸣枪",
            "抢跑: 第一次警告，第二次取消资格",
            "长跑: 站立起跑",
            "抢跑时间: 0.100秒内反应视为抢跑",
        ]
    }

    /// 接力规则
    pub fn relay_rules(&self) -> Vec<&'static str> {
        vec![
            "4×100米接力",
            "4×400米接力",
            "必须在接力区内完成交接棒",
            "接力区: 20米 (4×100米为30米)",
            "掉棒由原运动员捡起",
        ]
    }

    /// 跳远规则
    pub fn long_jump_rules(&self) -> Vec<&'static str> {
        vec![
            "助跑长度: 最长40米",
            "起跳板宽度: 20厘米",
            "有效试跳: 未踩线",
            "每人3次试跳，前8名再加3次",
            "测量: 起跳线到最近落地点",
        ]
    }

    /// 跳高规则
    pub fn high_jump_rules(&self) -> Vec<&'static str> {
        vec![
            "可自由选择起跳高度",
            "每高度有3次试跳机会",
            "可采用任何姿势过杆",
            "横杆掉落视为失败",
            "最终以最高高度排名",
        ]
    }

    /// 投掷项目规则
    pub fn throwing_rules(&self) -> Vec<&'static str> {
        vec![
            "每人3次试投，前8名再加3次",
            "必须在投掷圈内完成",
            "器械必须落在扇形区内",
            "不能触碰投掷圈外地面",
            "测量: 投掷圈中心到落地点",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "短跑抢跑",
            "跑出跑道 (影响他人)",
            "跳远踩线",
            "跳高碰落横杆",
            "投掷出界",
            "接力交接棒违规",
        ]
    }
}

impl Default for AthleticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AthleticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("athletics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【田径规则】\n\n\
            跑道规格:\n{}\n\n\
            短跑项目: {}米\n\
            中长跑项目: {}米\n\
            长跑项目: {}米\n\n\
            起跑规则:\n{}\n\n\
            跨栏规格:\n{}\n\n\
            接力规则:\n{}\n\n\
            跳远规则:\n{}\n\n\
            跳高规则:\n{}\n\n\
            投掷规则:\n{}\n",
            self.track_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.sprint_distances().iter().map(|d| d.to_string()).collect::<Vec<_>>().join("/"),
            self.middle_distance_events().iter().map(|d| d.to_string()).collect::<Vec<_>>().join("/"),
            self.long_distance_events().iter().map(|d| d.to_string()).collect::<Vec<_>>().join("/"),
            self.starting_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.hurdles_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.relay_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.long_jump_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.high_jump_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.throwing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_athletics_rules() {
        let rules = AthleticsRules::new();
        assert!(rules.sprint_distances().contains(&100));
    }
}