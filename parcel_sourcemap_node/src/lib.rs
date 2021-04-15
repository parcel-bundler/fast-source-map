extern crate napi;
#[macro_use]
extern crate napi_derive;
extern crate parcel_sourcemap;

use napi::{
    CallContext, Either, Env, JsBuffer, JsNull, JsNumber, JsObject, JsString, JsUndefined,
    Property, Result,
};
use parcel_sourcemap::{Mapping, OriginalLocation, SourceMap};

#[js_function(1)]
fn add_source(ctx: CallContext) -> Result<JsNumber> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let source = ctx.get::<JsString>(0)?.into_utf8()?;
    let source_index = source_map_instance.add_source(source.as_str()?);

    return ctx.env.create_uint32(source_index);
}

#[js_function(1)]
fn get_source(ctx: CallContext) -> Result<JsString> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let source_index = ctx.get::<JsNumber>(0)?.get_uint32()?;
    match source_map_instance.get_source(source_index) {
        Ok(source) => {
            return ctx.env.create_string(source);
        }
        Err(_err) => {
            return ctx.env.create_string("");
        }
    }
}

fn _get_sources(ctx: &CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let mut napi_sources_array = ctx
        .env
        .create_array_with_length(source_map_instance.sources.len())?;
    for (source_index, source) in source_map_instance.sources.iter().enumerate() {
        napi_sources_array.set_element(source_index as u32, ctx.env.create_string(&source[..])?)?;
    }

    // Return array
    return Ok(napi_sources_array);
}

#[js_function]
fn get_sources(ctx: CallContext) -> Result<JsObject> {
    return _get_sources(&ctx);
}

fn _get_sources_content(ctx: &CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let mut napi_sources_content_array = ctx
        .env
        .create_array_with_length(source_map_instance.sources_content.len())?;
    for (source_index, source_content) in source_map_instance.sources_content.iter().enumerate() {
        napi_sources_content_array.set_element(
            source_index as u32,
            ctx.env.create_string(&source_content[..])?,
        )?;
    }

    // Return array
    return Ok(napi_sources_content_array);
}

#[js_function]
fn get_sources_content(ctx: CallContext) -> Result<JsObject> {
    return _get_sources_content(&ctx);
}

#[js_function(1)]
fn get_source_index(ctx: CallContext) -> Result<JsNumber> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let source = ctx.get::<JsString>(0)?.into_utf8()?;
    let source_index = source_map_instance.get_source_index(source.as_str()?)?;

    match source_index {
        Some(i) => {
            return ctx.env.create_uint32(i);
        }
        None => {
            return ctx.env.create_int32(-1);
        }
    }
}

#[js_function(2)]
fn set_source_content_by_source(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let source = ctx.get::<JsString>(0)?.into_utf8()?;
    let source_index: usize = source_map_instance.add_source(source.as_str()?) as usize;
    let source_content = ctx.get::<JsString>(1)?.into_utf8()?;
    source_map_instance.set_source_content(source_index, source_content.as_str()?)?;

    return ctx.env.get_undefined();
}

#[js_function(1)]
fn get_source_content_by_source(ctx: CallContext) -> Result<JsString> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let source = ctx.get::<JsString>(0)?.into_utf8()?;
    let source_index = source_map_instance.get_source_index(source.as_str()?)?;
    match source_index {
        Some(i) => {
            let source_content = source_map_instance.get_source_content(i)?;
            return ctx.env.create_string(source_content);
        }
        None => {
            return ctx.env.create_string("");
        }
    }
}

#[js_function(1)]
fn add_name(ctx: CallContext) -> Result<JsNumber> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let name = ctx.get::<JsString>(0)?.into_utf8()?;
    let name_index = source_map_instance.add_name(name.as_str()?);
    return ctx.env.create_uint32(name_index);
}

#[js_function(1)]
fn get_name(ctx: CallContext) -> Result<JsString> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let name_index = ctx.get::<JsNumber>(0)?.get_uint32()?;
    match source_map_instance.get_name(name_index) {
        Ok(name) => {
            return ctx.env.create_string(name);
        }
        Err(_err) => {
            return ctx.env.create_string("");
        }
    }
}

fn _get_names(ctx: &CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let mut napi_names_array = ctx
        .env
        .create_array_with_length(source_map_instance.names.len())?;
    for (name_index, name) in source_map_instance.names.iter().enumerate() {
        napi_names_array.set_element(name_index as u32, ctx.env.create_string(&name[..])?)?;
    }

    // Return array
    return Ok(napi_names_array);
}

