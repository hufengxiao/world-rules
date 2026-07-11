//! 适应性雪橇规则
//!
//! 针对不同残疾类型的雪橇运动适应性规则，包括雪橇冰球、雪橇竞速等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 适应性雪橇规则
pub struct AdaptiveSledRules {
    metadata: RuleMetadata,
}

impl AdaptiveSledRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("适应性雪橇规则", "残疾人雪橇运动适应性规则")
                .with_origin("IPC")
                .with_tags(vec![
                    "体育".into(),
                    "冬季运动".into(),
                    "残奥".into(),
                    "适应性".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "雪橇冰球: 下肢残疾运动员",
            "坐姿滑雪: 脊髓损伤、截肢",
            "LW10: 腰椎损伤，坐姿滑雪",
            "LW11: 胸椎损伤，坐姿滑雪",
            "LW12: 髋关节损伤，坐姿滑雪",
            "单板滑雪: 下肢残疾",
            "SB-LL1: 严重下肢残疾",
            "SB-LL2: 轻度下肢残疾",
            "视力残疾滑雪: 使用引导员",
            "B1-B3: 视力残疾程度分级",
        ]
    }

    /// 雪橇冰球规则
    pub fn sledge_hockey_rules(&self) -> Vec<&'static str> {
        vec![
            "雪橇规格: 符合IPC标准",
            "冰刀: 双冰刀设计",
            "座椅: 固定在雪橇框架上",
            "球杆: 双头球杆（一端尖一端平）",
            "击球方式: 允许用球杆两端",
            "移动规则: 使用球杆尖刺推动",
            "犯规规则: 同冰球规则",
            "冲撞限制: 禁止撞击雪橇",
            "守门员规则: 允许特殊装备",
        ]
    }

    /// 坐姿滑雪规则
    pub fn sit_skiing_rules(&self) -> Vec<&'static str> {
        vec![
            "坐式滑雪器: 固定在滑雪板上",
            "悬吊系统: 减震和平衡调节",
            "座椅宽度: 不超过40cm",
            "座椅高度: 不超过地面30cm",
            "扶手: 允许安装",
            "平衡辅助: 允许使用手杖",
            "禁止: 动力辅助",
            "转弯规则: 利用重心转移",
            "出发方式: 坐姿固定出发",
        ]
    }

    /// 单板滑雪适应性
    pub fn snowboard_adaptations(&self) -> Vec<&'static str> {
        vec![
            "假肢固定: 允许假肢固定在雪板",
            "单腿单板: 截肢运动员允许",
            "绑带改装: 适应假肢需求",
            "平衡辅助: 允许使用手杖",
            "起跳规则: 坐姿起跳允许",
            "雪板规格: 符合IPC标准",
            "禁止: 电子辅助平衡",
            "安全装备: 头盔必须佩戴",
        ]
    }

    /// 视力残疾滑雪规则
    pub fn visually_impaired_skiing_rules(&self) -> Vec<&'static str> {
        vec![
            "引导员: 必须使用引导员",
            "通讯系统: 允许使用无线通讯",
            "引导位置: 引导员在前滑行",
            "声音信号: 转弯和障碍提示",
            "速度控制: 引导员负责速度",
            "安全间隔: 引导员保持安全距离",
            "同步计时: 运动员和引导员",
            "引导员资格: 必须有认证资格",
            "通讯频率: 指定频率",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "雪橇: 符合IPC认证",
            "冰刀: 钢制，锋利度合格",
            "座椅: 安全固定",
            "头盔: 必须佩戴认证头盔",
            "护具: 肩、肘、膝护具",
            "球杆: 双头设计，长度合适",
            "坐式滑雪器: IPC认证",
            "滑雪板: 标准规格",
            "通讯设备: 视力残疾专用",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "装备规格违规",
            "危险冲撞",
            "干扰引导员",
            "使用禁止设备",
            "通讯系统违规",
            "分级不符",
            "安全装置失效",
            "接受非法协助",
        ]
    }
}

impl Default for AdaptiveSledRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdaptiveSledRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("adaptive_sled")
    }

    fn explain(&self) -> String {
        format!(
            "【适应性雪橇规则】\n\n\
            运动分级:\n{}\n\n\
            雪橇冰球规则:\n{}\n\n\
            坐姿滑雪规则:\n{}\n\n\
            单板滑雪适应性:\n{}\n\n\
            视力残疾滑雪规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sledge_hockey_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sit_skiing_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.snowboard_adaptations()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.visually_impaired_skiing_rules()
                .iter()
                .map(|v| format!("  • {}", v))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_sled_rules_basic() {
        let rules = AdaptiveSledRules::new();
        assert_eq!(rules.metadata().name, "适应性雪橇规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_adaptive_sled_classification() {
        let rules = AdaptiveSledRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("雪橇冰球")));
        assert!(classification.iter().any(|c| c.contains("LW")));
        assert!(classification.iter().any(|c| c.contains("视力残疾")));
        assert!(classification.len() >= 6);
    }

    #[test]
    fn test_adaptive_sled_hockey() {
        let rules = AdaptiveSledRules::new();
        let hockey = rules.sledge_hockey_rules();
        assert!(hockey.iter().any(|h| h.contains("雪橇")));
        assert!(hockey.iter().any(|h| h.contains("球杆")));
        assert!(hockey.len() >= 6);
    }

    #[test]
    fn test_adaptive_sled_equipment() {
        let rules = AdaptiveSledRules::new();
        let equipment = rules.equipment_requirements();
        assert!(equipment.iter().any(|e| e.contains("头盔")));
        assert!(equipment.iter().any(|e| e.contains("雪橇")));
        assert!(equipment.len() >= 6);
    }

    #[test]
    fn test_adaptive_sled_category() {
        let rules = AdaptiveSledRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
