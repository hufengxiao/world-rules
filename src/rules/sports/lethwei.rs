//! 缅甸拳击规则
//!
//! 缅甸传统武术Lethwei，以头击和无护具闻名

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 缅甸拳击规则
pub struct LethweiRules {
    metadata: RuleMetadata,
}

impl LethweiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("缅甸拳击规则", "缅甸传统拳击Lethwei竞赛规则")
                .with_origin("缅甸")
                .with_tags(vec!["体育".into(), "武术".into(), "格斗".into()]),
        }
    }

    /// 特点规则
    pub fn unique_characteristics(&self) -> Vec<&'static str> {
        vec![
            "允许头击: 缅甸拳独有特点",
            "无拳套: 裸拳或缠手带",
            "无护具: 传统无护具比赛",
            "五回合制: 每回合3分钟",
            "倒地判负: 传统规则无计分",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "传统比赛: 5回合",
            "每回合3分钟",
            "回合间休息2分钟",
            "现代比赛: 可缩短至3回合",
            "锦标赛: 决赛5回合",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、上勾拳",
            "腿法: 扫腿、蹬腿、勾踢",
            "膝击: 正膝、侧膝、飞膝",
            "肘击: 横肘、竖肘、转身肘",
            "头击: 用头部撞击对手",
            "摔法: 扫摔、缠抱摔",
            "缠抱攻击: 近身膝肘连击",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "咬人",
            "撕扯头发",
            "攻击已倒地对手",
            "使用武器",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "轻量级: 57kg以下",
            "次中量级: 57-67kg",
            "中量级: 67-72kg",
            "次重量级: 72-81kg",
            "重量级: 81kg以上",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "KO胜利: 对手无法继续",
            "对手弃权",
            "对手团队抛毛巾",
            "裁判终止比赛",
            "对手累计四次倒地",
            "判定胜利(现代规则)",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "缠手带: 传统棉布带",
            "短裤: 缅甸拳专用",
            "无拳套: 裸拳比赛",
            "无头盔: 传统规则",
            "无护胫: 传统规则",
            "现代比赛可选护具",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "传统擂台: 7.3米见方",
            "地面: 稻草或软垫",
            "围绳: 传统可能无围绳",
            "现代擂台: 标准拳击擂台",
            "两个选手角落",
        ]
    }

    /// 传统仪式
    pub fn traditional_rituals(&self) -> Vec<&'static str> {
        vec![
            "赛前舞蹈: Lethwei yay",
            "祈祷仪式",
            "缠手仪式",
            "向老师致敬",
            "擂台祭拜",
        ]
    }
}

impl Default for LethweiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LethweiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lethwei")
    }

    fn explain(&self) -> String {
        format!(
            "【缅甸拳击规则】\n\n\
            特点规则:\n{}\n\n\
            允许技法:\n{}\n\n\
            胜利条件:\n{}\n\n\
            护具要求:\n{}\n",
            self.unique_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.victory_conditions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
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
    fn test_lethwei_rules() {
        let rules = LethweiRules::new();
        assert_eq!(rules.metadata().name, "缅甸拳击规则");
        assert!(!rules.unique_characteristics().is_empty());
    }

    #[test]
    fn test_lethwei_headbutt() {
        let rules = LethweiRules::new();
        let techniques = rules.permitted_techniques();
        assert!(techniques.iter().any(|t| t.contains("头击")));
    }

    #[test]
    fn test_lethwei_equipment() {
        let rules = LethweiRules::new();
        let equip = rules.equipment();
        assert!(equip
            .iter()
            .any(|e| e.contains("缠手带") || e.contains("无拳套")));
    }
}
