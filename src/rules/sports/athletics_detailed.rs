//! 田径项目详细规则
//!
//! 本模块提供田径运动的详细规则实现，涵盖径赛、田赛、全能等项目。
//! 符合世界田联 (World Athletics, 原IAAF) 标准。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 径赛项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackEvent {
    /// 100米短跑
    Sprint100m,
    /// 200米短跑
    Sprint200m,
    /// 400米短跑
    Sprint400m,
    /// 800米中跑
    Middle800m,
    /// 1500米中跑
    Middle1500m,
    /// 5000米长跑
    Long5000m,
    /// 10000米长跑
    Long10000m,
    /// 马拉松
    Marathon,
    /// 110米栏（男）
    Hurdles110m,
    /// 100米栏（女）
    Hurdles100m,
    /// 400米栏
    Hurdles400m,
    /// 3000米障碍
    Steeplechase,
    /// 4x100米接力
    Relay4x100m,
    /// 4x400米接力
    Relay4x400m,
}

impl TrackEvent {
    /// 获取项目名称
    pub fn name(&self) -> &'static str {
        match self {
            TrackEvent::Sprint100m => "100米",
            TrackEvent::Sprint200m => "200米",
            TrackEvent::Sprint400m => "400米",
            TrackEvent::Middle800m => "800米",
            TrackEvent::Middle1500m => "1500米",
            TrackEvent::Long5000m => "5000米",
            TrackEvent::Long10000m => "10000米",
            TrackEvent::Marathon => "马拉松",
            TrackEvent::Hurdles110m => "110米栏",
            TrackEvent::Hurdles100m => "100米栏",
            TrackEvent::Hurdles400m => "400米栏",
            TrackEvent::Steeplechase => "3000米障碍",
            TrackEvent::Relay4x100m => "4x100米接力",
            TrackEvent::Relay4x400m => "4x400米接力",
        }
    }

    /// 是否需要起跑器
    pub fn requires_starting_blocks(&self) -> bool {
        matches!(
            self,
            TrackEvent::Sprint100m
                | TrackEvent::Sprint200m
                | TrackEvent::Sprint400m
                | TrackEvent::Hurdles110m
                | TrackEvent::Hurdles100m
                | TrackEvent::Hurdles400m
                | TrackEvent::Relay4x100m
        )
    }

    /// 是否使用分道跑
    pub fn uses_lanes(&self) -> bool {
        matches!(
            self,
            TrackEvent::Sprint100m
                | TrackEvent::Sprint200m
                | TrackEvent::Sprint400m
                | TrackEvent::Hurdles110m
                | TrackEvent::Hurdles100m
                | TrackEvent::Hurdles400m
                | TrackEvent::Relay4x100m
        )
    }
}

/// 田赛项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldEvent {
    // 跳跃项目
    /// 跳高
    HighJump,
    /// 撑杆跳高
    PoleVault,
    /// 跳远
    LongJump,
    /// 三级跳远
    TripleJump,
    // 投掷项目
    /// 铅球
    ShotPut,
    /// 铁饼
    Discus,
    /// 标枪
    Javelin,
    /// 链球
    Hammer,
}

impl FieldEvent {
    /// 获取项目名称
    pub fn name(&self) -> &'static str {
        match self {
            FieldEvent::HighJump => "跳高",
            FieldEvent::PoleVault => "撑杆跳高",
            FieldEvent::LongJump => "跳远",
            FieldEvent::TripleJump => "三级跳远",
            FieldEvent::ShotPut => "铅球",
            FieldEvent::Discus => "铁饼",
            FieldEvent::Javelin => "标枪",
            FieldEvent::Hammer => "链球",
        }
    }

    /// 是否为跳跃项目
    pub fn is_jumping_event(&self) -> bool {
        matches!(
            self,
            FieldEvent::HighJump
                | FieldEvent::PoleVault
                | FieldEvent::LongJump
                | FieldEvent::TripleJump
        )
    }

    /// 是否为投掷项目
    pub fn is_throwing_event(&self) -> bool {
        !self.is_jumping_event()
    }
}

/// 全能项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinedEvent {
    /// 男子十项全能
    Decathlon,
    /// 女子七项全能
    Heptathlon,
}

impl CombinedEvent {
    /// 获取项目名称
    pub fn name(&self) -> &'static str {
        match self {
            CombinedEvent::Decathlon => "十项全能",
            CombinedEvent::Heptathlon => "七项全能",
        }
    }

    /// 获取包含的项目数量
    pub fn event_count(&self) -> u8 {
        match self {
            CombinedEvent::Decathlon => 10,
            CombinedEvent::Heptathlon => 7,
        }
    }
}

