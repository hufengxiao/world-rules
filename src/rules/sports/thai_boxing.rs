//! 泰拳拳击规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 泰拳拳击规则
pub struct ThaiBoxingRules {
    metadata: RuleMetadata,
}

impl ThaiBoxingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "泰拳拳击规则",
                "泰拳拳击比赛基本规则"
            )
            .with_origin("泰国")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 5回合",
            "每回合3分钟",
            "回合间休息2分钟",
            "积分制评分",
            "KO获胜",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、勾拳",
            "腿法: 扫踢、直踢",
            "肘法: 横肘、竖肘",
            "膝法: 直膝、斜膝",
            "摔法: 接腿摔",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击眼睛",
            "攻击后脑",
            "咬人抓挠",
            "攻击倒地选手",
            "违规摔法",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效打击",
            "清晰有力加分",
            "技术多样性",
            "场上控制",
            "进攻主动性",
        ]
    }

    /// 比赛场地
    pub fn ring_specifications(&self) -> Vec<&'static str> {
        vec![
            "擂台尺寸: 6.1-7.3米正方形",
            "擂台高度: 1.2米",
            "围绳4条",
            "地面垫子帆布",
            "红蓝角落",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 专业比赛10盎司",
            "护齿必须",
            "泰拳短裤",
            "护腿可选",
            "护胸可选",
        ]
    }

    /// 传统仪式
    pub fn traditions(&self) -> Vec<&'static str> {
        vec![
            "赛前拜师舞",
            "蒙空头环",
            "入场仪式",
            "音乐伴奏",
            "祈祷仪式",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前体检",
            "裁判监督",
            "医疗支持",
            "比赛中止规则",
            "选手保护",
        ]
    }
}

impl Default for ThaiBoxingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ThaiBoxingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("thai_boxing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【泰拳拳击规则】\n\n\
            允许技术:\n{}\n\n\
            禁止动作:\n{}\n\n\
            评分标准:\n{}\n\n\
            安全规则:\n{}\n",
            self.permitted_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thai_boxing_rules() {
        let rules = ThaiBoxingRules::new();
        assert!(!rules.permitted_techniques().is_empty());
    }
}