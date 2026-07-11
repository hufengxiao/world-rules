//! 昆斯伯里拳击规则
//!
//! 1867年制定的现代拳击规则基础，由约翰·格拉汉姆·钱伯斯创立

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 昆斯伯里拳击规则
pub struct BoxingQueensberryRules {
    metadata: RuleMetadata,
}

impl BoxingQueensberryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("昆斯伯里拳击规则", "现代拳击规则基础，1867年制定")
                .with_origin("英国")
                .with_tags(vec!["体育".into(), "拳击".into(), "历史".into()]),
        }
    }

    /// 核心规则
    pub fn core_rules(&self) -> Vec<&'static str> {
        vec![
            "禁止摔跤或搂抱",
            "禁止击打已倒地对手",
            "回合持续3分钟",
            "回合间休息1分钟",
            "被击倒后10秒内须站起",
            "比赛在围绳擂台内进行",
        ]
    }

    /// 比赛场地
    pub fn ring_requirements(&self) -> Vec<&'static str> {
        vec![
            "标准擂台: 24英尺(7.3米)见方",
            "最小擂台: 16英尺(4.9米)见方",
            "围绳高度: 4条绳索",
            "擂台地面: 弹性垫层",
            "四个中立角",
            "两个选手角落",
        ]
    }

    /// 拳套要求
    pub fn glove_requirements(&self) -> Vec<&'static str> {
        vec![
            "公平拳套原则",
            "双方拳套重量相同",
            "拳套须经裁判检查",
            "拳套需有足够填充",
            "拇指需固定在拳套内",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "重量级: 154磅以上",
            "中量级: 112-154磅",
            "轻量级: 112磅以下",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "摔跤或搂抱",
            "击打已倒地对手",
            "头撞",
            "击打腰部以下",
            "击打后脑",
            "用拳套边缘击打",
            "推搡对手",
            "吐出护齿",
        ]
    }

    /// 胜利方式
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "击倒胜利(KO)",
            "对手弃权",
            "裁判终止比赛",
            "对手被取消资格",
            "多数判定胜利",
        ]
    }

    /// 裁判职责
    pub fn referee_duties(&self) -> Vec<&'static str> {
        vec![
            "检查选手拳套",
            "确保公平竞争",
            "执行规则",
            "数秒判定击倒",
            "警告犯规行为",
            "终止比赛保护选手",
        ]
    }

    /// 历史意义
    pub fn historical_significance(&self) -> Vec<&'static str> {
        vec![
            "开创现代拳击规则先河",
            "引入拳套制度",
            "确立回合制度",
            "制定重量级别",
            "建立裁判制度",
            "奠定职业拳击基础",
        ]
    }

    /// 选手要求
    pub fn fighter_requirements(&self) -> Vec<&'static str> {
        vec![
            "佩戴拳击拳套",
            "穿着适当比赛服装",
            "禁止使用非法物质",
            "遵守比赛规则",
            "尊重裁判决定",
        ]
    }
}

impl Default for BoxingQueensberryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingQueensberryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_queensberry")
    }

    fn explain(&self) -> String {
        format!(
            "【昆斯伯里拳击规则】\n\n\
            核心规则:\n{}\n\n\
            比赛场地:\n{}\n\n\
            拳套要求:\n{}\n\n\
            禁止行为:\n{}\n",
            self.core_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ring_requirements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.glove_requirements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fouls()
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
    fn test_boxing_queensberry_rules() {
        let rules = BoxingQueensberryRules::new();
        assert_eq!(rules.metadata().name, "昆斯伯里拳击规则");
        assert!(!rules.core_rules().is_empty());
        assert!(!rules.historical_significance().is_empty());
    }

    #[test]
    fn test_boxing_queensberry_core() {
        let rules = BoxingQueensberryRules::new();
        let core = rules.core_rules();
        assert!(core.contains(&"禁止摔跤或搂抱"));
        assert!(core.contains(&"回合持续3分钟"));
    }

    #[test]
    fn test_boxing_queensberry_ring() {
        let rules = BoxingQueensberryRules::new();
        assert!(!rules.ring_requirements().is_empty());
        assert!(rules.ring_requirements().iter().any(|r| r.contains("围绳")));
    }
}