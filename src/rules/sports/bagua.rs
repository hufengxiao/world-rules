//! 八卦掌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 八卦掌规则
pub struct BaguaRules {
    metadata: RuleMetadata,
}

impl BaguaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("八卦掌规则", "八卦掌比赛与训练基本规则")
                .with_origin("中国")
                .with_tags(vec!["体育".into(), "武术".into(), "内家拳".into()]),
        }
    }

    /// 八卦掌特点
    pub fn characteristics(&self) -> Vec<&'static str> {
        vec![
            "走圈: 沿圈行步",
            "转身: 旋转变换",
            "连环: 连续不断",
            "变化: 灵活多变",
            "避正击斜: 不正面交锋",
        ]
    }

    /// 八卦掌套路
    pub fn forms(&self) -> Vec<&'static str> {
        vec![
            "老八掌: 基础八掌",
            "新八掌: 进阶套路",
            "六十四掌: 变化掌法",
            "八卦剑: 器械套路",
            "八卦刀: 刀术套路",
        ]
    }

    /// 八种掌法
    pub fn eight_palms(&self) -> Vec<&'static str> {
        vec![
            "乾卦狮形掌",
            "坤卦麒麟掌",
            "坎卦蛇形掌",
            "离卦鹰形掌",
            "震卦龙形掌",
            "艮卦熊形掌",
            "巽卦凤形掌",
            "兑卦猴形掌",
        ]
    }

    /// 步法要求
    pub fn stepping_methods(&self) -> Vec<&'static str> {
        vec![
            "趟泥步: 低身行步",
            "扣步: 转身扣脚",
            "摆步: 摆脚转向",
            "鹤行步: 仿鹤行走",
            "蛇行步: 仿蛇游走",
        ]
    }

    /// 技法原则
    pub fn technique_principles(&self) -> Vec<&'static str> {
        vec![
            "以动为本: 动中求静",
            "以变为法: 变中求胜",
            "以转为用: 转中化力",
            "以柔克刚: 柔中带刚",
            "以圆破直: 圆中化解",
        ]
    }

    /// 训练方法
    pub fn training_methods(&self) -> Vec<&'static str> {
        vec![
            "走圈练习: 基础训练",
            "单式练习: 分解训练",
            "套路练习: 整体训练",
            "推手练习: 应用训练",
            "器械练习: 兵器训练",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 动作评分",
            "推手比赛: 技术对抗",
            "表演评分: 规范性",
            "时间限制: 套路时限",
            "裁判评分制",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "训练场地: 平坦开阔",
            "比赛护具: 必备护具",
            "医疗支持: 赛场医疗",
            "循序渐进: 逐步提高",
            "禁止危险动作",
        ]
    }
}

impl Default for BaguaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BaguaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bagua")
    }

    fn explain(&self) -> String {
        format!(
            "【八卦掌规则】\n\n\
            八卦掌特点:\n{}\n\n\
            八种掌法:\n{}\n\n\
            步法要求:\n{}\n\n\
            技法原则:\n{}\n\n\
            安全规则:\n{}\n",
            self.characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.eight_palms()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stepping_methods()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique_principles()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
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
    fn test_bagua_rules() {
        let rules = BaguaRules::new();
        assert!(!rules.characteristics().is_empty());
        assert!(!rules.eight_palms().is_empty());
        assert_eq!(rules.eight_palms().len(), 8);
    }

    #[test]
    fn test_bagua_forms() {
        let rules = BaguaRules::new();
        let forms = rules.forms();
        assert!(forms.contains(&"老八掌: 基础八掌"));
        assert!(forms.contains(&"八卦剑: 器械套路"));
    }
}