use
{
    eframe::{egui, epi},
    eframe::egui::{color::*, *},
};

pub fn oelabs_design(egui_ctx: &egui::CtxRef)
{
    fonts(egui_ctx);
    style(egui_ctx);
}

fn fonts(egui_ctx: &egui::CtxRef)
{
    let mut fonts = FontDefinitions::default();
        
    fonts.font_data.insert
    (
        "LXGWWenKai-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../fonts/LXGWWenKai-Regular.ttf")),
    );

    fonts.fonts_for_family.get_mut(&FontFamily::Monospace)
    .unwrap()
    .insert(0, "LXGWWenKai-Regular".to_owned());

    fonts.fonts_for_family.get_mut(&FontFamily::Proportional)
    .unwrap()
    .insert(0, "LXGWWenKai-Regular".to_owned());

    {   //字体大小  
        fonts.family_and_size.insert    (TextStyle::Small,      (FontFamily::Proportional, 15.0));
        fonts.family_and_size.insert    (TextStyle::Body,       (FontFamily::Proportional, 20.0));
        fonts.family_and_size.insert    (TextStyle::Button,     (FontFamily::Proportional, 20.0));
        fonts.family_and_size.insert    (TextStyle::Heading,    (FontFamily::Proportional, 25.0));
        fonts.family_and_size.insert    (TextStyle::Monospace,  (FontFamily::Monospace, 18.0));
    }   

    egui_ctx.set_fonts(fonts);
}

