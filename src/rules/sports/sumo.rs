//! 相扑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 相扑规则
pub struct SumoRules {
    metadata: RuleMetadata,
}

impl SumoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "相扑规则",
                "相扑比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛场地
    pub fn ring_specifications(&self) -> Vec<&'static str> {
        vec![
            "土俵直径: 4.55米",
            "土俵高度: 34-60厘米",
            "胜负草席",
            "屋形: 悬挂屋顶",
            "神道教仪式场地",
        ]
    }

    /// 力士等级
    pub fn rankings(&self) -> Vec<&'static str> {
        vec![
            "横纲: 最高等级",
            "大关: 第二高等级",
            "关胁",
            "小结",
            "前头",
            "十两",
            "幕下",
        ]
    }

    /// 比赛规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "身体任何部位触地即输",
            "出界即输",
            "犯规动作判定",
            "比赛时间通常很短",
            "胜负判定",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "推掌: 推击对手",
            "拉手: 拉倒对手",
            "投技: 摔倒对手",
            "绊腿: 绊倒对手",
            "侧身: 闪避技术",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "握拳殴打",
            "抓头发",
            "挖眼睛",
            "攻击下身",
            "掐脖子",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "回: 缠腰布",
            "装饰: 银杏叶发型",
            "无鞋赤足",
            "入场仪式服装",
            "等级装饰",
        ]
    }

    /// 赛事安排
    pub fn tournament_schedule(&self) -> Vec<&'static str> {
        vec![
            "每年6大比赛",
            "每场比赛15天",
            "每天一场比赛",
            "胜多者晋级",
            "优胜者奖励",
        ]
    }

    /// 传统仪式
    pub fn traditions(&self) -> Vec<&'static str> {
        vec![
            "入场仪式",
            "撒盐净化",
            "拍手召唤神灵",
            "脚踏驱邪",
            "比赛前行礼",
        ]
    }
}

impl Default for SumoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SumoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sumo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【相扑规则】\n\n\
            比赛场地:\n{}\n\n\
            力士等级:\n{}\n\n\
            比赛规则:\n{}\n\n\
            禁止动作:\n{}\n",
            self.ring_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rankings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.match_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumo_rules() {
        let rules = SumoRules::new();
        assert!(!rules.ring_specifications().is_empty());
    }
}