/// 田径详细规则
pub struct AthleticsDetailedRules {
    metadata: RuleMetadata,
}

impl AthleticsDetailedRules {
    /// 创建新的田径详细规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("田径详细规则", "世界田联标准田径比赛规则")
                .with_origin("World Athletics (WA)")
                .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 跑道规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准跑道长度: 400米（最内圈测量线）",
            "跑道数量: 8条（国际标准比赛）",
            "跑道宽度: 1.22米 ± 0.01米",
            "弯道半径: 36.5米（标准设计）",
            "跑道倾斜度: 不超过1%（弯道不超过2%）",
            "表面材料: 合成橡胶（符合WA认证）",
            "内场: 可用于田赛项目",
            "安全区域: 跑道内侧至少1米安全带",
        ]
    }

    /// 起跑规则
    pub fn starting_rules(&self) -> Vec<&'static str> {
        vec![
            "短跑（400米及以下）: 使用起跑器",
            "口令: '各就位' → '预备' → 鸣枪/发令枪响",
            "起跑器位置: 100米和110米栏为持续性起跑",
            "中长跑: 站立式起跑，无起跑器",
            "抢跑判罚: 第一次抢跑警告全组，第二次抢跑者取消资格",
            "反应时间阈值: 0.100秒（小于此值为抢跑）",
            "起跑犯规监测: 使用电子起跑监测系统",
            "发令员权限: 可召回比赛（如运动员未就位）",
        ]
    }

    /// 分道跑规则
    pub fn lane_rules(&self) -> Vec<&'static str> {
        vec![
            "100米、200米、400米: 全程分道跑",
            "800米: 第一个弯道后可抢道（使用切线标志）",
            "4x400米接力: 第三棒运动员可抢道",
            "抢道线: 绿色标志线（800米）",
            "跑出本跑道: 不影响他人不判罚",
            "阻挡犯规: 故意阻挡他人取消资格",
            "串道: 踩踏内侧道线犯规",
            "外侧道线: 踩踏不判罚",
        ]
    }

    /// 接力比赛规则
    pub fn relay_rules(&self) -> Vec<&'static str> {
        vec![
            "4x100米接力区: 30米（预跑区10米）",
            "4x400米接力区: 20米",
            "交接棒: 必须在接力区内完成",
            "掉棒: 由掉棒运动员捡起，可跑回捡棒",
            "交接棒方式: '上挑式'或'下压式'",
            "阻挡犯规: 交接棒时阻挡其他队取消资格",
            "运动员位置: 必须在指定跑道等待",
            "接力棒规格: 中空圆形，长28-30厘米，重50克",
        ]
    }

    /// 跨栏规则
    pub fn hurdles_specifications(&self) -> Vec<&'static str> {
        vec![
            "男子110米栏: 栏高1.067米，10个栏，栏间距9.14米",
            "女子100米栏: 栏高0.84米，10个栏，栏间距8.5米",
            "男子400米栏: 栏高0.914米，10个栏，栏间距35米",
            "女子400米栏: 栏高0.762米，10个栏，栏间距35米",
            "起跑到第一栏距离: 110米栏为13.72米",
            "最后一栏到终点: 110米栏为14.02米",
            "栏架宽度: 1.18-1.20米",
            "栏架重量: 不超过10公斤",
            "碰栏: 不判罚（除非故意推倒）",
            "栏架阻力: 3.6-4公斤力可推倒",
        ]
    }

    /// 3000米障碍规则
    pub fn steeplechase_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离: 3000米",
            "障碍栏架数量: 28个（每圈7个）",
            "水池障碍: 7个（每圈1个）",
            "障碍栏架高度: 男子0.914米，女子0.762米",
            "水池深度: 最深15厘米（前沿）",
            "水池长度: 3.66米（包括栏架）",
            "起跳点: 可在栏架任何一侧起跳",
            "踩水池边缘: 允许",
            "碰栏: 不判罚",
        ]
    }

    /// 跳高规则
    pub fn high_jump_rules(&self) -> Vec<&'static str> {
        vec![
            "起跳高度: 运动员自行选择",
            "升杆高度: 每次至少2厘米",
            "试跳次数: 每高度最多3次",
            "连续失败: 连续3次失败（同一高度或不同高度）即淘汰",
            "过杆姿势: 允许任何姿势（背越式最常见）",
            "失败判定: 横杆掉落、身体触地（起跳脚）",
            "助跑限制: 无限制（通常20-30米）",
            "时间限制: 1分钟内完成试跳（最后阶段2分钟）",
            "成绩相等: 按失败次数少者优先、试跳次数少者优先",
        ]
    }

    /// 撑杆跳高规则
    pub fn pole_vault_rules(&self) -> Vec<&'static str> {
        vec![
            "起跳高度: 运动员自行选择",
            "升杆高度: 每次至少5厘米",
            "试跳次数: 每高度最多3次",
            "撑杆材质: 玻璃纤维或碳纤维（不限长度和重量）",
            "握杆位置: 无限制",
            "失败判定: 横杆掉落、身体触垫、手碰横杆后掉杆",
            "助跑限制: 无限制（通常40-50米）",
            "时间限制: 1分钟内完成试跳（最后阶段2分钟）",
            "插斗规格: 1米长，斜面角度约105度",
            "落地区: 泡沫垫，长6米，宽6米",
        ]
    }

    /// 跳远规则
    pub fn long_jump_rules(&self) -> Vec<&'static str> {
        vec![
            "助跑长度: 最长40米，最短无限制",
            "起跳板宽度: 20厘米（白色标记）",
            "落地区宽度: 2.75-3米",
            "落地区长度: 至少10米",
            "试跳次数: 预赛3次，前8名决赛3次",
            "成功试跳: 起跳时未踩线",
            "失败判定: 踩线、起跳板外起跳、落地后向后走",
            "测量: 起跳线前沿到最近落地点",
            "风速限制: 超过2.0米/秒不承认纪录",
            "时间限制: 1分钟内完成试跳",
        ]
    }

    /// 三级跳远规则
    pub fn triple_jump_rules(&self) -> Vec<&'static str> {
        vec![
            "三跳顺序: 单脚跳 → 跨步跳 → 跳跃",
            "起跳板距离（男）: 11米、13米（可选）",
            "起跳板距离（女）: 9米、11米（可选）",
            "落地区宽度: 2.75-3米",
            "第一跳: 同一脚起跳和落地",
            "第二跳: 另一脚落地",
            "第三跳: 双脚落地",
            "失败判定: 踩线、起跳板外起跳、落地后向后走",
            "测量: 起跳线前沿到最近落地点",
            "风速限制: 超过2.0米/秒不承认纪录",
        ]
    }

    /// 投掷项目通用规则
    pub fn throwing_general_rules(&self) -> Vec<&'static str> {
        vec![
            "试投次数: 预赛3次，前8名决赛3次",
            "有效试投: 在投掷圈内完成，器械落在扇形区内",
            "投掷时间: 1分钟内完成",
            "出投掷圈: 必须从后半圆出圈",
            "触圈外地面: 判为失败",
            "测量: 投掷圈中心到最近落地点",
            "犯规判定: 触碰投掷圈上沿、踩踏抵趾板外沿",
            "器械规格: 必须符合WA认证标准",
        ]
    }

    /// 铅球规则
    pub fn shot_put_rules(&self) -> Vec<&'static str> {
        vec![
            "男子铅球重量: 7.26公斤",
            "女子铅球重量: 4.00公斤",
            "投掷圈直径: 2.135米",
            "落地区角度: 40度扇形区",
            "推球姿势: 必须从肩部推出（不得抛掷）",
            "推球方式: 滑步推球或旋转推球",
            "抵趾板高度: 10厘米",
            "抵趾板宽度: 11.2-30厘米",
            "起止位置: 必须从静止姿势开始",
        ]
    }

    /// 铁饼规则
    pub fn discus_rules(&self) -> Vec<&'static str> {
        vec![
            "男子铁饼重量: 2.00公斤",
            "女子铁饼重量: 1.00公斤",
            "投掷圈直径: 2.50米",
            "落地区角度: 40度扇形区",
            "投掷方式: 旋转投掷（1.5-2圈）",
            "起止位置: 必须从静止姿势开始",
            "安全笼: 高4米，开口宽度6米",
            "铁饼直径: 219-221毫米（男），180-182毫米（女）",
        ]
    }

    /// 标枪规则
    pub fn javelin_rules(&self) -> Vec<&'static str> {
        vec![
            "男子标枪重量: 800克",
            "女子标枪重量: 600克",
            "助跑长度: 无限制（通常25-35米）",
            "起止区域: 至少4米宽，长度不限",
            "投掷方式: 只能单手握枪",
            "落地判定: 枪尖先着地为有效",
            "枪身角度: 落地时枪尖与地面夹角影响测量",
            "落地区角度: 29度扇形区",
            "标枪长度: 男子260-270厘米，女子220-230厘米",
            "新规则标枪: 1986年新规格（重心前移）",
        ]
    }

    /// 链球规则
    pub fn hammer_rules(&self) -> Vec<&'static str> {
        vec![
            "男子链球重量: 7.26公斤",
            "女子链球重量: 4.00公斤",
            "投掷圈直径: 2.135米",
            "落地区角度: 40度扇形区",
            "投掷方式: 旋转投掷（3-4圈）",
            "起止位置: 必须从静止姿势开始",
            "安全笼: 高7米，开口宽度6米",
            "链球总长度: 男子117.5-121.5厘米，女子116-119.5厘米",
            "握法: 使用手套，双手握住把手",
        ]
    }

    /// 全能比赛规则
    pub fn combined_events_rules(&self) -> Vec<&'static str> {
        vec![
            "十项全能（男）: 2天10项（5+5）",
            "七项全能（女）: 2天7项（4+3）",
            "评分系统: 根据成绩换算积分",
            "积分表: WA国际评分表",
            "名次判定: 总积分高者获胜",
            "积分相等: 按优胜项目数判定",
            "项目顺序: 固定顺序，不得更改",
            "间隔时间: 项目间至少30分钟休息",
            "犯规处理: 某项目犯规可获得基本分",
            "弃权: 某项目弃权则无法继续比赛",
        ]
    }

    /// 十项全能项目顺序
    pub fn decathlon_events(&self) -> Vec<&'static str> {
        vec![
            "第一天: 100米、跳远、铅球、跳高、400米",
            "第二天: 110米栏、铁饼、撑杆跳高、标枪、1500米",
        ]
    }

    /// 七项全能项目顺序
    pub fn heptathlon_events(&self) -> Vec<&'static str> {
        vec![
            "第一天: 100米栏、跳高、铅球、200米",
            "第二天: 跳远、标枪、800米",
        ]
    }

    /// 竞走规则
    pub fn race_walking_rules(&self) -> Vec<&'static str> {
        vec![
            "定义: 必须保持与地面持续接触",
            "支撑腿: 前腿从着地到垂直必须伸直",
            "裁判: 至少6名裁判（包括主裁判）",
            "警告（红卡）: 可见违规",
            "取消资格: 3张红卡",
            "距离项目: 20公里、35公里",
            "场地竞走: 20公里、50公里（已取消）",
            "补给站: 每圈设置",
            "时间限制: 无具体限制（但有时间上限）",
        ]
    }

    /// 马拉松规则
    pub fn marathon_rules(&self) -> Vec<&'static str> {
        vec![
            "标准距离: 42.195公里",
            "赛道类型: 公路跑（城市道路）",
            "起点和终点: 可在不同地点",
            "测量方法: 自行车测量法（最短路线）",
            "补给站: 每5公里设置（饮用水和运动饮料）",
            "领跑员: 禁止使用",
            "配速员: 在终点前可离开",
            "计时: 净计时和枪声计时",
            "参赛资格: 需达到报名标准（精英赛事）",
            "关门时间: 各赛段有时间限制",
        ]
    }

    /// 越野跑规则
    pub fn cross_country_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离: 4公里、8公里、12公里",
            "地形: 草地、泥地、山地",
            "个人赛和团体赛",
            "团体名次: 前4名选手积分相加",
            "装备: 鞋钉长度不超过9毫米",
            "标志: 赛道需清晰标志",
            "障碍: 可设置人工障碍",
            "起跑: 站立式起跑",
        ]
    }

    /// 室内田径规则
    pub fn indoor_rules(&self) -> Vec<&'static str> {
        vec![
            "跑道长度: 200米（标准）",
            "弯道坡度: 最大18度",
            "直道长度: 60米冲刺跑道",
            "分道跑: 所有径赛全程分道",
            "项目限制: 无10000米、马拉松等",
            "铅球: 使用不同规格",
            "跳远/三级跳: 助跑距离较短",
            "全能: 男子七项全能、女子五项全能",
        ]
    }

    /// 残疾人田径规则
    pub fn para_athletics_rules(&self) -> Vec<&'static str> {
        vec![
            "分级制度: T（径赛）、F（田赛）",
            "轮椅竞速: T32-34、T51-54级",
            "截肢运动员: T42-47级",
            "视力障碍: T11-13级",
            "脑瘫运动员: T32-38级",
            "轮椅规格: 符合IPC认证",
            "假肢规格: 需符合技术标准",
            "领跑员: 视力障碍运动员可使用领跑员",
            "声音信号: 视力障碍运动员可使用声音引导",
        ]
    }

    /// 赛风赛纪
    pub fn competition_conduct(&self) -> Vec<&'static str> {
        vec![
            "运动员着装: 整洁、符合WA规定",
            "号码布: 必须佩戴清晰可见",
            "广告: 符合WA广告规定",
            "不端行为: 取消比赛资格",
            "拒绝领奖: 取消成绩和奖金",
            "技术会议: 必须参加",
            "检录时间: 提前到场检录",
            "热身区域: 指定区域热身",
        ]
    }

    /// 禁药规定
    pub fn anti_doping_rules(&self) -> Vec<&'static str> {
        vec![
            "遵守WADA反兴奋剂条例",
            "禁药清单: WADA年度发布",
            "检测: 赛内和赛外均可检测",
            "治疗用药豁免: 需提前申请TUE",
            "行踪申报: 精英运动员需申报行踪",
            "违规处罚: 禁赛2-4年",
            "申诉程序: CAS仲裁",
            "恢复治疗: PRP等需符合规定",
        ]
    }

    /// 世界纪录认定
    pub fn world_record_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛级别: 需为WA认证赛事",
            "计时系统: 电子计时（100米-10000米）",
            "风速测量: 径赛需风速记录",
            "风速限制: 不超过2.0米/秒",
            "场地认证: 符合WA一级场地标准",
            "器械认证: 使用WA认证器械",
            "申请程序: 赛后提交申请材料",
            "审核时间: WA技术委员会审核",
        ]
    }

    /// 年龄组别
    pub fn age_groups(&self) -> Vec<&'static str> {
        vec![
            "U18: 16-17岁（原青年）",
            "U20: 18-19岁（原少年）",
            "成年: 20岁以上",
            "大师赛: 35岁以上（每5岁一组）",
            "年龄限制: 各赛事有具体规定",
            "青年纪录: U18和U20纪录",
            "大师纪录: 分年龄组纪录",
        ]
    }
}