#[js_function]
fn get_names(ctx: CallContext) -> Result<JsObject> {
    return _get_names(&ctx);
}

#[js_function(1)]
fn get_name_index(ctx: CallContext) -> Result<JsNumber> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let name = ctx.get::<JsString>(0)?.into_utf8()?;
    let name_index = source_map_instance.get_name_index(name.as_str()?);

    match name_index {
        Some(i) => {
            return ctx.env.create_uint32(i);
        }
        None => {
            return ctx.env.create_int32(-1);
        }
    }
}

fn mapping_to_js_object(ctx: &CallContext, mapping: &Mapping) -> Result<JsObject> {
    let mut mapping_obj = ctx.env.create_object()?;

    let mut generated_position_obj = ctx.env.create_object()?;
    generated_position_obj
        .set_named_property("line", ctx.env.create_uint32((mapping.generated_line) + 1)?)?;
    generated_position_obj
        .set_named_property("column", ctx.env.create_uint32(mapping.generated_column)?)?;
    mapping_obj.set_named_property("generated", generated_position_obj)?;

    let original_position = mapping.original;
    if let Some(original_position) = original_position {
        let mut original_position_obj = ctx.env.create_object()?;
        original_position_obj.set_named_property(
            "line",
            ctx.env.create_uint32(original_position.original_line + 1)?,
        )?;
        original_position_obj.set_named_property(
            "column",
            ctx.env.create_uint32(original_position.original_column)?,
        )?;
        mapping_obj.set_named_property("original", original_position_obj)?;

        mapping_obj
            .set_named_property("source", ctx.env.create_uint32(original_position.source)?)?;

        if let Some(name) = original_position.name {
            mapping_obj.set_named_property("name", ctx.env.create_uint32(name)?)?;
        }
    }

    return Ok(mapping_obj);
}

#[js_function]
fn get_mappings(ctx: CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let mut mappings_arr = ctx.env.create_array()?;
    let mut index: u32 = 0;
    for (generated_line, mapping_line) in source_map_instance.mapping_lines.iter() {
        for (generated_column, original_position) in mapping_line.mappings.iter() {
            mappings_arr.set_element(
                index,
                mapping_to_js_object(
                    &ctx,
                    &Mapping {
                        generated_line: *generated_line,
                        generated_column: *generated_column,
                        original: *original_position,
                    },
                )?,
            )?;
            index += 1;
        }
    }
    return Ok(mappings_arr);
}

#[js_function]
fn to_buffer(ctx: CallContext) -> Result<JsBuffer> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &SourceMap = ctx.env.unwrap(&this)?;

    let mut buffer_data = Vec::new();
    source_map_instance.to_buffer(&mut buffer_data)?;
    return Ok(ctx.env.create_buffer_with_data(buffer_data)?.into_raw());
}

#[js_function(3)]
fn append_sourcemap(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let sourcemap_object = ctx.get::<JsObject>(0)?;
    let previous_map_instance = ctx.env.unwrap::<SourceMap>(&sourcemap_object)?;
    let line_offset = ctx.get::<JsNumber>(1)?.get_int64()?;
    let column_offset = ctx.get::<JsNumber>(2)?.get_int64()?;

    source_map_instance.append_sourcemap(previous_map_instance, line_offset, column_offset)?;
    return ctx.env.get_undefined();
}

#[js_function(6)]
fn add_vlq_map(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let vlq_mappings = ctx.get::<JsString>(0)?.into_utf8()?;

    let js_sources_arr = ctx.get::<JsObject>(1)?;
    let js_sources_arr_len: u32 = js_sources_arr
        .get_named_property::<JsNumber>("length")?
        .get_uint32()?;
    let mut sources = Vec::with_capacity(js_sources_arr_len as usize);
    for i in 0..js_sources_arr_len {
        sources.push(
            js_sources_arr
                .get_element::<JsString>(i)?
                .into_utf8()?
                .into_owned()?,
        );
    }

    let js_sources_content_arr = ctx.get::<JsObject>(2)?;
    let js_sources_content_arr_len: u32 = js_sources_arr
        .get_named_property::<JsNumber>("length")?
        .get_uint32()?;
    let mut sources_content = Vec::with_capacity(js_sources_content_arr_len as usize);
    for i in 0..js_sources_content_arr_len {
        sources_content.push(
            js_sources_content_arr
                .get_element::<JsString>(i)?
                .into_utf8()?
                .into_owned()?,
        );
    }

    let js_names_arr = ctx.get::<JsObject>(3)?;
    let js_names_arr_len: u32 = js_names_arr
        .get_named_property::<JsNumber>("length")?
        .get_uint32()?;
    let mut names = Vec::with_capacity(js_names_arr_len as usize);
    for i in 0..js_names_arr_len {
        names.push(
            js_names_arr
                .get_element::<JsString>(i)?
                .into_utf8()?
                .into_owned()?,
        );
    }

    let line_offset = ctx.get::<JsNumber>(4)?.get_int64()?;
    let column_offset = ctx.get::<JsNumber>(5)?.get_int64()?;

    source_map_instance.add_vlq_map(
        vlq_mappings.as_slice(),
        sources.iter().map(|s| &s[..]).collect(),
        sources_content.iter().map(|s| &s[..]).collect(),
        names.iter().map(|n| &n[..]).collect(),
        line_offset,
        column_offset,
    )?;

    return ctx.env.get_undefined();
}

