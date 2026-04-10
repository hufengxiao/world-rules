//! 泰拳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 泰拳规则
pub struct MuayThaiRules {
    metadata: RuleMetadata,
}

impl MuayThaiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "泰拳规则",
                "泰拳比赛基本规则"
            )
            .with_origin("泰国")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛场地
    pub fn ring_specifications(&self) -> Vec<&'static str> {
        vec![
            "擂台尺寸: 6.1-7.3米正方形",
            "擂台高度: 1.2米",
            "围绳: 4条，间隔0.5米",
            "地面: 垫子和帆布",
            "两个角落配有红蓝两色",
        ]
    }

    /// 八肢技术
    pub fn eight_limbs(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、勾拳、摆拳",
            "腿法: 扫踢、直踢、侧踢",
            "膝法: 直膝、斜膝、飞膝",
            "肘法: 横肘、竖肘、旋肘",
            "双拳、双腿、双膝、双肘共八肢",
        ]
    }

    /// 比赛回合
    pub fn rounds(&self) -> Vec<&'static str> {
        vec![
            "专业比赛: 5回合",
            "每回合3分钟",
            "回合间休息2分钟",
            "业余比赛: 3回合",
            "可根据规则调整",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效打击得分",
            "清晰有力的技术加分",
            "技术多样性加分",
            "场上控制能力",
            "进攻主动性加分",
        ]
    }

    /// 有效打击
    pub fn valid_strikes(&self) -> Vec<&'static str> {
        vec![
            "拳打头部和身体",
            "踢击腿部和身体",
            "膝击身体",
            "肘击头部",
            "必须清晰有力",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击眼睛",
            "攻击后脑",
            "咬人",
            "抓挠",
            "攻击倒地选手",
            "侮辱对手",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 专业比赛10盎司",
            "护齿必须佩戴",
            "短裤: 泰拳专用短裤",
            "护腿: 业余比赛必须",
            "护胸: 业余比赛必须",
        ]
    }

    /// 传统仪式
    pub fn traditions(&self) -> Vec<&'static str> {
        vec![
            "赛前拜师舞",
            "蒙空头环佩戴",
            "入场仪式",
            "音乐伴奏",
            "祈祷仪式",
        ]
    }
}

impl Default for MuayThaiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MuayThaiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("muay_thai")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【泰拳规则】\n\n\
            八肢技术:\n{}\n\n\
            比赛回合:\n{}\n\n\
            得分标准:\n{}\n\n\
            禁止行为:\n{}\n",
            self.eight_limbs().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rounds().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muay_thai_rules() {
        let rules = MuayThaiRules::new();
        assert!(!rules.eight_limbs().is_empty());
    }
}