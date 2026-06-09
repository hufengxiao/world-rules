//! 历史学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 历史学定律集合
pub struct HistoryLaws {
    metadata: RuleMetadata,
}

impl HistoryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("历史学定律", "历史学基本定律")
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

    /// 史学方法
    pub fn historical_methods(&self) -> Vec<&'static str> {
        vec![
            "史料批判: 对历史资料的真实性和可靠性进行考证",
            "年鉴学派: 关注长时段社会经济结构的历史研究",
            "全球史观: 从全球视角理解人类历史的联系",
            "口述史: 通过访谈记录亲历者的历史记忆",
            "计量史学: 运用统计方法分析历史数据",
            "微观史学: 通过小事件揭示大社会结构",
        ]
    }

    /// 历史规律
    pub fn historical_patterns(&self) -> Vec<&'static str> {
        vec![
            "文明兴衰: 文明经历兴起繁荣衰落的周期",
            "技术革命: 关键技术变革推动社会形态转变",
            "帝国周期: 帝国经历扩张巩固衰落的过程",
            "人口转型: 从高出生率高死亡率到低出生率低死亡率",
            "全球化进程: 世界各地联系日益紧密的长期趋势",
            "制度演化: 社会制度在实践中不断调整和完善",
        ]
    }

    /// 经济史
    pub fn economic_history(&self) -> Vec<&'static str> {
        vec![
            "工业革命: 18世纪英国开始的机械化生产变革",
            "大航海时代: 15-17世纪欧洲海上探索和殖民",
            "大萧条: 1929年开始的全球性经济危机",
            "布雷顿森林体系: 二战后建立的国际货币体系",
            "全球化浪潮: 国际贸易和投资自由化的进程",
            "数字革命: 计算机和互联网引发的经济转型",
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

    fn explain(&self) -> String {
        format!(
            "【历史学定律】\n\n发展定律:\n{}\n\n因果定律:\n{}\n\n认知定律:\n{}\n",
            self.development_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.causality_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cognition_laws()
                .iter()
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