fn style(egui_ctx: &egui::CtxRef)
{
    let  mut style;
    {
        style = Style::default();
        {//impl Default for Style 
            // body_text_style: TextStyle::Body,
            // override_text_style: None,
                //如果设置，这将更改TextStyle所有小部件的默认值,在大多数小部件上，您还可以设置显式文本样式，这将优先于此
            // wrap: None,
                // 如果设置，标签按钮 wtc 将使用它来确定是否在Ui它们所在的右边缘换行文本。默认情况下，这是None ;Some<bool>
            // spacing: Spacing::default(),
                //小部件之间的大小和距离
            // interaction: Interaction::default(),
                //交互发生的方式和时间
            // visuals: Visuals::default(),
                //颜色等
            // animation_time: 1.0 / 12.0,
                //一个典型的动画应该持续多少秒
            // debug: Default::default(),
        }

        style.animation_time = 0.3;
        let  (mut spacing,mut visuals);
        {
            spacing = style::Spacing::default();
            {
                spacing.item_spacing        =   vec2(20.0, 10.0);     //  vec2(8.0, 3.0)  // 所有物件的 横向和纵向间距
                spacing.window_padding      =   vec2(10.0, 5.0);     //  Vec2::splat(6.0)  // 窗口的边框宽度
                spacing.button_padding      =   vec2(10.0, 5.0);    //  vec2(4.0, 1.0)  // 按钮与文字的间隔长度
                spacing.interact_size       =   vec2(80.0, 30.0);   //  vec2(40.0, 18.0)  //整个交互的高度 ，需要大于按钮的总高度 （间隔+文字
                spacing.indent              =   15.0;   //  18.0
                spacing.slider_width        =   500.0;  //  100.0
                spacing.text_edit_width     =   500.0;  //  280.0
                spacing.scroll_bar_width    =   25.0;   //  8.0
                spacing.icon_width          =   20.0;   //  14.0    <!!> check box etc . width
                spacing.icon_spacing        =   10.0;   //  0.0     <!!> check box etc . spacing
                //spacing.tooltip_width       =   600.0;            //  600
                // spacing.indent_ends_with_horizontal_line = false; //false
                style.spacing   =   spacing;         
            }

            visuals = style::Visuals::default();
            {
                visuals.dark_mode =   false;
                // visuals.override_text_color =  None, // Option<Color32>,
                {//background colors,另外两个在widgets
                    visuals.faint_bg_color              =   Color32::from_rgb(23, 22, 21);  //  Color32::from_gray(24)  
                    visuals.extreme_bg_color            =   Color32::from_rgb(35, 34, 31);  //  Color32::from_gray(10)
                }
                {//window, 颜色 & bg_stroke 在widgets里
                    visuals.window_corner_radius        =   0.0;                            //  6.0,
                    visuals.window_shadow               =   epaint::Shadow{extrusion:32.0,color:Color32::new(8,15,15,88)};  //  Shadow::big_dark()
                    visuals.popup_shadow                =   epaint::Shadow{extrusion:5.0,color:Color32::new(52,51,35,73)};  //  small_dark()
                }
                {//最后几个
                    visuals.code_bg_color               =   Color32::from_gray(28);             //  Color32::from_gray(64)
                    visuals.hyperlink_color             =   Color32::from_rgb(160, 90, 168);    //  Color32::from_rgb(90, 170, 255)   
                    visuals.collapsing_header_frame = true;  //false                          
                    // resize_corner_size: 12.0,
                    // text_cursor_width: 2.0, // 
                    // text_cursor_preview: false,   // preview text cursor on hover                 
                    // clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
                    // button_frame: true,
                }

                
                let (mut widgets, mut selection);
                {
                    {
                        // noninteractive: WidgetVisuals {
                        //     bg_fill: Color32::from_gray(27), // window background
                        //     bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines, windows outlines
                        //     fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // normal text color
                        //     corner_radius: 2.0,
                        //     expansion: 0.0,
                        // },
                        // inactive: WidgetVisuals {
                        //     bg_fill: Color32::from_gray(60), // button background
                        //     bg_stroke: Default::default(),
                        //     fg_stroke: Stroke::new(1.0, Color32::from_gray(180)), // button text
                        //     corner_radius: 2.0,
                        //     expansion: 0.0,
                        // },
                        // hovered: WidgetVisuals {
                        //     bg_fill: Color32::from_gray(70),
                        //     bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
                        //     fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                        //     corner_radius: 3.0,
                        //     expansion: 1.0,
                        // },
                        // active: WidgetVisuals {
                        //     bg_fill: Color32::from_gray(55),
                        //     bg_stroke: Stroke::new(1.0, Color32::WHITE),
                        //     fg_stroke: Stroke::new(2.0, Color32::WHITE),
                        //     corner_radius: 2.0,
                        //     expansion: 1.0,
                        // },
                        // open: WidgetVisuals {
                        //     bg_fill: Color32::from_gray(27),
                        //     bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                        //     fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                        //     corner_radius: 2.0,
                        //     expansion: 0.0,
                        // },
                    }
                    {
                        let noninteractive :style::WidgetVisuals;
                        let (inactive,hovered,active,open);                            
                        {
                            noninteractive    =   style::WidgetVisuals
                            {
                                bg_fill: Color32::new(17,17,17,255), // window background
                                bg_stroke: Stroke::new(1.0, Color32::from_gray(66)), // separators, indentation lines, windows outlines
                                fg_stroke: Stroke::new(2.0, Color32::from_gray(154)), // normal text color
                                corner_radius: 0.0,
                                expansion: 4.0,
                            };
                            inactive    =   style::WidgetVisuals
                            {
                                bg_fill: Color32::new(27,38,43,255),
                                fg_stroke: Stroke::new(2.0, Color32::new(169,148,116,255)),
                                .. noninteractive
                            };
                            hovered     =   style::WidgetVisuals
                            {
                                bg_fill: Color32::new(45,108,113,255),
                                bg_stroke: Stroke::new(0.0, Color32::new(0,0,0,0)), // e.g. hover over window edge or button
                                fg_stroke: Stroke::new(2.0, Color32::new(151,222,235,255)),
                                corner_radius: 8.0,
                                expansion: noninteractive.expansion + 4.0,
                            };
                            active      =   style::WidgetVisuals
                            {
                                bg_fill: Color32::new(56,138,61,255),
                                bg_stroke: Stroke::new(2.0, Color32::new(181,222,223,255)), // e.g. hover over window edge or button
                                corner_radius: 20.0,
                                .. hovered
                            };
                            open        =   style::WidgetVisuals
                            {
                                bg_fill: Color32::new(56,138,61,255),
                                bg_stroke: Stroke::new(2.0, Color32::new(181,222,223,255)),
                                fg_stroke: Stroke::new(2.0, Color32::new(151,222,235,255)),
                                corner_radius: 2.0,
                                .. noninteractive
                            }
                        }                       
                        widgets = style::Widgets{noninteractive,inactive,hovered,active,open};
                    }
                    selection   =   style::Selection::default();
                    {
                        selection.bg_fill   =   Color32::new(56,138,61,255);
                        selection.stroke    =   Stroke::new(2.0, Color32::new(151,222,235,255))
                    }
                    visuals.widgets =   widgets;
                    visuals.selection   =   selection;
                    
                    style.visuals       =   visuals; 
                }
            }


        }
        egui_ctx.set_style(style);
    }
}