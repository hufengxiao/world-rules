//! K-1踢拳规则
//!
//! 日本K-1踢拳规则，融合拳击和踢击的综合格斗规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// K-1踢拳规则
pub struct K1Rules {
    metadata: RuleMetadata,
}

impl K1Rules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("K-1踢拳规则", "日本K-1踢拳锦标赛规则")
                .with_origin("日本")
                .with_tags(vec!["体育".into(), "格斗".into(), "踢拳".into()]),
        }
    }

    /// K-1历史特点
    pub fn historical_characteristics(&self) -> Vec<&'static str> {
        vec![
            "创立于1993年",
            "融合拳击与踢击",
            "站立格斗主导",
            "日本格斗代表",
            "全球踢拳影响",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "标准比赛: 3回合",
            "决赛比赛: 5回合",
            "每回合3分钟",
            "回合间休息1分钟",
            "延长时间: 1回合决胜",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、勾拳",
            "腿法: 低扫、中扫、高扫",
            "膝击: 站立状态允许",
            "缠抱膝击: 限时攻击",
            "摔法防御: 防摔技术",
            "组合攻击",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "肘击: 完全禁止",
            "地面攻击",
            "摔法攻击",
            "关节技",
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "缠抱超过限制",
            "推人出擂台",
        ]
    }

    /// 缠抱规则
    pub fn clinching_rules(&self) -> Vec<&'static str> {
        vec![
            "允许单手缠抱",
            "缠抱时间限制: 5秒",
            "膝击后必须分开",
            "裁判主动分开",
            "双手缠抱警告",
            "持续缠抱扣分",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 57.5kg以下",
            "羽量级: 57.5-60kg",
            "轻量级: 60-62.5kg",
            "次中量级: 62.5-67kg",
            "中量级: 67-70kg",
            "重量级: 70-85kg",
            "超重量级: 85kg以上",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "KO胜利: 完全击倒",
            "TKO胜利: 对手无法继续",
            "裁判终止比赛",
            "判定胜利: 评分获胜",
            "对手弃权",
            "对手被取消资格",
            "三次读秒判定负",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "打击效果: 优先评分",
            "有效打击数量",
            "腿法成功率",
            "摔法防御",
            "主动攻击程度",
            "比赛控制能力",
            "10-10评分制",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 10oz标准",
            "护齿: 必须佩戴",
            "护裆: 必须佩戴",
            "短裤: K-1专用",
            "无护腿",
            "无头盔",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "标准擂台: 7米×7米",
            "围绳高度: 1.2米",
            "地面软垫: 确认厚度",
            "两个选手角落",
            "裁判区域",
        ]
    }

    /// 读秒规则
    pub fn knockdown_rules(&self) -> Vec<&'static str> {
        vec![
            "击倒读秒: 10秒",
            "站立恢复判断",
            "三次击倒判定负",
            "同一回合两次击倒判定负",
            "裁判保护性终止",
            "医疗检查确认",
        ]
    }
}

impl Default for K1Rules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for K1Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("k1")
    }

    fn explain(&self) -> String {
        format!(
            "【K-1踢拳规则】\n\n\
            历史特点:\n{}\n\n\
            允许技法:\n{}\n\n\
            禁止技法:\n{}\n\n\
            缠抱规则:\n{}\n",
            self.historical_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prohibited_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.clinching_rules()
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
    fn test_k1_rules() {
        let rules = K1Rules::new();
        assert_eq!(rules.metadata().name, "K-1踢拳规则");
        assert!(!rules.historical_characteristics().is_empty());
    }

    #[test]
    fn test_k1_no_elbows() {
        let rules = K1Rules::new();
        let prohibited = rules.prohibited_techniques();
        assert!(prohibited.iter().any(|t| t.contains("肘击")));
    }

    #[test]
    fn test_k1_clinching_time() {
        let rules = K1Rules::new();
        let clinching = rules.clinching_rules();
        assert!(clinching.iter().any(|c| c.contains("5秒")));
    }

    #[test]
    fn test_k1_rounds() {
        let rules = K1Rules::new();
        let rounds = rules.round_system();
        assert!(rounds.iter().any(|r| r.contains("3回合")));
        assert!(rounds.iter().any(|r| r.contains("3分钟")));
    }
}