#[js_function]
fn to_vlq(ctx: CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let mut vlq_output: Vec<u8> = vec![];
    source_map_instance.write_vlq(&mut vlq_output)?;
    let vlq_string = ctx.env.create_string_from_vec_u8(vlq_output)?;
    let mut result_obj: JsObject = ctx.env.create_object()?;
    result_obj.set_named_property("mappings", vlq_string)?;
    result_obj.set_named_property("sources", _get_sources(&ctx)?)?;
    result_obj.set_named_property("sourcesContent", _get_sources_content(&ctx)?)?;
    result_obj.set_named_property("names", _get_names(&ctx)?)?;

    return Ok(result_obj);
}

#[js_function(1)]
fn add_indexed_mappings(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    // TODO: Figure out a more optimal way of handling typed arrays...
    let js_mapping_arr = ctx.get::<JsObject>(0)?;
    let length: u32 = js_mapping_arr
        .get_named_property::<JsNumber>("length")?
        .get_uint32()?;

    let mut generated_line: u32 = 0; // 0
    let mut generated_column: u32 = 0; // 1
    let mut original_line: i32 = 0; // 2
    let mut original_column: i32 = 0; // 3
    let mut original_source: i32 = 0; // 4
    for i in 0..length {
        let value: i32 = js_mapping_arr.get_element::<JsNumber>(i)?.get_int32()?;

        match i % 6 {
            0 => {
                generated_line = value as u32;
            }
            1 => {
                generated_column = value as u32;
            }
            2 => {
                original_line = value;
            }
            3 => {
                original_column = value;
            }
            4 => {
                original_source = value;
            }
            5 => {
                source_map_instance.add_mapping(
                    generated_line,
                    generated_column,
                    if original_line > -1 && original_column > -1 && original_source > -1 {
                        Some(OriginalLocation {
                            original_line: original_line as u32,
                            original_column: original_column as u32,
                            source: original_source as u32,
                            name: if value > -1 { Some(value as u32) } else { None },
                        })
                    } else {
                        None
                    },
                );
            }
            // This is a rust bug? i % 6 can never return anything else...
            _ => (),
        }
    }

    return ctx.env.get_undefined();
}

#[js_function(2)]
fn offset_lines(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let generated_line = ctx.get::<JsNumber>(0)?.get_uint32()?;
    let generated_line_offset = ctx.get::<JsNumber>(1)?.get_int64()?;
    source_map_instance.offset_lines(generated_line, generated_line_offset)?;
    return ctx.env.get_undefined();
}

#[js_function(3)]
fn offset_columns(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let generated_line = ctx.get::<JsNumber>(0)?.get_uint32()?;
    let generated_column = ctx.get::<JsNumber>(1)?.get_uint32()?;
    let generated_column_offset = ctx.get::<JsNumber>(2)?.get_int64()?;

    source_map_instance.offset_columns(
        generated_line,
        generated_column,
        generated_column_offset,
    )?;
    return ctx.env.get_undefined();
}

#[js_function(3)]
fn add_empty_map(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let source = ctx.get::<JsString>(0)?.into_utf8()?;
    let source_content = ctx.get::<JsString>(1)?.into_utf8()?;
    let line_offset = ctx.get::<JsNumber>(2)?.get_int64()?;
    source_map_instance.add_empty_map(source.as_str()?, source_content.as_str()?, line_offset)?;
    return ctx.env.get_undefined();
}

#[js_function(1)]
fn extends_buffer(ctx: CallContext) -> Result<JsUndefined> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let map_buffer = ctx.get::<JsBuffer>(0)?.into_value()?;

    source_map_instance.extends_buffer(&map_buffer[..])?;
    return ctx.env.get_undefined();
}

