//! 高棉拳击规则
//!
//! 柬埔寨传统武术Pradal Serey/Bokator，高棉文化的格斗艺术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 高棉拳击规则
pub struct BokatorRules {
    metadata: RuleMetadata,
}

impl BokatorRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("高棉拳击规则", "柬埔寨传统武术Bokator/Pradal Serey竞赛规则")
                .with_origin("柬埔寨")
                .with_tags(vec!["体育".into(), "武术".into(), "格斗".into()]),
        }
    }

    /// 历史特点
    pub fn historical_characteristics(&self) -> Vec<&'static str> {
        vec![
            "起源于古代高棉帝国",
            "与吴哥窟雕刻相关",
            "战场武术演化",
            "动物形态模仿",
            "完整武术体系",
        ]
    }

    /// 动物形态
    pub fn animal_forms(&self) -> Vec<&'static str> {
        vec![
            "鹰形: 飞翔攻击和爪法",
            "龙形: 灵活变化攻击",
            "牛形: 强力冲击技术",
            "狮形: 猛烈攻击组合",
            "象形: 重型打击技术",
            "鹤形: 精准踢击技术",
            "蛇形: 缠绕摔投技术",
            "猴形: 敏捷闪避攻击",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "传统比赛: 5回合",
            "每回合3分钟",
            "回合间休息2分钟",
            "现代比赛: 3-5回合",
            "现代比赛带计分",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、勾拳",
            "肘击: 各种肘法攻击",
            "膝击: 正膝、侧膝、飞膝",
            "腿法: 扫腿、蹬腿、勾踢",
            "摔法: 扫摔、缠抱摔",
            "地面技: 部分地面攻击",
            "缠抱技术: 近身膝肘",
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
            "手指插眼",
            "关节技(部分赛事)",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "轻量级: 60kg以下",
            "次中量级: 60-67kg",
            "中量级: 67-72kg",
            "次重量级: 72-81kg",
            "重量级: 81kg以上",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "缠手带: 传统棉布带",
            "拳套: 现代比赛使用",
            "短裤: 高棉拳专用",
            "无头盔: 传统规则",
            "护齿: 现代比赛必须",
            "护裆: 推荐使用",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "KO胜利",
            "对手弃权",
            "裁判终止比赛",
            "判定胜利(现代规则)",
            "对手累计三次倒地",
        ]
    }

    /// 传统仪式
    pub fn traditional_rituals(&self) -> Vec<&'static str> {
        vec![
            "赛前舞蹈: Kun Kru",
            "祈祷仪式",
            "向老师致敬",
            "擂台祭拜",
            "音乐伴奏: 传统乐器",
        ]
    }
}

impl Default for BokatorRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BokatorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bokator")
    }

    fn explain(&self) -> String {
        format!(
            "【高棉拳击规则】\n\n\
            动物形态:\n{}\n\n\
            允许技法:\n{}\n\n\
            胜利条件:\n{}\n\n\
            传统仪式:\n{}\n",
            self.animal_forms()
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
            self.traditional_rituals()
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
    fn test_bokator_rules() {
        let rules = BokatorRules::new();
        assert_eq!(rules.metadata().name, "高棉拳击规则");
        assert!(!rules.animal_forms().is_empty());
    }

    #[test]
    fn test_bokator_animal_forms() {
        let rules = BokatorRules::new();
        let forms = rules.animal_forms();
        assert_eq!(forms.len(), 8);
        assert!(forms.iter().any(|f| f.contains("狮形")));
        assert!(forms.iter().any(|f| f.contains("象形")));
    }

    #[test]
    fn test_bokator_techniques() {
        let rules = BokatorRules::new();
        let techniques = rules.permitted_techniques();
        assert!(techniques.iter().any(|t| t.contains("肘击")));
        assert!(techniques.iter().any(|t| t.contains("膝击")));
    }
}