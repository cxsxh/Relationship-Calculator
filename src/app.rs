use std::fs;
use std::path::Path;

use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, RichText, Stroke};
use eframe::{App, CreationContext, Frame};

use crate::data::relations::RELATION_DEFINITIONS;
use crate::engine::calculator::{RelationCalculator, MAX_STEPS};
use crate::models::{RelationResult, RelationType};

pub struct RelationshipCalculatorApp {
    calculator: RelationCalculator,
    selected_relations: Vec<RelationType>,
    picker: RelationType,
    result: RelationResult,
    status_message: String,
}

impl RelationshipCalculatorApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        Self {
            calculator: RelationCalculator::new(),
            selected_relations: Vec::new(),
            picker: RelationType::Mother,
            result: RelationResult::prompt(),
            status_message: format!("支持最多 {MAX_STEPS} 步关系链，全部本地计算。"),
        }
    }

    fn recalculate(&mut self) {
        self.result = self.calculator.calculate(&self.selected_relations);
    }

    fn add_relation(&mut self, relation: RelationType) {
        if self.selected_relations.len() >= MAX_STEPS {
            self.status_message = format!("最多只能添加 {MAX_STEPS} 步关系。");
            return;
        }

        self.selected_relations.push(relation);
        self.status_message = format!("已添加：{}。", relation.label());
        self.recalculate();
    }

    fn undo_last(&mut self) {
        if let Some(relation) = self.selected_relations.pop() {
            self.status_message = format!("已移除最后一步：{}。", relation.label());
            self.recalculate();
        } else {
            self.status_message = "当前没有可撤销的关系步骤。".to_owned();
        }
    }

    fn clear_all(&mut self) {
        self.selected_relations.clear();
        self.result = RelationResult::prompt();
        self.status_message = "已清空全部关系步骤。".to_owned();
    }

    fn copy_result(&mut self, ctx: &egui::Context) {
        ctx.copy_text(self.result.to_clipboard_text());
        self.status_message = "结果已复制到剪贴板。".to_owned();
    }

    fn draw_header(&self, ui: &mut egui::Ui) {
        ui.heading(
            RichText::new("关系计算器")
                .size(28.0)
                .color(Color32::from_rgb(30, 55, 110)),
        );
        ui.label(RichText::new("Rust 原生桌面版 · 干净简洁 · 本地离线计算").size(14.0));
        ui.add_space(6.0);
        ui.label("选择关系链后即可得到标准称呼和多种常见叫法。");
    }

    fn draw_chain_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("关系输入");
        ui.label("当前主体：我");
        ui.add_space(4.0);

        ui.label(RichText::new("当前关系链").strong());
        ui.horizontal_wrapped(|ui| {
            if self.selected_relations.is_empty() {
                ui.label(RichText::new("暂无关系步骤").italics().color(Color32::GRAY));
            } else {
                ui.label(chip("我", Color32::from_rgb(62, 99, 221)));
                for relation in &self.selected_relations {
                    ui.label(RichText::new("→").size(18.0).color(Color32::DARK_GRAY));
                    ui.label(chip(relation.label(), Color32::from_rgb(77, 132, 228)));
                }
            }
        });

        ui.add_space(12.0);
        ui.label(RichText::new("快捷添加").strong());
        egui::Grid::new("quick_relation_grid")
            .num_columns(2)
            .spacing([12.0, 10.0])
            .show(ui, |ui| {
                for (index, item) in RELATION_DEFINITIONS.iter().enumerate() {
                    let button = egui::Button::new(
                        RichText::new(format!(
                            "{}  {}",
                            item.relation.label(),
                            item.relation.code()
                        ))
                        .size(15.0),
                    )
                    .min_size(egui::vec2(170.0, 34.0));

                    let response = ui.add(button).on_hover_text(format!(
                        "{} · {}",
                        item.relation.hint(),
                        item.description
                    ));
                    if response.clicked() {
                        self.add_relation(item.relation);
                    }

                    if index % 2 == 1 {
                        ui.end_row();
                    }
                }
            });

        ui.add_space(12.0);
        ui.label(RichText::new("下拉选择").strong());
        egui::ComboBox::from_id_salt("relation_picker")
            .selected_text(self.picker.label())
            .width(180.0)
            .show_ui(ui, |ui| {
                for relation in RelationType::ALL {
                    ui.selectable_value(&mut self.picker, relation, relation.label());
                }
            });

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui
                .add_sized([108.0, 34.0], egui::Button::new("添加步骤"))
                .clicked()
            {
                self.add_relation(self.picker);
            }

            if ui
                .add_sized([108.0, 34.0], egui::Button::new("撤销一步"))
                .clicked()
            {
                self.undo_last();
            }

            if ui
                .add_sized([108.0, 34.0], egui::Button::new("清空全部"))
                .clicked()
            {
                self.clear_all();
            }
        });

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui
                .add_sized([108.0, 34.0], egui::Button::new("重新计算"))
                .clicked()
            {
                self.recalculate();
                self.status_message = "已完成重新计算。".to_owned();
            }

            if ui
                .add_sized([108.0, 34.0], egui::Button::new("复制结果"))
                .clicked()
            {
                self.copy_result(ui.ctx());
            }
        });

        ui.add_space(10.0);
        ui.label(RichText::new(&self.status_message).color(Color32::from_rgb(36, 77, 150)));
    }

    fn draw_result_panel(&self, ui: &mut egui::Ui) {
        ui.heading("计算结果");
        ui.add_space(4.0);

        let frame = egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(250, 252, 255))
            .stroke(Stroke::new(1.0, Color32::from_rgb(210, 220, 242)))
            .inner_margin(egui::Margin::same(16.0));

        frame.show(ui, |ui| {
            let title_color = if self.result.is_matched {
                Color32::from_rgb(24, 114, 52)
            } else {
                Color32::from_rgb(156, 79, 18)
            };

            ui.label(RichText::new("标准称呼").strong().size(16.0));
            ui.add_space(4.0);
            ui.label(
                RichText::new(&self.result.standard_title)
                    .size(34.0)
                    .color(title_color),
            );

            ui.add_space(14.0);
            ui.label(RichText::new("常见叫法").strong());
            ui.horizontal_wrapped(|ui| {
                if self.result.common_aliases.is_empty() {
                    ui.label(RichText::new("暂无").italics().color(Color32::GRAY));
                } else {
                    for alias in &self.result.common_aliases {
                        ui.label(chip(alias, Color32::from_rgb(53, 121, 197)));
                    }
                }
            });

            ui.add_space(14.0);
            ui.label(RichText::new("方言叫法").strong());
            ui.horizontal_wrapped(|ui| {
                if self.result.dialect_aliases.is_empty() {
                    ui.label(RichText::new("暂无").italics().color(Color32::GRAY));
                } else {
                    for alias in &self.result.dialect_aliases {
                        ui.label(chip(alias, Color32::from_rgb(126, 88, 192)));
                    }
                }
            });

            ui.add_space(14.0);
            ui.label(RichText::new("中性称呼").strong());
            match &self.result.neutral_title {
                Some(title) => ui.label(chip(title, Color32::from_rgb(89, 138, 92))),
                None => ui.label(RichText::new("暂无").italics().color(Color32::GRAY)),
            };

            ui.add_space(14.0);
            ui.label(RichText::new("关系路径").strong());
            ui.label(&self.result.relation_path_text);

            ui.add_space(10.0);
            ui.label(RichText::new("关系编码").strong());
            ui.monospace(&self.result.code_path_text);

            ui.add_space(10.0);
            ui.label(RichText::new("说明").strong());
            ui.label(&self.result.message);
        });

        ui.add_space(12.0);
        ui.heading("设计原则");
        ui.label("安全：全部逻辑本地执行，不联网，不运行外部脚本。");
        ui.label("性能：规则映射在内存中完成，毫秒级返回结果。");
        ui.label("可扩展：数据和界面解耦，便于继续补充更多亲属规则。");
    }
}

