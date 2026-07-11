//! 形意拳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 形意拳规则
pub struct XingyiRules {
    metadata: RuleMetadata,
}

impl XingyiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("形意拳规则", "形意拳比赛与训练基本规则")
                .with_origin("中国")
                .with_tags(vec!["体育".into(), "武术".into(), "内家拳".into()]),
        }
    }

    /// 五行拳法
    pub fn five_elements(&self) -> Vec<&'static str> {
        vec![
            "崩拳: 木，直线出击",
            "炮拳: 火，上下发力",
            "横拳: 土，横向击打",
            "劈拳: 金，劈砍动作",
            "钻拳: 水，螺旋钻击",
        ]
    }

    /// 十二形拳
    pub fn twelve_animals(&self) -> Vec<&'static str> {
        vec![
            "龙形: 蜿蜒起伏",
            "虎形: 威猛扑击",
            "猴形: 灵活跳跃",
            "马形: 奔踢冲撞",
            "鼍形: 蛇龟结合",
            "鸡形: 金鸡独立",
            "燕形: 燕子抄水",
            "蛇形: 蜿蜒缠绕",
            "骀形: 鸟形展翅",
            "鹰形: 抓击锐利",
            "熊形: 威严稳重",
            "鹤形: 翱翔凌空",
        ]
    }

    /// 基本套路
    pub fn forms(&self) -> Vec<&'static str> {
        vec!["五行连环拳", "十二形合练", "杂势捶", "安身炮", "八字功"]
    }

    /// 技法特点
    pub fn characteristics(&self) -> Vec<&'static str> {
        vec![
            "直进直退: 直线移动",
            "硬打硬开: 强硬发力",
            "寸劲发力: 短距离发力",
            "气沉丹田: 内功修炼",
            "手眼身法步: 整体协调",
        ]
    }

    /// 三体式站桩
    pub fn standing_post(&self) -> Vec<&'static str> {
        vec![
            "三体式: 基础桩法",
            "浑圆桩: 内功桩法",
            "降龙桩: 降式桩法",
            "伏虎桩: 低式桩法",
            "站桩要求: 时间和姿势",
        ]
    }

    /// 训练方法
    pub fn training_methods(&self) -> Vec<&'static str> {
        vec![
            "站桩练气: 内功基础",
            "单式练习: 分解训练",
            "五行拳练习: 核心拳法",
            "十二形练习: 形意变化",
            "实战应用: 对抗训练",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 动作评分",
            "推手比赛: 技术对抗",
            "散手比赛: 技击对抗",
            "时间限制: 套路时长",
            "裁判评分制",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 比赛必备",
            "控制发力: 训练安全",
            "医疗支持: 赛场保障",
            "循序渐进: 学习进度",
            "禁止危险动作",
        ]
    }
}

impl Default for XingyiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for XingyiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("xingyi")
    }

    fn explain(&self) -> String {
        format!(
            "【形意拳规则】\n\n\
            五行拳法:\n{}\n\n\
            十二形拳:\n{}\n\n\
            技法特点:\n{}\n\n\
            三体式站桩:\n{}\n\n\
            安全规则:\n{}\n",
            self.five_elements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.twelve_animals()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.standing_post()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xingyi_rules() {
        let rules = XingyiRules::new();
        assert!(!rules.five_elements().is_empty());
        assert!(!rules.twelve_animals().is_empty());
        assert_eq!(rules.five_elements().len(), 5);
        assert_eq!(rules.twelve_animals().len(), 12);
    }

    #[test]
    fn test_xingyi_forms() {
        let rules = XingyiRules::new();
        let forms = rules.forms();
        assert!(forms.contains(&"五行连环拳"));
        assert!(forms.contains(&"杂势捶"));
    }
}
