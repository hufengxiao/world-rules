//! 跆拳道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跆拳道规则
pub struct TaekwondoRules {
    metadata: RuleMetadata,
}

impl TaekwondoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跆拳道规则",
                "跆拳道比赛基本规则"
            )
            .with_origin("韩国")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子奥运: -58kg, -68kg, -80kg, +80kg",
            "女子奥运: -49kg, -57kg, -67kg, +67kg",
            "世锦赛级别更多",
        ]
    }

    /// 得分区域
    pub fn scoring_areas(&self) -> Vec<&'static str> {
        vec![
            "躯干: 胸部和腹部(护具覆盖区域)",
            "头部: 头部正面和侧面",
            "禁止攻击背部",
            "禁止攻击后脑",
        ]
    }

    /// 得分分值
    pub fn point_values(&self) -> Vec<&'static str> {
        vec![
            "躯干正踢: 2分",
            "躯干旋转踢: 3分",
            "躯干腾空踢: 4分",
            "头部正踢: 3分",
            "头部旋转踢: 4分",
            "头部腾空踢: 5分",
            "击倒: 加1分",
        ]
    }

    /// 比赛回合
    pub fn rounds(&self) -> Vec<&'static str> {
        vec![
            "奥运比赛: 3回合",
            "每回合2分钟",
            "回合间休息1分钟",
            "金分加时: 1分钟",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击禁止区域",
            "用拳攻击头部",
            "推搡对手",
            "抓对手",
            "故意拖延",
            "假装受伤",
            "越界",
            "攻击倒地对手",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "头盔: 必须佩戴",
            "护胸: 必须佩戴",
            "护腿: 必须佩戴",
            "护臂: 必须佩戴",
            "护齿: 必须佩戴",
            "手套: 必须佩戴",
            "电子护具系统(奥运会)",
        ]
    }

    /// 判定规则
    pub fn winning_criteria(&self) -> Vec<&'static str> {
        vec![
            "得分高者获胜",
            "击倒对手获胜(KO)",
            "对手被判犯规出局",
            "金分加时: 先得分者获胜",
            "加时后平分: 裁判判定",
        ]
    }
}

impl Default for TaekwondoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TaekwondoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("taekwondo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跆拳道规则】\n\n\
            得分区域:\n{}\n\n\
            得分分值:\n{}\n\n\
            比赛回合:\n{}\n\n\
            禁止行为:\n{}\n",
            self.scoring_areas().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.point_values().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rounds().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taekwondo_rules() {
        let rules = TaekwondoRules::new();
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.point_values().is_empty());
    }
}