impl App for RelationshipCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(243, 246, 252))
                    .inner_margin(egui::Margin::same(18.0)),
            )
            .show(ctx, |ui| {
                self.draw_header(ui);
                ui.add_space(18.0);

                ui.columns(2, |columns| {
                    self.draw_chain_panel(&mut columns[0]);
                    self.draw_result_panel(&mut columns[1]);
                });
            });
    }
}

fn chip(text: &str, color: Color32) -> RichText {
    RichText::new(format!("  {text}  "))
        .color(Color32::WHITE)
        .background_color(color)
        .size(14.0)
}

fn configure_fonts(ctx: &egui::Context) {
    let candidates = [
        r"C:\Windows\Fonts\msyh.ttc",
        r"C:\Windows\Fonts\msyh.ttf",
        r"C:\Windows\Fonts\simhei.ttf",
        r"C:\Windows\Fonts\simsun.ttc",
    ];

    for path in candidates {
        if let Ok(bytes) = fs::read(Path::new(path)) {
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "system-chinese".to_owned(),
                FontData::from_owned(bytes).into(),
            );

            if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
                family.insert(0, "system-chinese".to_owned());
            }
            if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
                family.push("system-chinese".to_owned());
            }

            ctx.set_fonts(fonts);
            break;
        }
    }
}
