//! 适应性田径规则
//!
//! 针对轮椅田径、假肢田径等不同残疾类型的适应性规则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 适应性田径规则
pub struct AdaptiveAthleticsRules {
    metadata: RuleMetadata,
}

impl AdaptiveAthleticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("适应性田径规则", "残疾人田径适应性规则")
                .with_origin("IPC/WPA")
                .with_tags(vec![
                    "体育".into(),
                    "田径".into(),
                    "残奥".into(),
                    "适应性".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "T级: 径赛（跑道项目）",
            "T11-T13: 视力残疾",
            "T20: 智力残疾",
            "T31-34: 脑瘫（轮椅）",
            "T35-38: 脑瘫（站立）",
            "T40-47: 截肢和其他肢体残疾",
            "T51-54: 脊髓损伤（轮椅）",
            "F级: 田赛（跳跃和投掷）",
            "F11-F13: 视力残疾田赛",
            "F31-38: 脑瘫田赛",
            "F40-47: 截肢和其他肢体残疾田赛",
            "F51-57: 脊髓损伤田赛",
        ]
    }

    /// 轮椅竞速规则
    pub fn wheelchair_racing_rules(&self) -> Vec<&'static str> {
        vec![
            "轮椅规格: 符合IPC标准",
            "轮径限制: 最大70cm",
            "最大宽度: 不超过85cm",
            "最小重量: 不低于10kg",
            "后轮转向: 允许固定系统",
            "手圈: 必须安装在驱动轮上",
            "禁止: 电子驱动辅助",
            "赛道规则: 分道比赛",
            "超车规则: 必须从右侧超越",
        ]
    }

    /// 义肢赛跑规则
    pub fn prosthetic_running_rules(&self) -> Vec<&'static str> {
        vec![
            "义肢类型: 跑步专用义肢",
            "长度限制: 根据身高计算",
            "材料要求: 碳纤维弹性义肢",
            "检查要求: 赛前必须通过技术检查",
            "备用义肢: 允许携带备用",
            "禁止: 电子储能装置",
            "禁止: 可调节高度义肢",
            "对称性检查: 双腿长度差限制",
        ]
    }

    /// 投掷规则适应性
    pub fn throwing_adaptations(&self) -> Vec<&'static str> {
        vec![
            "坐姿投掷: 使用投掷凳",
            "站姿投掷: 标准姿势允许",
            "假肢固定: 投掷臂假肢允许",
            "引导辅助: 视力残疾使用引导员",
            "投掷区域: 轮椅投掷区加固",
            "握持辅助: 允许使用手套/绑带",
            "重量调整: 可申请使用轻量器械",
            "旋转投掷: 轮椅固定旋转投掷",
        ]
    }

    /// 跳跃规则适应性
    pub fn jumping_adaptations(&self) -> Vec<&'static str> {
        vec![
            "助跑辅助: 视力残疾使用引导绳",
            "单腿跳跃: 假肢跳跃允许",
            "起跳规则: 单腿起跳有效",
            "落地区域: 加宽安全区域",
            "轮椅跳高: 轮椅跳高项目",
            "假肢规定: 必须通过技术检查",
            "引导员: 视力残疾允许引导员",
            "禁止: 弹性鞋底辅助",
        ]
    }

    /// 起跑规则
    pub fn starting_rules(&self) -> Vec<&'static str> {
        vec![
            "起跑器: 轮椅起跑器固定",
            "信号适应: 视力残疾使用声音信号",
            "扶助器: 允许起跑扶助",
            "假肢准备: 允许额外准备时间",
            "分级起跑: 同级运动员同组",
            "抢跑规则: 两次抢跑取消资格",
            "站立困难: 允许坐姿起跑",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "轮椅规格不符",
            "假肢长度违规",
            "使用禁止辅助设备",
            "干扰其他运动员",
            "接受非法场外协助",
            "分级不符",
            "起跑犯规",
            "赛道违规",
        ]
    }
}

impl Default for AdaptiveAthleticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdaptiveAthleticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("adaptive_athletics")
    }

    fn explain(&self) -> String {
        format!(
            "【适应性田径规则】\n\n\
            运动分级:\n{}\n\n\
            轮椅竞速规则:\n{}\n\n\
            义肢赛跑规则:\n{}\n\n\
            投掷规则适应性:\n{}\n\n\
            跳跃规则适应性:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wheelchair_racing_rules()
                .iter()
                .map(|w| format!("  • {}", w))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prosthetic_running_rules()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.throwing_adaptations()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.jumping_adaptations()
                .iter()
                .map(|j| format!("  • {}", j))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_athletics_rules_basic() {
        let rules = AdaptiveAthleticsRules::new();
        assert_eq!(rules.metadata().name, "适应性田径规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_adaptive_athletics_classification() {
        let rules = AdaptiveAthleticsRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("T级")));
        assert!(classification.iter().any(|c| c.contains("F级")));
        assert!(classification.iter().any(|c| c.contains("视力残疾")));
        assert!(classification.len() >= 8);
    }

    #[test]
    fn test_adaptive_athletics_wheelchair() {
        let rules = AdaptiveAthleticsRules::new();
        let wheelchair = rules.wheelchair_racing_rules();
        assert!(wheelchair.iter().any(|w| w.contains("轮椅规格")));
        assert!(wheelchair.iter().any(|w| w.contains("禁止")));
        assert!(wheelchair.len() >= 6);
    }

    #[test]
    fn test_adaptive_athletics_prosthetic() {
        let rules = AdaptiveAthleticsRules::new();
        let prosthetic = rules.prosthetic_running_rules();
        assert!(prosthetic.iter().any(|p| p.contains("义肢")));
        assert!(prosthetic.iter().any(|p| p.contains("长度")));
        assert!(prosthetic.len() >= 5);
    }

    #[test]
    fn test_adaptive_athletics_category() {
        let rules = AdaptiveAthleticsRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
