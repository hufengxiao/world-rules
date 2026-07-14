//! 水上项目详细规则
//!
//! 本模块提供水上运动的详细规则实现，涵盖游泳、跳水、水球、花样游泳、公开水域等。
//! 符合世界泳联 (World Aquatics, 原FINA) 标准。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 游泳泳姿类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwimmingStroke {
    /// 自由泳
    Freestyle,
    /// 仰泳
    Backstroke,
    /// 蛙泳
    Breaststroke,
    /// 蝶泳
    Butterfly,
    /// 混合泳
    IndividualMedley,
}

impl SwimmingStroke {
    /// 获取泳姿名称
    pub fn name(&self) -> &'static str {
        match self {
            SwimmingStroke::Freestyle => "自由泳",
            SwimmingStroke::Backstroke => "仰泳",
            SwimmingStroke::Breaststroke => "蛙泳",
            SwimmingStroke::Butterfly => "蝶泳",
            SwimmingStroke::IndividualMedley => "混合泳",
        }
    }

    /// 是否需要双手触壁
    pub fn requires_two_hand_touch(&self) -> bool {
        matches!(
            self,
            SwimmingStroke::Breaststroke | SwimmingStroke::Butterfly
        )
    }

    /// 是否允许滚翻转身
    pub fn allows_flip_turn(&self) -> bool {
        matches!(
            self,
            SwimmingStroke::Freestyle | SwimmingStroke::Backstroke
        )
    }
}

/// 跳水项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivingEvent {
    /// 1米跳板
    Springboard1m,
    /// 3米跳板
    Springboard3m,
    /// 10米跳台
    Platform10m,
    /// 双人3米跳板
    Synchronized3m,
    /// 双人10米跳台
    Synchronized10m,
}

impl DivingEvent {
    /// 获取项目名称
    pub fn name(&self) -> &'static str {
        match self {
            DivingEvent::Springboard1m => "1米跳板",
            DivingEvent::Springboard3m => "3米跳板",
            DivingEvent::Platform10m => "10米跳台",
            DivingEvent::Synchronized3m => "双人3米跳板",
            DivingEvent::Synchronized10m => "双人10米跳台",
        }
    }

    /// 是否为双人项目
    pub fn is_synchronized(&self) -> bool {
        matches!(self, DivingEvent::Synchronized3m | DivingEvent::Synchronized10m)
    }

    /// 获取高度（米）
    pub fn height(&self) -> u8 {
        match self {
            DivingEvent::Springboard1m => 1,
            DivingEvent::Springboard3m => 3,
            DivingEvent::Platform10m => 10,
            DivingEvent::Synchronized3m => 3,
            DivingEvent::Synchronized10m => 10,
        }
    }
}

/// 水上项目详细规则
pub struct AquaticsDetailedRules {
    metadata: RuleMetadata,
}

impl AquaticsDetailedRules {
    /// 创建新的水上项目详细规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水上项目详细规则", "世界泳联标准水上比赛规则")
                .with_origin("World Aquatics (WA)")
                .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    // ==================== 游泳规则 ====================

