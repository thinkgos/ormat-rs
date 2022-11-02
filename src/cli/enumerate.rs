use anyhow::{anyhow, Result};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde_json::json;

use crate::ast::EnumField;

pub fn run_enumerate() -> Result<()> {
    let show_prompt = || {
        println!(
            "{}\n{}\n",
            "枚举输入格式(以逗号分隔): id,name,comment".green(),
            r"    id<number>: 必填
    name<string>: 必填
    name<string>: 可选"
                .yellow()
        );
    };

    show_prompt();

    let mut enum_fields: Vec<EnumField> = vec![];
    loop {
        let field: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("请输入枚举项<id,name,comment>:")
            .interact_text()?;

        let field = match parse_enum_field(&field) {
            Ok(field) => field,
            Err(_e) => {
                println!("无效的枚举项, 请检查!!!");
                show_prompt();
                continue;
            }
        };

        enum_fields.push(field);
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("是否还要继续输入?")
            .default(true)
            .show_default(true)
            .interact()?
        {
            break;
        }
    }
    if !enum_fields.is_empty() {
        println!("{}", into_enum_annotation(&enum_fields)?);
    }
    Ok(())
}

fn parse_enum_field(s: &str) -> Result<EnumField> {
    let fields: Vec<_> = s.splitn(3, ",").collect();

    if fields.len() < 2 {
        return Err(anyhow!("id和name是必填项"));
    }
    let mut fields = fields.into_iter();

    let id = fields.next().unwrap_or_default().trim();
    let name = fields.next().unwrap_or_default().trim().to_string();
    if name.is_empty() {
        return Err(anyhow!("name不能为空"));
    }
    let comment = fields.next().map(|v| v.trim()).unwrap_or_default();
    let comment = if comment.is_empty() {
        None
    } else {
        Some(comment.to_owned())
    };

    let id: isize = id.parse()?;

    Ok(EnumField { id, name, comment })
}

fn into_enum_annotation(v: &[EnumField]) -> Result<String> {
    let mut json_value = json!({});
    v.into_iter().for_each(|v| {
        if v.comment.is_some() {
            json_value[v.id.to_string()] = json!([v.name, v.comment]);
        } else {
            json_value[v.id.to_string()] = json!([v.name]);
        }
    });

    Ok(format!("[@enum:{}]", serde_json::to_string(&json_value)?))
}
