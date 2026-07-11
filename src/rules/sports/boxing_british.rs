//! 英国拳击规则
//!
//! 英国拳击理事会(BBBoC)管理的职业拳击规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 英国拳击规则
pub struct BoxingBritishRules {
    metadata: RuleMetadata,
}

impl BoxingBritishRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英国拳击规则", "英国拳击理事会(BBBoC)职业拳击规则")
                .with_origin("英国")
                .with_tags(vec!["体育".into(), "拳击".into(), "职业拳击".into()]),
        }
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "迷你蝇量级(105磅)",
            "轻蝇量级(108磅)",
            "蝇量级(112磅)",
            "超蝇量级(115磅)",
            "雏量级(118磅)",
            "超雏量级(122磅)",
            "羽量级(126磅)",
            "超羽量级(130磅)",
            "轻量级(135磅)",
            "超轻量级(140磅)",
            "次中量级(147磅)",
            "超次中量级(154磅)",
            "中量级(160磅)",
            "超中量级(168磅)",
            "轻重量级(175磅)",
            "次重量级(200磅)",
            "重量级(200磅以上)",
        ]
    }

    /// 称重规则
    pub fn weigh_in_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前24-30小时称重",
            "官方称重一次机会",
            "超重可协商二次称重",
            "超重罚款按合同比例",
            "英国头衔赛严格限制",
            "称重后需补充水分",
        ]
    }

    /// 裁判制度
    pub fn officiating(&self) -> Vec<&'static str> {
        vec![
            "英国注册裁判",
            "三名台下裁判",
            "一名台上裁判",
            "比赛监督",
            "计时员和记分员",
            "医疗团队待命",
        ]
    }

    /// 许可制度
    pub fn licensing(&self) -> Vec<&'static str> {
        vec![
            "BBBoC拳手执照",
            "年度体检",
            "脑部扫描(MRI)",
            "眼科检查",
            "血液检测",
            "保险覆盖",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打后脑",
            "击打下体",
            "头撞",
            "肘击",
            "咬人",
            "搂抱过久",
            "推搡",
            "击打已倒地对手",
        ]
    }

    /// 安全规定
    pub fn safety_regulations(&self) -> Vec<&'static str> {
        vec![
            "赛前医学检查",
            "赛后医学观察",
            "脑震荡恢复期规定",
            "拳击执照暂停制度",
            "禁赛记录追踪",
            "医疗中止权",
        ]
    }

    /// 胜利判定
    pub fn victory_methods(&self) -> Vec<&'static str> {
        vec![
            "一致判定(UD)",
            "分歧判定(SD)",
            "多数判定(MD)",
            "技术判定(TD)",
            "KO胜利",
            "TKO胜利",
            "弃权(RTD)",
            "取消资格(DQ)",
        ]
    }

    /// 回合制度
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "冠军赛: 12回合",
            "非冠军赛: 4-10回合",
            "每回合3分钟",
            "回合间休息1分钟",
            "技术平局判定",
        ]
    }
}

impl Default for BoxingBritishRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingBritishRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_british")
    }

    fn explain(&self) -> String {
        format!(
            "【英国拳击规则】\n\n\
            重量级别:\n{}\n\n\
            称重规则:\n{}\n\n\
            许可制度:\n{}\n\n\
            安全规定:\n{}\n",
            self.weight_classes()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weigh_in_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.licensing()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_regulations()
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
    fn test_boxing_british_rules() {
        let rules = BoxingBritishRules::new();
        assert_eq!(rules.metadata().name, "英国拳击规则");
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.licensing().is_empty());
    }

    #[test]
    fn test_boxing_british_weigh_in() {
        let rules = BoxingBritishRules::new();
        assert!(!rules.weigh_in_rules().is_empty());
        assert!(rules.weigh_in_rules().iter().any(|w| w.contains("24")));
    }

    #[test]
    fn test_boxing_british_safety() {
        let rules = BoxingBritishRules::new();
        let safety = rules.safety_regulations();
        assert!(safety.iter().any(|s| s.contains("脑震荡")));
    }
}