    /// 标准泳池规格
    pub fn pool_specifications(&self) -> Vec<&'static str> {
        vec![
            "奥运会标准池: 50米长",
            "短池: 25米长",
            "宽度: 至少21米（8条泳道）",
            "深度: 至少2米（奥运会）",
            "泳道宽度: 2.5米",
            "泳道线: 必须使用防浪线",
            "水温: 25-28°C（国际比赛26°C ±1°C）",
            "照明: 至少1500勒克斯",
            "出发台: 高度0.5-0.75米",
        ]
    }

    /// 游泳出发规则
    pub fn swimming_starting_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳、蛙泳、蝶泳: 跳台出发",
            "仰泳: 水中出发（手握扶手）",
            "口令: '各就位' → 长哨声 → 枪响",
            "跳台出发: 听到枪响后起跳",
            "仰泳出发: 脚不得露出水面",
            "抢跳判罚: 第一次警告，第二次取消资格",
            "抢跳检测: 使用电子起跳监测系统",
            "重赛条件: 发生抢跳或有技术故障",
        ]
    }

    /// 自由泳技术规则
    pub fn freestyle_rules(&self) -> Vec<&'static str> {
        vec![
            "定义: 可采用任何泳姿",
            "实际使用: 爬泳（最效率）",
            "身体位置: 多数时间俯卧",
            "转身: 可采用任何方式（通常滚翻）",
            "转身要求: 必须触壁",
            "终点: 可单手触壁",
            "潜泳: 出发和转身后潜泳不超过15米",
            "犯规: 走动、借助池底",
        ]
    }

    /// 仰泳技术规则
    pub fn backstroke_rules(&self) -> Vec<&'static str> {
        vec![
            "定义: 全程保持仰卧姿势",
            "出发: 水中出发，双手握扶手",
            "身体翻转: 转身时不得超过90度",
            "转身: 可采用滚翻转身",
            "转身后: 必须恢复仰卧姿势",
            "潜泳: 出发和转身后不超过15米",
            "终点: 必须仰卧触壁",
            "犯规: 翻转超过90度、站立",
        ]
    }

    /// 蛙泳技术规则
    pub fn breaststroke_rules(&self) -> Vec<&'static str> {
        vec![
            "定义: 采用蛙泳技术",
            "手臂动作: 双手必须同时划水",
            "腿部动作: 双腿必须同时蹬腿",
            "禁止动作: 蝶泳腿（交替打水）",
            "头部位置: 每次划水后头须露出水面",
            "转身: 必须双手同时触壁",
            "转身后: 可做一次长划臂",
            "潜泳: 出发和转身后可做一次完整动作",
            "犯规: 蝶泳腿、不对称动作",
        ]
    }

    /// 蝶泳技术规则
    pub fn butterfly_rules(&self) -> Vec<&'static str> {
        vec![
            "定义: 采用蝶泳技术",
            "手臂动作: 双臂必须同时向前挥动",
            "腿部动作: 双腿必须同时上下打水",
            "身体位置: 保持俯卧姿势",
            "海豚腿: 允许使用",
            "转身: 必须双手同时触壁",
            "潜泳: 出发和转身后不超过15米",
            "犯规: 交替打腿、单手触壁",
        ]
    }

    /// 混合泳规则
    pub fn medley_rules(&self) -> Vec<&'static str> {
        vec![
            "顺序: 蝶泳 → 仰泳 → 蛙泳 → 自由泳",
            "每种泳姿游1/4距离",
            "蝶泳: 必须双手同时触壁",
            "仰泳转蛙泳: 必须仰卧触壁",
            "蛙泳: 必须双手同时触壁",
            "自由泳: 最后一段采用自由泳",
            "自由泳段: 可采用任何泳姿（通常爬泳）",
            "接力混合泳顺序: 仰泳 → 蛙泳 → 蝶泳 → 自由泳",
        ]
    }

    /// 游泳接力规则
    pub fn swimming_relay_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳接力: 4x100米、4x200米",
            "混合泳接力: 4x100米（仰→蛙→蝶→自）",
            "交接棒: 前一运动员触壁后",
            "抢跳: 前一运动员触壁前起跳",
            "时间差: 通常在0.03-0.05秒",
            "犯规: 抢跳取消资格",
            "电子监测: 使用接力起跳监测系统",
            "队员顺序: 赛前提交，不得更改",
        ]
    }

    /// 公开水域游泳规则
    pub fn open_water_rules(&self) -> Vec<&'static str> {
        vec![
            "项目: 5公里、10公里、25公里",
            "场地: 自然水域（湖泊、河流、海洋）",
            "水温: 16-31°C",
            "出发: 站立式或水中出发",
            "泳帽颜色: 按种子排名分配",
            "身体标记: 编号写在身体上",
            "补给站: 每2.5公里设置",
            "跟游: 禁止跟随对手超过5秒",
            "犯规: 拉扯、阻挡、危险动作",
            "终点: 必须触碰电子计时板",
        ]
    }

    // ==================== 跳水规则 ====================

    /// 跳水项目列表
    pub fn diving_events(&self) -> Vec<&'static str> {
        vec![
            "1米跳板（非奥运会）",
            "3米跳板",
            "10米跳台",
            "双人3米跳板",
            "双人10米跳台",
            "高台跳水（27米/20米）",
        ]
    }

    /// 跳水动作姿势
    pub fn diving_positions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("A", "直体（Straight）"),
            ("B", "屈体（Pike）"),
            ("C", "抱膝（Tuck）"),
            ("D", "任意姿势（Free）"),
        ]
    }

    /// 跳水动作组别
    pub fn diving_groups(&self) -> Vec<&'static str> {
        vec![
            "第1组: 面对池向前跳水",
            "第2组: 面对板向后跳水",
            "第3组: 面对池反身跳水",
            "第4组: 面对板向内跳水",
            "第5组: 转体跳水",
            "第6组: 臂立跳水（仅跳台）",
        ]
    }

    /// 跳水评分规则
    pub fn diving_scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "个人项目: 7名裁判",
            "双人项目: 11名裁判（执行分6人+同步分5人）",
            "分数范围: 0-10分（0.5分递增）",
            "个人计分: 去掉2个最高分和2个最低分",
            "双人计分: 执行分平均值 + 同步分平均值",
            "总分公式: 有效分之和 × 难度系数 × 0.6",
            "难度系数: 1.2 - 4.2",
            "裁判关注: 起跳、空中、入水",
        ]
    }

    /// 跳水扣分因素
    pub fn diving_deductions(&self) -> Vec<&'static str> {
        vec![
            "入水水花: -0.5至-2分",
            "身体姿态: 未完全伸直 -0.5至-1分",
            "空中角度: 转体角度偏差 -0.5分",
            "入水角度: 非垂直入水 -0.5至-2分",
            "脚尖绷直度: 未绷直 -0.5分",
            "起跳高度: 高度不足 -0.5分",
            "动作完成度: 未完成预定周数 -2分或更多",
            "双人同步: 时间差 > 0.5秒 -0.5至-2分",
        ]
    }

    /// 跳板和跳台规格
    pub fn diving_facility_specs(&self) -> Vec<&'static str> {
        vec![
            "3米跳板: 长4.8米，宽0.5米",
            "跳板弹性: 符合WA认证标准",
            "10米跳台: 长6米，宽2米",
            "跳台高度: 10米 ± 0.05米",
            "水深: 至少5米",
            "水面照明: 至少500勒克斯",
            "气泡系统: 可启用（减少水花）",
            "安全区域: 跳板周围无障碍物",
        ]
    }

    // ==================== 花样游泳规则 ====================

    /// 花样游泳项目
    pub fn artistic_swimming_events(&self) -> Vec<&'static str> {
        vec![
            "双人技术自选",
            "双人自由自选",
            "团体技术自选",
            "团体自由自选",
            "自由组合",
            "技巧自选（新规则）",
        ]
    }

    /// 花样游泳评分规则
    pub fn artistic_swimming_scoring(&self) -> Vec<&'static str> {
        vec![
            "裁判数量: 技术裁判3人、艺术裁判3人",
            "分数范围: 0-10分",
            "技术分: 执行难度、完成质量",
            "艺术分: 编排、音乐表现、艺术印象",
            "难度分: 单独计算",
            "总分公式: (技术分 + 艺术分) × 难度系数",
            "扣分: 时间超时、触底、辅助道具违规",
            "申诉: 必须在成绩公布后30分钟内",
        ]
    }

    /// 花样游泳技术要求
    pub fn artistic_swimming_requirements(&self) -> Vec<&'static str> {
        vec![
            "技术自选: 必须完成规定动作",
            "自由自选: 自由编排动作",
            "音乐: 必须使用背景音乐",
            "时间限制: 技术自选2分20秒±15秒",
            "团体人数: 8人（最少4人）",
            "泳衣: 不得过于透明",
            "首饰: 禁止佩戴",
            "发型: 可使用发胶固定",
        ]
    }

    // ==================== 水球规则 ====================

    /// 水球基本规则
    pub fn water_polo_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节，每节8分钟（净时间）",
            "场地: 25米长，20米宽（男）",
            "水深: 至少2米",
            "队员: 每队13人（7人上场）",
            "门将: 特殊泳帽颜色",
            "进攻时间: 30秒内必须射门",
            "得分: 球完全越过球门线",
            "犯规: 普通、严重、罚出场",
        ]
    }

    /// 水球犯规规则
    pub fn water_polo_fouls(&self) -> Vec<&'static str> {
        vec![
            "普通犯规: 判对方球权",
            "严重犯规: 罚出场20秒",
            "暴力犯规: 罚出场4分钟或红牌",
            "阻挡: 普通犯规",
            "拉拽: 严重犯规",
            "踢打: 暴力犯规",
            "累计犯规: 3次严重犯规后罚出场",
            "点球: 严重犯规在5米区内",
        ]
    }

    /// 水球场地和装备
    pub fn water_polo_equipment(&self) -> Vec<&'static str> {
        vec![
            "球门: 3米宽，0.9米高",
            "球: 男子68-71厘米，400-450克",
            "球: 女子65-67厘米，400-450克",
            "泳帽: 必须佩戴（双方不同颜色）",
            "泳衣: 符合WA标准",
            "裁判: 2名裁判",
            "计时: 使用电子计时系统",
            "换人: 可随时换人（需在换人区）",
        ]
    }

    // ==================== 高台跳水规则 ====================

    /// 高台跳水规则
    pub fn high_diving_rules(&self) -> Vec<&'static str> {
        vec![
            "男子高度: 27米",
            "女子高度: 20米",
            "入水速度: 约85公里/小时（27米）",
            "入水深度: 可达5米",
            "安全措施: 潜水员在场待命",
            "比赛轮次: 4轮",
            "裁判: 7人制",
            "难度系数: 2.0-5.5",
        ]
    }

    // ==================== 大师游泳规则 ====================

    /// 大师游泳规则
    pub fn masters_swimming_rules(&self) -> Vec<&'static str> {
        vec![
            "年龄组: 25-29, 30-34, ..., 100+",
            "接力年龄: 取平均年龄",
            "比赛距离: 50米、100米、200米、400米",
            "规则: 与标准游泳相同",
            "纪录: 分年龄组纪录",
            "参赛资格: 通常无资格限制",
            "热身: 允许热身泳道",
            "时间限制: 无具体限制",
        ]
    }

    // ==================== 残疾人游泳规则 ====================

    /// 残疾人游泳规则
    pub fn para_swimming_rules(&self) -> Vec<&'static str> {
        vec![
            "分级: S（游泳）、SB（花样）、SM（混合）",
            "S1-S10: 身体残疾程度递减",
            "S11-S13: 视力障碍程度递减",
            "S14: 智力残疾",
            "起跳辅助: 可使用辅助设备",
            "触壁辅助: 可使用辅助棒",
            "泳姿调整: 根据残疾类型调整",
            "盲人游泳: 可使用引导员",
        ]
    }

    // ==================== 通用规则 ====================

    /// 反兴奋剂规定
    pub fn anti_doping_rules(&self) -> Vec<&'static str> {
        vec![
            "遵守WADA反兴奋剂条例",
            "禁药清单: WADA年度发布",
            "检测: 赛内和赛外均可检测",
            "治疗用药豁免: 需提前申请TUE",
            "行踪申报: 精英运动员需申报",
            "违规处罚: 禁赛2-4年",
            "申诉: CAS仲裁",
            "检测时间: 赛后立即进行",
        ]
    }

    /// 世界纪录认定规则
    pub fn world_record_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛级别: 需为WA认证赛事",
            "计时系统: 电子计时",
            "泳池认证: 符合WA认证标准",
            "水温记录: 需记录水温",
            "申请程序: 赛后提交申请材料",
            "审核时间: WA技术委员会审核",
            "短池纪录: 25米池纪录单独认证",
            "公开水域: 需记录水温、水质等",
        ]
    }

    /// 年龄组别
    pub fn age_groups(&self) -> Vec<&'static str> {
        vec![
            "青年组: 14-18岁",
            "成年组: 18岁以上",
            "大师赛: 25岁以上（每5岁一组）",
            "年龄限制: 各赛事有具体规定",
            "青年纪录: 分年龄组纪录",
            "大师纪录: 分年龄组纪录",
            "青年比赛: 世界青年游泳锦标赛",
        ]
    }

    /// 比赛装备规定
    pub fn equipment_regulations(&self) -> Vec<&'static str> {
        vec![
            "泳衣: 符合WA认证标准",
            "泳帽: 材质不限",
            "泳镜: 允许使用",
            "禁止装备: 脚蹼、手蹼",
            "泳衣覆盖: 男士腰至膝，女士腰至膝",
            "材料: 纺织材料（禁止聚氨酯）",
            "厚度: 不超过0.8毫米",
            "透水率: 符合WA标准",
        ]
    }
}

