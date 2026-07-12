//! 生物节律规则
//!
//! 生物体节律现象和原理，包括昼夜节律、季节节律、
//! 生物钟、节律调控等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 生物节律规则集合
pub struct ChronobiologyRules {
    metadata: RuleMetadata,
}

impl ChronobiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物节律规则", "生物体节律现象和原理")
                .with_origin("生物节律")
                .with_tags(vec!["科学".into(), "生命科学".into(), "节律".into()]),
        }
    }

    /// 昼夜节律定律
    pub fn circadian_rhythms(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("昼夜节律定律", "~24小时", "约24小时周期"),
            ("光照同步定律", "光照调节", "光照调节节律"),
            ("自由运行定律", "内在周期", "无外界线索的周期"),
            ("相位定律", "节律相位", "节律的相位"),
            ("振幅定律", "节律振幅", "节律变化幅度"),
            ("周期定律", "节律周期", "节律周期长度"),
            ("节律稳定定律", "稳定周期", "节律周期稳定"),
        ]
    }

    /// 生物钟定律
    pub fn biological_clock(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("中央生物钟定律", "SCN", "视交叉上核是中央钟"),
            ("外周生物钟定律", "组织钟", "各组织有外周钟"),
            ("时钟基因定律", "时钟基因", "时钟基因调控节律"),
            ("反馈回路定律", "调控回路", "转录翻译反馈回路"),
            ("同步定律", "钟同步", "中央钟同步外周钟"),
            ("输出通路定律", "节律输出", "节律信号输出"),
            ("信号整合定律", "信号整合", "整合调节节律"),
        ]
    }

    /// 节律基因定律
    pub fn clock_genes(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Clock基因定律", "核心时钟", "Clock是核心时钟基因"),
            ("Bmal1基因定律", "与Clock协同", "Bmal1与Clock协同"),
            ("Period基因定律", "周期基因", "Period调控周期"),
            ("Cryptochrome基因定律", "CRY基因", "CRY参与反馈"),
            ("Rev-Erb基因定律", "负调控", "Rev-Erb负调控"),
            ("Ror基因定律", "正调控", "Ror正调控"),
            ("基因突变定律", "节律改变", "基因突变改变节律"),
        ]
    }

    /// 睡眠觉醒节律定律
    pub fn sleep_wake_rhythm(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("睡眠周期定律", "睡眠觉醒", "睡眠觉醒周期"),
            ("睡眠阶段定律", "睡眠阶段", "睡眠不同阶段"),
            ("REM睡眠定律", "快速眼动", "REM睡眠特征"),
            ("深度睡眠定律", "深睡眠", "深度睡眠特征"),
            ("觉醒定律", "觉醒状态", "觉醒状态调节"),
            ("睡眠时长定律", "睡眠时长", "睡眠时长个体差异"),
            ("睡眠结构定律", "睡眠结构", "睡眠结构变化"),
        ]
    }

    /// 季节节律定律
    pub fn seasonal_rhythms(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("季节节律定律", "年周期", "约一年周期"),
            ("光周期定律", "日照长度", "日照长度指示季节"),
            ("冬眠定律", "冬眠节律", "冬眠季节节律"),
            ("繁殖节律定律", "繁殖季节", "繁殖季节节律"),
            ("迁徙节律定律", "迁徙时间", "迁徙季节节律"),
            ("换毛节律定律", "换毛时间", "换毛季节节律"),
            ("代谢节律定律", "代谢变化", "代谢季节变化"),
        ]
    }

    /// 潮汐节律定律
    pub fn tidal_rhythms(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("潮汐节律定律", "~12.4小时", "约12.4小时周期"),
            ("月节律定律", "月亮周期", "月亮周期节律"),
            ("潮间带定律", "潮间带适应", "潮间带生物节律"),
            ("开壳节律定律", "贝壳开闭", "贝壳开闭节律"),
            ("活动节律定律", "潮汐活动", "潮汐相关活动"),
            ("生殖节律定律", "潮汐生殖", "潮汐相关生殖"),
            ("同步定律", "潮汐同步", "节律与潮汐同步"),
        ]
    }

    /// 节律调控定律
    pub fn rhythm_regulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光照调控定律", "光信号", "光照调控节律"),
            ("温度调控定律", "温度信号", "温度调控节律"),
            ("进食调控定律", "进食时间", "进食调控节律"),
            ("社会调控定律", "社会线索", "社会线索调控"),
            ("运动调控定律", "运动时间", "运动调控节律"),
            ("药物调控定律", "药物调节", "药物调节节律"),
            ("激素调控定律", "激素调节", "激素调节节律"),
        ]
    }

    /// 节律紊乱定律
    pub fn rhythm_disorders(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("时差定律", "时区变化", "跨时区节律紊乱"),
            ("倒班定律", "工作时间", "倒班节律紊乱"),
            ("睡眠紊乱定律", "睡眠障碍", "睡眠节律紊乱"),
            ("节律延迟定律", "相位延迟", "节律相位延迟"),
            ("节律提前定律", "相位提前", "节律相位提前"),
            ("节律消失定律", "节律丧失", "节律丧失"),
            ("非24小时定律", "非24节律", "非24小时节律"),
        ]
    }

    /// 节律测量定律
    pub fn rhythm_measurement(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("节律监测定律", "持续监测", "持续监测节律"),
            ("活动记录定律", "活动监测", "活动节律记录"),
            ("体温监测定律", "体温节律", "体温节律监测"),
            ("激素监测定律", "激素节律", "激素节律监测"),
            ("基因表达定律", "表达节律", "基因表达节律"),
            ("相位分析定律", "相位分析", "节律相位分析"),
            ("周期分析定律", "周期分析", "节律周期分析"),
        ]
    }

    /// 节律应用定律
    pub fn chronobiology_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("时间治疗定律", "最佳给药时间", "根据节律给药"),
            ("睡眠治疗定律", "睡眠调整", "调整睡眠节律"),
            ("光照治疗定律", "光照调节", "光照调节节律"),
            ("时差适应定律", "时差调整", "调整时差"),
            ("倒班适应定律", "倒班适应", "适应倒班"),
            ("最佳表现定律", "最佳时间", "最佳表现时间"),
            ("健康促进定律", "节律健康", "健康节律生活"),
        ]
    }
}

