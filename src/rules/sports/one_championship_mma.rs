//! ONE Championship MMA规则
//!
//! ONE Championship是亚洲最大的综合格斗组织，采用独特的规则体系

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// ONE Championship MMA规则
pub struct OneChampionshipMmaRules {
    metadata: RuleMetadata,
}

impl OneChampionshipMmaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("ONE Championship MMA规则", "ONE Championship综合格斗竞赛规则")
                .with_origin("亚洲/新加坡")
                .with_tags(vec!["体育".into(), "格斗".into(), "MMA".into()]),
        }
    }

    /// 独特规则特点
    pub fn unique_characteristics(&self) -> Vec<&'static str> {
        vec![
            "禁止肘击: 区别于其他MMA",
            "禁止头部打击地面技术",
            "允许站立膝击",
            "开放重量级比赛",
            "超分量级挑战制度",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "非冠军赛: 3回合",
            "冠军赛: 5回合",
            "每回合5分钟",
            "回合间休息1分钟",
            "锦标赛可延长",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 各种拳击技术",
            "腿法: 各种踢击技术",
            "膝击: 仅站立状态允许",
            "摔法: 各种摔投技术",
            "地面控制: 擒拿和关节技",
            "地面拳法: 允许(非头部)",
            "缠抱攻击: 近身攻击",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "肘击: 完全禁止",
            "地面膝击: 禁止",
            "头部打击: 地面禁止",
            "头部踢击: 禁止",
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "撕扯头发",
            "手指插入",
            "咬人",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 56.7kg",
            "轻量级: 70.3kg",
            "次中量级: 77.1kg",
            "中量级: 93kg",
            "轻重量级: 102kg",
            "重量级: 120.2kg",
            "超重量级: 无限制",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "KO胜利: 击倒对手",
            "TKO胜利: 裁判终止",
            "投降胜利: 关节技或窒息",
            "判定胜利: 三裁判评分",
            "对手弃权",
            "对手被取消资格",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "有效打击: 优先评分",
            "摔法成功率",
            "地面控制时间",
            "主动攻击程度",
            "比赛掌控能力",
            "10-9评分制",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 4oz专用",
            "护齿: 必须佩戴",
            "护裆: 必须佩戴",
            "短裤: MMA专用",
            "无鞋比赛",
            "无护腿",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "圆形擂台: 直径9米",
            "围绳高度: 1.2米",
            "地面软垫: 厚度确认",
            "两个选手角落",
            "裁判区域",
        ]
    }

    /// 超分量级制度
    pub fn super_fight_rules(&self) -> Vec<&'static str> {
        vec![
            "允许跨级别挑战",
            "冠军腰带不因体重失去",
            "双冠军挑战制度",
            "超级赛事规则",
            "特殊规则可协商",
        ]
    }
}

impl Default for OneChampionshipMmaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OneChampionshipMmaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("one_championship_mma")
    }

    fn explain(&self) -> String {
        format!(
            "【ONE Championship MMA规则】\n\n\
            独特规则特点:\n{}\n\n\
            允许技法:\n{}\n\n\
            禁止技法:\n{}\n\n\
            重量级别:\n{}\n",
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
            self.prohibited_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weight_classes()
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
    fn test_one_championship_mma_rules() {
        let rules = OneChampionshipMmaRules::new();
        assert_eq!(rules.metadata().name, "ONE Championship MMA规则");
        assert!(!rules.unique_characteristics().is_empty());
    }

    #[test]
    fn test_one_championship_no_elbows() {
        let rules = OneChampionshipMmaRules::new();
        let prohibited = rules.prohibited_techniques();
        assert!(prohibited.iter().any(|t| t.contains("肘击")));
    }

    #[test]
    fn test_one_championship_weight_classes() {
        let rules = OneChampionshipMmaRules::new();
        let weights = rules.weight_classes();
        assert!(weights.iter().any(|w| w.contains("重量级")));
        assert!(weights.iter().any(|w| w.contains("超重量级")));
    }

    #[test]
    fn test_one_championship_rounds() {
        let rules = OneChampionshipMmaRules::new();
        let rounds = rules.round_system();
        assert!(rounds.iter().any(|r| r.contains("5回合")));
    }
}