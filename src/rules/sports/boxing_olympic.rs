//! 奥运会拳击规则
//!
//! 奥运会拳击遵循国际拳击协会(IBA)规则，采用业余拳击制度

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 奥运会拳击规则
pub struct BoxingOlympicRules {
    metadata: RuleMetadata,
}

impl BoxingOlympicRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会拳击规则", "国际拳击协会(原AIBA)奥运会拳击竞赛规则")
                .with_origin("国际奥委会")
                .with_tags(vec!["体育".into(), "拳击".into(), "奥运会".into()]),
        }
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级(52kg): 男子轻量级别",
            "羽量级(57kg): 男子中量级别",
            "轻量级(63kg): 男子级别",
            "次中量级(69kg): 男子级别",
            "中量级(75kg): 男子级别",
            "轻重量级(81kg): 男子级别",
            "重量级(91kg): 男子级别",
            "超重量级(+91kg): 男子最高级别",
        ]
    }

    /// 女子重量级别
    pub fn women_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级(50kg): 女子最轻级别",
            "羽量级(54kg): 女子轻量级别",
            "轻量级(57kg): 女子级别",
            "次中量级(60kg): 女子级别",
            "中量级(66kg): 女子级别",
            "重量级(75kg): 女子最高级别",
        ]
    }

    /// 回合制度
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "男子: 3回合，每回合3分钟",
            "女子: 4回合，每回合2分钟",
            "回合间休息1分钟",
            "裁判可随时终止比赛",
            "医学暂停允许",
        ]
    }

    /// 得分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "五名裁判评分",
            "10分制评分系统",
            "有效击打: 击中头部或躯干正面",
            "使用电子计分系统",
            "多数判定原则确定胜者",
            "技术得分击打区域: 头部、躯干正面",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "必戴头盔: 保护头部(2016年后男子取消)",
            "拳套: 10oz标准",
            "护齿: 必须佩戴",
            "拳击背心: 女子选手必须",
            "腹股沟护具: 可选",
            "无鞋比赛: 自2020年东京奥运会",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打后脑或颈部",
            "击打背部",
            "击打腰带以下",
            "头撞",
            "肘击",
            "搂抱对手",
            "推搡",
            "转身背对对手",
            "击打已倒地对手",
            "吐出护齿拖延时间",
        ]
    }

    /// 裁判职责
    pub fn referee_duties(&self) -> Vec<&'static str> {
        vec![
            "检查选手护具",
            "开始和结束比赛",
            "执行犯规处罚",
            "警告和扣分",
            "判断选手能否继续",
            "数秒判定KO",
            "终止比赛保护选手",
        ]
    }

    /// 胜利方式
    pub fn victory_methods(&self) -> Vec<&'static str> {
        vec![
            "判定胜利: 多数裁判评分胜",
            "KO胜利: 对手10秒内无法站起",
            "TKO胜利: 裁判终止比赛",
            "弃权胜利: 对手退出比赛",
            "取消资格胜利: 对手严重犯规",
            "RSC胜利: 裁判停止比赛",
        ]
    }

    /// 资格赛制
    pub fn qualification(&self) -> Vec<&'static str> {
        vec![
            "洲际预选赛",
            "世界预选赛",
            "外卡名额",
            "主办国名额",
            "青年世界锦标赛名额",
        ]
    }
}

impl Default for BoxingOlympicRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_olympic")
    }

    fn explain(&self) -> String {
        format!(
            "【奥运会拳击规则】\n\n\
            重量级别:\n{}\n\n\
            回合制度:\n{}\n\n\
            得分系统:\n{}\n\n\
            护具要求:\n{}\n\n\
            禁止行为:\n{}\n",
            self.weight_classes()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.round_system()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
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
    fn test_boxing_olympic_rules() {
        let rules = BoxingOlympicRules::new();
        assert_eq!(rules.metadata().name, "奥运会拳击规则");
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.women_weight_classes().is_empty());
    }

    #[test]
    fn test_boxing_olympic_scoring() {
        let rules = BoxingOlympicRules::new();
        assert!(!rules.scoring_system().is_empty());
        assert!(rules.scoring_system().contains(&"五名裁判评分"));
    }

    #[test]
    fn test_boxing_olympic_equipment() {
        let rules = BoxingOlympicRules::new();
        assert!(!rules.equipment().is_empty());
        assert!(rules.equipment().iter().any(|e| e.contains("拳套")));
    }
}