impl Default for ChronobiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChronobiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("chronobiology")
    }

    fn explain(&self) -> String {
        format!(
            "【生物节律规则】\n\n\
            生物节律研究生物体的周期性现象，是理解生物时间调控的基础。\n\n\
            昼夜节律:\n{}\n\n\
            生物钟:\n{}\n\n\
            节律基因:\n{}\n\n\
            睡眠觉醒节律:\n{}\n\n\
            季节节律:\n{}\n\n\
            潮汐节律:\n{}\n\n\
            节律调控:\n{}\n\n\
            节律紊乱:\n{}\n\n\
            节律测量:\n{}\n\n\
            节律应用:\n{}",
            self.circadian_rhythms()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biological_clock()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.clock_genes()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sleep_wake_rhythm()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.seasonal_rhythms()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tidal_rhythms()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rhythm_regulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rhythm_disorders()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rhythm_measurement()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.chronobiology_applications()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chronobiology_rules() {
        let rules = ChronobiologyRules::new();
        assert_eq!(rules.circadian_rhythms().len(), 7);
        assert_eq!(rules.biological_clock().len(), 7);
        assert_eq!(rules.clock_genes().len(), 7);
        assert_eq!(rules.sleep_wake_rhythm().len(), 7);
        assert_eq!(rules.seasonal_rhythms().len(), 7);
        assert_eq!(rules.tidal_rhythms().len(), 7);
        assert_eq!(rules.rhythm_regulation().len(), 7);
        assert_eq!(rules.rhythm_disorders().len(), 7);
        assert_eq!(rules.rhythm_measurement().len(), 7);
        assert_eq!(rules.chronobiology_applications().len(), 7);
    }

    #[test]
    fn test_chronobiology_metadata() {
        let rules = ChronobiologyRules::new();
        assert_eq!(rules.metadata().name, "生物节律规则");
    }
}