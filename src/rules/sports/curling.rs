//! 冰壶规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冰壶规则
pub struct CurlingRules {
    metadata: RuleMetadata,
}

impl CurlingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冰壶规则",
                "冰壶比赛基本规则"
            )
            .with_origin("苏格兰")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 场地规格
    pub fn rink_specifications(&self) -> Vec<&'static str> {
        vec![
            "冰道长度: 44.5米",
            "冰道宽度: 4.3-5米",
            "大本营: 直径3.66米",
            "圆心: 直径0.3米",
            "投掷线: 虎线",
        ]
    }

    /// 冰壶规格
    pub fn stone_specifications(&self) -> Vec<&'static str> {
        vec![
            "重量: 19.96公斤",
            "直径: 约30厘米",
            "材质: 花岗岩",
            "把手: 投掷控制",
            "底面: 凹面设计",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4人",
            "主将: 最后投壶",
            "副将: 第二投壶",
            "第一队员: 第一投壶",
            "第二队员: 第三投壶",
        ]
    }

    /// 比赛形式
    pub fn game_format(&self) -> Vec<&'static str> {
        vec![
            "每场比赛10局",
            "每队每局投8壶",
            "每壶由一人投掷",
            "交替投壶",
            "额外局决胜",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "大本营内冰壶计分",
            "靠近圆心壶得分",
            "每壶1分",
            "只有一队得分",
            "得分队下一局先投",
        ]
    }

    /// 投掷规则
    pub fn delivery_rules(&self) -> Vec<&'static str> {
        vec![
            "必须从投掷区开始",
            "滑行投掷",
            "释放壶前虎线",
            "壶越过比赛线",
            "违规壶移除",
        ]
    }

    /// 擦冰规则
    pub fn sweeping_rules(&self) -> Vec<&'static str> {
        vec![
            "擦冰减少摩擦",
            "延长滑行距离",
            "改变行进方向",
            "只能擦己方壶",
            "对方壶过虎线可擦",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "故意破坏冰面",
            "干扰对方壶",
            "违规擦冰",
            "装备违规",
            "不当行为",
        ]
    }
}

impl Default for CurlingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CurlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("curling")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冰壶规则】\n\n\
            场地规格:\n{}\n\n\
            计分规则:\n{}\n\n\
            投掷规则:\n{}\n\n\
            擦冰规则:\n{}\n",
            self.rink_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.delivery_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.sweeping_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curling_rules() {
        let rules = CurlingRules::new();
        assert!(!rules.rink_specifications().is_empty());
    }
}