impl Default for AquaticsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AquaticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("aquatics_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【水上项目详细规则】\n\
            世界泳联(World Aquatics)标准规则\n\n\
            泳池规格:\n{}\n\n\
            游泳出发规则:\n{}\n\n\
            跳水评分规则:\n{}\n\n\
            水球规则:\n{}\n\n\
            花样游泳技术要求:\n{}\n",
            self.pool_specifications()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.swimming_starting_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diving_scoring_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.water_polo_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.artistic_swimming_requirements()
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
    fn test_aquatics_detailed_rules_creation() {
        let rules = AquaticsDetailedRules::new();
        assert_eq!(rules.metadata().name, "水上项目详细规则");
    }

    #[test]
    fn test_swimming_stroke_properties() {
        assert!(SwimmingStroke::Breaststroke.requires_two_hand_touch());
        assert!(!SwimmingStroke::Freestyle.requires_two_hand_touch());
        assert!(SwimmingStroke::Freestyle.allows_flip_turn());
        assert!(!SwimmingStroke::Butterfly.allows_flip_turn());
    }

    #[test]
    fn test_diving_event_properties() {
        assert_eq!(DivingEvent::Platform10m.height(), 10);
        assert!(DivingEvent::Synchronized3m.is_synchronized());
        assert!(!DivingEvent::Springboard3m.is_synchronized());
    }

    #[test]
    fn test_pool_specifications() {
        let rules = AquaticsDetailedRules::new();
        let specs = rules.pool_specifications();
        assert!(!specs.is_empty());
        assert!(specs.iter().any(|s| s.contains("50米")));
    }

    #[test]
    fn test_swimming_starting_rules() {
        let rules = AquaticsDetailedRules::new();
        let rules_list = rules.swimming_starting_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("出发")));
    }

    #[test]
    fn test_diving_groups() {
        let rules = AquaticsDetailedRules::new();
        let groups = rules.diving_groups();
        assert!(!groups.is_empty());
        assert_eq!(groups.len(), 6);
    }

    #[test]
    fn test_water_polo_rules() {
        let rules = AquaticsDetailedRules::new();
        let rules_list = rules.water_polo_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("比赛时间")));
    }

    #[test]
    fn test_explain() {
        let rules = AquaticsDetailedRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("水上项目详细规则"));
        assert!(explanation.contains("World Aquatics"));
    }
}