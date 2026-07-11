//! 马来传统武术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 马来传统武术规则
pub struct SilatMelayuRules {
    metadata: RuleMetadata,
}

impl SilatMelayuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("马来传统武术规则", "马来传统武术比赛基本规则")
                .with_origin("马来西亚")
                .with_tags(vec!["体育".into(), "武术".into(), "东南亚武术".into()]),
        }
    }

    /// 流派分类
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "Silat Gayong: 盖勇流派",
            "Silat Cekak: 切卡流派",
            "Silat Lincah: 林卡流派",
            "Silat Harimau: 虎形流派",
            "Silat Sendeng: 塞登流派",
        ]
    }

    /// 基本技术
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 手部攻击技术",
            "腿法: 腿部攻击技术",
            "肘法: 肘击技术",
            "膝法: 膝击技术",
            "摔法: 摔投技术",
        ]
    }

    /// 动物模仿
    pub fn animal_forms(&self) -> Vec<&'static str> {
        vec![
            "虎形: 猛虎扑击动作",
            "蛇形: 蜿蜒缠绕动作",
            "鹰形: 翱翔抓击动作",
            "猴形: 灵活跳跃动作",
            "龙形: 强劲旋舞动作",
        ]
    }

    /// 武器技能
    pub fn weapons(&self) -> Vec<&'static str> {
        vec![
            "短刀: 传统刀具技术",
            "长刀: 刀剑技术",
            "棍棒: 短棍技术",
            "匕首: 小型武器",
            "空手: 无武器技法",
        ]
    }

    /// 比赛形式
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 动作表演评分",
            "对抗比赛: 实技比赛",
            "团体比赛: 团队竞赛",
            "表演赛: 文化展示",
            "综合比赛: 多项结合",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "完美击打: 高分技术",
            "有效击打: 有效得分",
            "控制得分: 成功控制",
            "套路评分: 动作规范",
            "表演评分: 美感评分",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "正赛时间: 3分钟",
            "决赛时间: 5分钟",
            "套路时间: 3-5分钟",
            "休息时间: 局间1分钟",
            "加时赛: 平局进行",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 比赛必备",
            "控制接触: 安全打击",
            "医疗支持: 赛场医生",
            "裁判监督: 安全保障",
            "禁止危险动作",
        ]
    }

    /// 传统礼仪
    pub fn traditions(&self) -> Vec<&'static str> {
        vec![
            "入场礼仪: 比赛开始礼",
            "退出礼仪: 比赛结束礼",
            "师生礼仪: 传统文化",
            "服装规定: 传统服饰",
            "精神修养: 武德要求",
        ]
    }

    /// 段位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初级级别: 基础掌握",
            "中级级别: 进阶技术",
            "高级级别: 高级技巧",
            "教练级别: 教学资格",
            "大师级别: 最高等级",
        ]
    }
}

impl Default for SilatMelayuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SilatMelayuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("silat_melayu")
    }

    fn explain(&self) -> String {
        format!(
            "【马来传统武术规则】\n\n\
            流派分类:\n{}\n\n\
            基本技术:\n{}\n\n\
            动物模仿:\n{}\n\n\
            武器技能:\n{}\n\n\
            安全规则:\n{}\n",
            self.styles()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.basic_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.animal_forms()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weapons()
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
    fn test_silat_melayu_rules() {
        let rules = SilatMelayuRules::new();
        assert!(!rules.styles().is_empty());
        assert!(!rules.basic_techniques().is_empty());
        assert!(!rules.animal_forms().is_empty());
    }

    #[test]
    fn test_silat_melayu_styles() {
        let rules = SilatMelayuRules::new();
        let styles = rules.styles();
        assert!(styles.contains(&"Silat Gayong: 盖勇流派"));
        assert!(styles.contains(&"Silat Harimau: 虎形流派"));
    }
}