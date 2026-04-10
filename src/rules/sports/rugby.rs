//! 橄榄球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 橄榄球规则
pub struct RugbyRules {
    metadata: RuleMetadata,
}

impl RugbyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "橄榄球规则",
                "橄榄球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 比赛形式
    pub fn match_formats(&self) -> Vec<&'static str> {
        vec![
            "十五人制: 标准比赛，每队15人",
            "七人制: 缩短版比赛，每队7人",
            "比赛时间: 15人制80分钟(上下半场各40分钟)",
            "七人制: 14分钟(上下半场各7分钟)",
            "中场休息: 10-15分钟",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "达阵: 5分，球在对方端区触地",
            "追加射门: 2分，达阵后射门",
            "罚踢射门: 3分，犯规后射门",
            "落踢射门: 3分，比赛中落踢射门",
            "达阵得分后由得分方开球",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "只能向后传球",
            "不能向前传球",
            "可以用脚向前踢球",
            "持球者被擒抱后必须释放球",
            "争球时不能越位",
            "不能阻挡对方无球队员",
        ]
    }

    /// 擒抱规则
    pub fn tackle_rules(&self) -> Vec<&'static str> {
        vec![
            "擒抱: 阻止持球者前进",
            "擒抱必须在肩部以下",
            "不能擒抱颈部或头部",
            "被擒抱者必须立即释放球",
            "不能从空中擒抱接球者",
        ]
    }

    /// 争球规则
    pub fn scrum_rules(&self) -> Vec<&'static str> {
        vec![
            "争球: 前锋8人顶推对抗",
            "由非犯规方投球",
            "投球进入通道",
            "双方争夺控球权",
            "不得过早推搡",
        ]
    }

    /// 边线球规则
    pub fn lineout_rules(&self) -> Vec<&'static str> {
        vec![
            "边线球: 球出界后重新开始",
            "两队前锋排成两行",
            "投球者将球投入场内",
            "队员可被举起争球",
            "不能干扰对方接球",
        ]
    }

    /// 犯规与处罚
    pub fn fouls_penalties(&self) -> Vec<&'static str> {
        vec![
            "越位: 罚踢",
            "向前传球: 争球",
            "高空擒抱: 黄牌/红牌",
            "故意犯规: 罚踢或黄牌",
            "暴力行为: 红牌驱逐",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "球衣短裤: 队服",
            "护齿: 必须佩戴",
            "护肩: 可选保护装备",
            "橄榄球鞋: 钉鞋",
            "不得佩戴首饰",
        ]
    }
}

impl Default for RugbyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RugbyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rugby")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【橄榄球规则】\n\n\
            比赛形式:\n{}\n\n\
            得分规则:\n{}\n\n\
            基本规则:\n{}\n\n\
            犯规与处罚:\n{}\n",
            self.match_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls_penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rugby_rules() {
        let rules = RugbyRules::new();
        assert!(!rules.match_formats().is_empty());
    }
}