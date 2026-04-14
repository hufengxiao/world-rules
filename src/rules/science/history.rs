//! 历史学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 历史学定律集合
pub struct HistoryLaws {
    metadata: RuleMetadata,
}

impl HistoryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "历史学定律",
                "历史学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "历史".into()]),
        }
    }

    /// 历史发展定律
    pub fn development_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("历史进程定律", "连续发展", "历史连续发展"),
            ("历史规律定律", "规律性", "历史发展规律"),
            ("历史循环定律", "周期现象", "历史周期现象"),
            ("历史进步定律", "进步趋势", "历史进步趋势"),
            ("历史转折定律", "关键事件", "历史转折点"),
            ("历史传承定律", "文化传承", "历史文化传承"),
            ("历史变革定律", "社会变革", "历史社会变革"),
        ]
    }

    /// 历史因果定律
    pub fn causality_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("因果定律", "因果关系", "历史因果关系"),
            ("必然性定律", "必然趋势", "历史必然性"),
            ("偶然性定律", "偶然事件", "历史偶然性"),
            ("多因素定律", "多重原因", "历史多因素"),
            ("长期趋势定律", "长期影响", "长期历史趋势"),
            ("短期波动定律", "短期变化", "短期历史变化"),
            ("交互作用定律", "相互影响", "因素相互影响"),
        ]
    }

    /// 历史认知定律
    pub fn cognition_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("历史客观定律", "客观事实", "历史客观存在"),
            ("历史主观定律", "主观理解", "历史主观理解"),
            ("历史视角定律", "多元视角", "多元历史视角"),
            ("历史证据定律", "证据支撑", "历史证据支撑"),
            ("历史解释定律", "解释多样性", "历史解释多样性"),
            ("历史记忆定律", "集体记忆", "集体历史记忆"),
            ("历史书写定律", "书写规律", "历史书写规律"),
        ]
    }

    /// 历史分期定律
    pub fn periodization_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分期定律", "时期划分", "历史时期划分"),
            ("断代定律", "断代标准", "历史断代标准"),
            ("分期依据定律", "划分依据", "分期划分依据"),
            ("分期意义定律", "分期意义", "分期研究意义"),
            ("分期争议定律", "分期讨论", "分期划分争议"),
        ]
    }

    /// 历史时期
    pub fn periods(&self) -> Vec<&'static str> {
        vec![
            "古代史",
            "近代史",
            "现代史",
            "当代史",
            "上古史",
            "中世纪",
            "工业革命",
            "信息时代",
        ]
    }

    /// 历史研究方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "文献研究",
            "考古方法",
            "比较研究",
            "计量史学",
            "口述历史",
            "档案研究",
            "田野调查",
            "跨学科研究",
        ]
    }
}

impl Default for HistoryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HistoryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("history")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【历史学定律】\n\n发展定律:\n{}\n\n因果定律:\n{}\n\n认知定律:\n{}\n",
            self.development_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.causality_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cognition_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_laws() {
        let laws = HistoryLaws::new();
        assert!(!laws.development_laws().is_empty());
        assert!(!laws.causality_laws().is_empty());
    }
}