impl Default for AthleticsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AthleticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("athletics_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【田径详细规则】\n\
            世界田联(World Athletics)标准规则\n\n\
            跑道规格:\n{}\n\n\
            起跑规则:\n{}\n\n\
            接力规则:\n{}\n\n\
            跨栏规格:\n{}\n\n\
            跳高规则:\n{}\n\n\
            跳远规则:\n{}\n\n\
            投掷规则:\n{}\n\n\
            全能规则:\n{}\n",
            self.track_specifications()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.starting_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.relay_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hurdles_specifications()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.high_jump_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.long_jump_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.throwing_general_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.combined_events_rules()
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
    fn test_athletics_detailed_rules_creation() {
        let rules = AthleticsDetailedRules::new();
        assert_eq!(rules.metadata().name, "田径详细规则");
    }

    #[test]
    fn test_track_event_properties() {
        assert!(TrackEvent::Sprint100m.requires_starting_blocks());
        assert!(!TrackEvent::Marathon.requires_starting_blocks());
        assert!(TrackEvent::Sprint100m.uses_lanes());
        assert!(!TrackEvent::Middle1500m.uses_lanes());
    }

    #[test]
    fn test_field_event_properties() {
        assert!(FieldEvent::HighJump.is_jumping_event());
        assert!(!FieldEvent::ShotPut.is_jumping_event());
        assert!(FieldEvent::Discus.is_throwing_event());
    }

    #[test]
    fn test_combined_events() {
        assert_eq!(CombinedEvent::Decathlon.event_count(), 10);
        assert_eq!(CombinedEvent::Heptathlon.event_count(), 7);
    }

    #[test]
    fn test_track_specifications() {
        let rules = AthleticsDetailedRules::new();
        let specs = rules.track_specifications();
        assert!(!specs.is_empty());
        assert!(specs.iter().any(|s| s.contains("400米")));
    }

    #[test]
    fn test_starting_rules() {
        let rules = AthleticsDetailedRules::new();
        let rules_list = rules.starting_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("起跑器")));
    }

    #[test]
    fn test_hurdles_specifications() {
        let rules = AthleticsDetailedRules::new();
        let specs = rules.hurdles_specifications();
        assert!(!specs.is_empty());
        assert!(specs.iter().any(|s| s.contains("110米栏")));
    }

    #[test]
    fn test_explain() {
        let rules = AthleticsDetailedRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("田径详细规则"));
        assert!(explanation.contains("World Athletics"));
    }
}
