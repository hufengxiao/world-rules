//! 菲律宾短棍术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 菲律宾短棍术规则
pub struct EscrimaRules {
    metadata: RuleMetadata,
}

impl EscrimaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("菲律宾短棍术规则", "菲律宾短棍术比赛基本规则")
                .with_origin("菲律宾")
                .with_tags(vec!["体育".into(), "武术".into(), "兵器术".into()]),
        }
    }

    /// 流派分类
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "Arnis: 马尼拉风格",
            "Escrima: 维萨亚风格",
            "Kali: 南部风格",
            "现代Arnis: 现代化流派",
            "Doce Pares: 十二对流派",
        ]
    }

    /// 武器类型
    pub fn weapons(&self) -> Vec<&'static str> {
        vec![
            "单棍: 单根短棍",
            "双棍: 两根短棍",
            "长棍: 长棍武器",
            "刀剑: 刀剑技术",
            "空手: 无武器技法",
        ]
    }

    /// 基本技术
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "十二攻击角度: 基础攻击路线",
            "防守格挡: 防御技术",
            "反击技术: 反击动作",
            "缴械技术: 夺取武器",
            "锁技控制: 控制技术",
        ]
    }

    /// 比赛形式
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 技术表演",
            "对抗比赛: 实战比赛",
            "双棍比赛: 双棍对抗",
            "单棍比赛: 单棍对抗",
            "全接触比赛: 实战风格",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "有效打击: 指定部位击中",
            "缴械得分: 成功缴械",
            "控制得分: 成功控制",
            "套路评分: 动作规范",
            "判定得分: 比赛评分",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "正赛: 3分钟",
            "决赛: 5分钟",
            "休息时间: 局间1分钟",
            "加时赛: 平局进行",
            "套路时间: 限定时长",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 头盔护身",
            "武器钝化: 安全武器",
            "医疗支持: 赛场医生",
            "裁判监督: 安全保障",
            "禁止危险动作",
        ]
    }

    /// 服装要求
    pub fn uniform_requirements(&self) -> Vec<&'static str> {
        vec![
            "训练服: 传统服装",
            "护具: 比赛必备",
            "武器规格: 标准尺寸",
            "颜色要求: 统一颜色",
            "整洁规定: 服装整洁",
        ]
    }

    /// 段位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初级级别: 基础技术",
            "中级级别: 进阶掌握",
            "高级级别: 高级技巧",
            "教练级别: 教学资格",
            "大师级别: 最高等级",
        ]
    }
}

impl Default for EscrimaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EscrimaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("escrima")
    }

    fn explain(&self) -> String {
        format!(
            "【菲律宾短棍术规则】\n\n\
            流派分类:\n{}\n\n\
            武器类型:\n{}\n\n\
            基本技术:\n{}\n\n\
            比赛形式:\n{}\n\n\
            安全规则:\n{}\n",
            self.styles()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weapons()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.basic_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_types()
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
    fn test_escrima_rules() {
        let rules = EscrimaRules::new();
        assert!(!rules.styles().is_empty());
        assert!(!rules.weapons().is_empty());
        assert!(!rules.basic_techniques().is_empty());
    }

    #[test]
    fn test_escrima_weapons() {
        let rules = EscrimaRules::new();
        let weapons = rules.weapons();
        assert!(weapons.contains(&"单棍: 单根短棍"));
        assert!(weapons.contains(&"双棍: 两根短棍"));
    }
}