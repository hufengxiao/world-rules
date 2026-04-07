//! 冰球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冰球规则
pub struct IceHockeyRules {
    metadata: RuleMetadata,
}

impl IceHockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冰球规则",
                "冰球比赛基本规则"
            )
            .with_origin("加拿大")
            .with_tags(vec!["体育".into(), "冰上".into()]),
        }
    }

    /// 场地规格
    pub fn rink_dimensions(&self) -> Vec<&'static str> {
        vec![
            "国际标准: 长60米，宽30米",
            "NHL标准: 长61米，宽26米",
            "圆角设计",
            "冰面厚度约2.5厘米",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 6人(包括守门员)",
            "位置: 守门员、2名后卫、3名前锋",
            "替补席: 可随时换人",
            "换人频繁: 每几分钟换一组",
        ]
    }

    /// 比赛时间
    pub fn periods(&self) -> Vec<&'static str> {
        vec![
            "NHL: 3节，每节20分钟",
            "国际比赛: 3节，每节20分钟",
            "节间休息: 15-18分钟",
            "加时赛: 5分钟(3对3)",
            "季后赛加时: 无时间限制",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线得1分",
            "不能用手传球射门",
            "不能用脚故意踢球进门",
            "高杆射门: 球杆高于肩部无效",
        ]
    }

    /// 犯规类型
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "小罚: 2分钟",
            "双小罚: 4分钟",
            "大罚: 5分钟",
            "违例罚: 10分钟",
            "严重违例: 取消比赛资格",
            "守门员犯规: 由其他队员代罚",
        ]
    }

    /// 常见犯规
    pub fn common_fouls(&self) -> Vec<&'static str> {
        vec![
            "绊人",
            "用球杆勾人",
            "冲撞",
            "肘击",
            "打架",
            "高杆(球杆高于肩部)",
            "干扰",
            "延迟比赛",
        ]
    }

    /// 越位规则
    pub fn offside(&self) -> Vec<&'static str> {
        vec![
            "攻方队员先于球进入攻区",
            "越位时比赛暂停",
            "在攻区边线争球",
            "延迟越位: 可选择不吹停",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "头盔: 必须佩戴",
            "面罩: NHL可选，其他强制",
            "护胸: 强制",
            "护肘护腿: 强制",
            "手套: 强制",
            "球杆: 镰度不超过规定",
            "冰刀鞋: 强制",
        ]
    }
}

impl Default for IceHockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IceHockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_hockey")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冰球规则】\n\n\
            比赛时间:\n{}\n\n\
            犯规类型:\n{}\n\n\
            常见犯规:\n{}\n\n\
            越位规则:\n{}\n",
            self.periods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.common_fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.offside().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ice_hockey_rules() {
        let rules = IceHockeyRules::new();
        assert!(!rules.rink_dimensions().is_empty());
        assert!(!rules.penalties().is_empty());
    }
}