#[js_function(2)]
fn find_closest_mapping(ctx: CallContext) -> Result<Either<JsObject, JsNull>> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    let generated_line = ctx.get::<JsNumber>(0)?.get_uint32()?;
    let generated_column = ctx.get::<JsNumber>(1)?.get_uint32()?;
    match source_map_instance.find_closest_mapping(generated_line, generated_column) {
        Some(mapping) => {
            return mapping_to_js_object(&ctx, &mapping).map(Either::A);
        }
        None => {
            return ctx.env.get_null().map(Either::B);
        }
    }
}

#[js_function]
fn get_project_root(ctx: CallContext) -> Result<JsString> {
    let this: JsObject = ctx.this_unchecked();
    let source_map_instance: &mut SourceMap = ctx.env.unwrap(&this)?;

    return ctx
        .env
        .create_string(source_map_instance.project_root.as_str());
}

#[js_function(1)]
fn constructor(ctx: CallContext) -> Result<JsObject> {
    let mut this: JsObject = ctx.this_unchecked();
    let second_argument = ctx.get::<Either<JsBuffer, JsString>>(0)?;
    match second_argument {
        Either::A(js_buffer) => {
            let buffer = js_buffer.into_value()?;
            let sourcemap = SourceMap::from_buffer(&buffer[..])?;
            ctx.env.wrap(&mut this, sourcemap)?;
        }
        Either::B(js_string) => {
            let project_root = js_string.into_utf8()?;
            ctx.env
                .wrap(&mut this, SourceMap::new(project_root.as_str()?))?;
        }
    }
    return Ok(this);
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
    let add_source_method = Property::new(&env, "addSource")?.with_method(add_source);
    let get_source_method = Property::new(&env, "getSource")?.with_method(get_source);
    let get_sources_method = Property::new(&env, "getSources")?.with_method(get_sources);
    let get_source_index_method =
        Property::new(&env, "getSourceIndex")?.with_method(get_source_index);
    let set_source_content_by_source_method =
        Property::new(&env, "setSourceContentBySource")?.with_method(set_source_content_by_source);
    let get_source_content_by_source_method =
        Property::new(&env, "getSourceContentBySource")?.with_method(get_source_content_by_source);
    let get_sources_content_method =
        Property::new(&env, "getSourcesContent")?.with_method(get_sources_content);
    let add_name_method = Property::new(&env, "addName")?.with_method(add_name);
    let get_name_method = Property::new(&env, "getName")?.with_method(get_name);
    let get_names_method = Property::new(&env, "getNames")?.with_method(get_names);
    let get_name_index_method = Property::new(&env, "getNameIndex")?.with_method(get_name_index);
    let get_mappings_method = Property::new(&env, "getMappings")?.with_method(get_mappings);
    let to_buffer_method = Property::new(&env, "toBuffer")?.with_method(to_buffer);
    let append_sourcemap_method =
        Property::new(&env, "appendSourcemap")?.with_method(append_sourcemap);
    let add_indexed_mappings_method =
        Property::new(&env, "addIndexedMappings")?.with_method(add_indexed_mappings);
    let add_vlq_map_method = Property::new(&env, "addVLQMap")?.with_method(add_vlq_map);
    let to_vlq_method = Property::new(&env, "toVLQ")?.with_method(to_vlq);
    let offset_lines_method = Property::new(&env, "offsetLines")?.with_method(offset_lines);
    let offset_columns_method = Property::new(&env, "offsetColumns")?.with_method(offset_columns);
    let add_empty_map_method = Property::new(&env, "addEmptyMap")?.with_method(add_empty_map);
    let extends_buffer_method = Property::new(&env, "extendsBuffer")?.with_method(extends_buffer);
    let get_project_root_method =
        Property::new(&env, "getProjectRoot")?.with_method(get_project_root);
    let find_closest_mapping_method =
        Property::new(&env, "findClosestMapping")?.with_method(find_closest_mapping);
    let sourcemap_class = env.define_class(
        "SourceMap",
        constructor,
        &[
            add_source_method,
            get_source_method,
            get_sources_method,
            get_source_index_method,
            set_source_content_by_source_method,
            get_source_content_by_source_method,
            get_sources_content_method,
            add_name_method,
            get_name_method,
            get_names_method,
            get_name_index_method,
            get_mappings_method,
            append_sourcemap_method,
            add_indexed_mappings_method,
            add_vlq_map_method,
            to_buffer_method,
            to_vlq_method,
            offset_lines_method,
            offset_columns_method,
            add_empty_map_method,
            extends_buffer_method,
            find_closest_mapping_method,
            get_project_root_method,
        ],
    )?;
    exports.set_named_property("SourceMap", sourcemap_class)?;
    return Ok(());
}
