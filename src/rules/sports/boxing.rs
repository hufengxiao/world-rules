//! 拳击规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 拳击规则
pub struct BoxingRules {
    metadata: RuleMetadata,
}

impl BoxingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "拳击规则",
                "拳击比赛基本规则"
            )
            .with_origin("国际拳击协会")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛分级
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 52kg以下",
            "羽量级: 52-57kg",
            "轻量级: 57-63kg",
            "中量级: 63-69kg",
            "次中量级: 69-75kg",
            "重量级: 75-91kg",
            "超重量级: 91kg以上",
        ]
    }

    /// 比赛回合
    pub fn rounds(&self) -> Vec<&'static str> {
        vec![
            "职业比赛: 4-12回合",
            "每回合3分钟",
            "回合间休息1分钟",
            "业余比赛: 3回合，每回合3分钟",
            "奥运会: 3回合，每回合3分钟",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效击打: 击中头部或躯干正面",
            "10分制: 每回合胜者10分，败者9分或更低",
            "击倒: 对手被击倒可得额外分数",
            "五名裁判评分",
            "多数判定原则",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打后脑",
            "击打背部",
            "击打腰部以下",
            "头撞",
            "肘击",
            "推搡",
            "搂抱",
            "击打倒地对手",
            "拖延时间",
            "用拳套边缘击打",
        ]
    }

    /// 裁判信号
    pub fn referee_signals(&self) -> Vec<&'static str> {
        vec![
            "停止: 双手分开",
            "开始: 双手合拢",
            "击倒: 开始数秒",
            "犯规: 指示犯规行为",
            "警告: 向拳手发出警告",
            "取消资格: 终止比赛",
        ]
    }

    /// 击倒规则
    pub fn knockout_rules(&self) -> Vec<&'static str> {
        vec![
            "被击倒后裁判开始数秒(1-10)",
            "数秒期间对手必须在指定区域",
            "10秒内未站起则判定KO失败",
            "站起后裁判评估能否继续",
            "连续被击倒三次判定TKO",
            "被击倒后本轮结束前不得再击倒",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 职业10oz，业余更大",
            "护齿: 必须佩戴",
            "短裤: 无遮挡设计",
            "业余比赛需佩戴头盔",
            "鞋子: 轻便运动鞋",
        ]
    }
}

impl Default for BoxingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【拳击规则】\n\n\
            重量级别:\n{}\n\n\
            比赛回合:\n{}\n\n\
            禁止行为:\n{}\n\n\
            击倒规则:\n{}\n",
            self.weight_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rounds().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.knockout_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxing_rules() {
        let rules = BoxingRules::new();
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.fouls().is_empty());
    }
}