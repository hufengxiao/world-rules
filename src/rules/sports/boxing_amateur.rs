//! 业余拳击规则
//!
//! 业余拳击遵循国际拳击协会规则，强调安全和运动精神

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 业余拳击规则
pub struct BoxingAmateurRules {
    metadata: RuleMetadata,
}

impl BoxingAmateurRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("业余拳击规则", "国际业余拳击联合会竞赛规则")
                .with_origin("国际拳击协会")
                .with_tags(vec!["体育".into(), "拳击".into(), "业余".into()]),
        }
    }

    /// 比赛回合
    pub fn rounds(&self) -> Vec<&'static str> {
        vec![
            "初级组: 3回合，每回合1.5分钟",
            "中级组: 3回合，每回合2分钟",
            "高级组: 3回合，每回合3分钟",
            "精英组: 3回合，每回合3分钟",
            "回合间休息1分钟",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "纸量级(46-49kg)",
            "蝇量级(49-52kg)",
            "雏量级(52-56kg)",
            "羽量级(56-60kg)",
            "轻量级(60-64kg)",
            "次中量级(64-69kg)",
            "中量级(69-75kg)",
            "轻重量级(75-81kg)",
            "重量级(81-91kg)",
            "超重量级(91kg以上)",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "头盔: 必须佩戴，保护头部",
            "拳套: 10oz红蓝两色",
            "护齿: 必须佩戴",
            "腹股沟护具: 可选但推荐",
            "拳击背心: 女子选手必须",
            "拳击短裤: 无遮挡设计",
            "拳击鞋: 软底运动鞋",
        ]
    }

    /// 得分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "电子计分系统",
            "五名裁判评分",
            "10分制评分",
            "有效击打区域: 头部和躯干正面",
            "清晰击打得1分",
            "多数判定原则",
        ]
    }

    /// 年龄分组
    pub fn age_categories(&self) -> Vec<&'static str> {
        vec![
            "儿童组(10-11岁)",
            "少年组(12-13岁)",
            "青年组(14-15岁)",
            "青少年组(16-17岁)",
            "精英组(18-40岁)",
            "大师组(41岁以上)",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打后脑或颈部",
            "击打背部",
            "击打腰带以下",
            "头撞对手",
            "肘击",
            "搂抱对手",
            "推搡或压住对手",
            "转身背对",
            "击打已倒地对手",
            "拖延比赛时间",
        ]
    }

    /// 医疗安全
    pub fn medical_safety(&self) -> Vec<&'static str> {
        vec![
            "赛前体检必做",
            "赛后医学检查",
            "脑震荡协议",
            "救护车待命",
            "医生可终止比赛",
            "禁止选手带伤参赛",
        ]
    }

    /// 裁判系统
    pub fn officiating(&self) -> Vec<&'static str> {
        vec![
            "台上裁判一名",
            "台下裁判五名",
            "比赛监督一名",
            "计时员和记分员",
            "医学监督",
            "视频回放系统",
        ]
    }
}

impl Default for BoxingAmateurRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingAmateurRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_amateur")
    }

    fn explain(&self) -> String {
        format!(
            "【业余拳击规则】\n\n\
            比赛回合:\n{}\n\n\
            重量级别:\n{}\n\n\
            护具要求:\n{}\n\n\
            得分系统:\n{}\n",
            self.rounds()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weight_classes()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
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
    fn test_boxing_amateur_rules() {
        let rules = BoxingAmateurRules::new();
        assert_eq!(rules.metadata().name, "业余拳击规则");
        assert!(!rules.rounds().is_empty());
        assert!(!rules.weight_classes().is_empty());
    }

    #[test]
    fn test_boxing_amateur_equipment() {
        let rules = BoxingAmateurRules::new();
        let equip = rules.equipment();
        assert!(equip.iter().any(|e| e.contains("头盔")));
        assert!(equip.iter().any(|e| e.contains("拳套")));
    }

    #[test]
    fn test_boxing_amateur_age_categories() {
        let rules = BoxingAmateurRules::new();
        assert!(!rules.age_categories().is_empty());
        assert!(rules.age_categories().iter().any(|a| a.contains("精英组")));
    }
}