use crate::error::object_err;
use napi::{
  CallContext, JsBuffer, JsFunction, JsNumber, JsObject, JsString, JsUndefined, JsUnknown, Result,
};
use object::{self, Object, ObjectSymbol, SectionIndex, SymbolKind};

#[derive(Serialize, Deserialize)]
struct SectionInfo {
  index: usize,
  address: u64,
  size: u64,
  align: u64,
  file_range: Option<(u64, u64)>,
  name: String,
}

// TODO other fields
#[derive(Serialize, Deserialize)]
struct ObjectSymbolInfo {
  index: usize,
  name: String,
  address: u64,
  kind: String,
}

struct FileWrapper {
  content: Vec<u8>,
}

fn kind_to_string(kind: SymbolKind) -> String {
  let kind = match kind {
    SymbolKind::Data => "Data",
    SymbolKind::File => "File",
    SymbolKind::Label => "Label",
    SymbolKind::Null => "Null",
    SymbolKind::Section => "Section",
    SymbolKind::Text => "Text",
    SymbolKind::Tls => "Tls",
    _ => "Unknown",
  };

  String::from(kind)
}

fn symbols_info(symbols: object::SymbolIterator) -> Vec<ObjectSymbolInfo> {
  let mut sb_vec: Vec<ObjectSymbolInfo> = Vec::new();

  for sb in symbols {
    let name = match sb.name().is_ok() {
      true => sb.name().unwrap(),
      false => "",
    };

    let address = sb.address();
    let kind = kind_to_string(sb.kind());
    let index = sb.index().0;

    sb_vec.push(ObjectSymbolInfo {
      index,
      name: String::from(name),
      address,
      kind,
    });
  }

  sb_vec
}

fn section_info<'a>(section: impl object::ObjectSection<'a>) -> SectionInfo {
  let index = section.index().0;
  let address = section.address();
  let size = section.size();
  let align = section.align();
  let file_range = section.file_range();
  let name = match section.name() {
    Ok(name) => name,
    Err(_e) => "",
  };

  SectionInfo {
    index,
    address,
    size,
    align,
    file_range,
    name: String::from(name),
  }
  // let index = section.index();
}

impl FileWrapper {
  fn file(&self) -> object::Result<object::File> {
    object::File::parse(&*self.content)
  }

  fn format(&self) -> object::Result<String> {
    let file = self.file()?;

    let format = match file.format() {
      object::BinaryFormat::Coff => "Coff",
      object::BinaryFormat::Elf => "Elf",
      object::BinaryFormat::MachO => "MachO",
      object::BinaryFormat::Pe => "Pe",
      object::BinaryFormat::Wasm => "Wasm",
      _ => "Unknown",
    };

    Ok(String::from(format))
  }

  fn symbols(&self) -> object::Result<Vec<ObjectSymbolInfo>> {
    let file = self.file()?;

    Ok(symbols_info(file.symbols()))
  }

  fn dynamic_symbols(&self) -> object::Result<Vec<ObjectSymbolInfo>> {
    let file = self.file()?;

    Ok(symbols_info(file.dynamic_symbols()))
  }

  fn section_by_name(&self, name: &str) -> object::Result<Option<SectionInfo>> {
    let file = self.file()?;
    let section = file.section_by_name(name);

    if section.is_none() {
      return Ok(None);
    }

    let section = section.unwrap();

    Ok(Some(section_info(section)))
  }

  fn section_by_index(&self, index: usize) -> object::Result<Option<SectionInfo>> {
    let file = self.file()?;
    let section = file.section_by_index(SectionIndex(index))?;

    Ok(Some(section_info(section)))
  }
}

fn symbols_info_js(ctx: CallContext, symbol_info_list: Vec<ObjectSymbolInfo>) -> Result<JsObject> {
  let mut arr = ctx.env.create_array()?;

  for (i, info) in symbol_info_list.iter().enumerate() {
    let index: u32 = i.try_into().map_err(|_| {
      // TODO we need a way to iterate rust values in js for better performance
      napi::Error::new(napi::Status::Unknown, String::from("too many symbols"))
    })?;
    let value = ctx.env.to_js_value::<ObjectSymbolInfo>(info)?;
    arr.set_element(index, value)?;
  }

  Ok(arr)
}

#[js_function(0)]
fn symbols(ctx: CallContext) -> Result<JsObject> {
  let this = ctx.this::<JsObject>()?;
  let wrapper = ctx.env.unwrap::<FileWrapper>(&this)?;
  let symbols = wrapper.symbols().map_err(object_err)?;

  symbols_info_js(ctx, symbols)
}

#[js_function(0)]
fn dynamic_symbols(ctx: CallContext) -> Result<JsObject> {
  let this = ctx.this::<JsObject>()?;
  let wrapper = ctx.env.unwrap::<FileWrapper>(&this)?;
  let symbols = wrapper.dynamic_symbols().map_err(object_err)?;

  symbols_info_js(ctx, symbols)
}

#[js_function(0)]
fn format(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this::<JsObject>()?;
  let wrapper = ctx.env.unwrap::<FileWrapper>(&this)?;

  let format = wrapper.format().map_err(object_err)?;

  ctx.env.create_string(&format)
}

#[js_function(1)]
fn section_by_name(ctx: CallContext) -> Result<JsUnknown> {
  let this = ctx.this::<JsObject>()?;
  let wrapper = ctx.env.unwrap::<FileWrapper>(&this)?;
  let name = ctx.get::<JsString>(0)?.into_utf8()?;

  let str = name.as_str()?;

  let info = wrapper.section_by_name(str).map_err(object_err)?;

  if info.is_none() {
    return Ok(ctx.env.get_null()?.into_unknown());
  }
  let info = info.unwrap();
  Ok(ctx.env.to_js_value(&info)?.into_unknown())
}

#[js_function(1)]
fn section_by_index(ctx: CallContext) -> Result<JsUnknown> {
  let this = ctx.this::<JsObject>()?;
  let wrapper = ctx.env.unwrap::<FileWrapper>(&this)?;
  let index = ctx.get::<JsNumber>(0)?.get_int32()?;

  let info = wrapper
    .section_by_index(index as usize)
    .map_err(object_err)?;
  if info.is_none() {
    return Ok(ctx.env.get_null()?.into_unknown());
  }
  let info = info.unwrap();
  Ok(ctx.env.to_js_value(&info)?.into_unknown())
}

#[js_function(1)]
fn file_constructor(ctx: CallContext) -> Result<JsUndefined> {
  // TODO Here is a assertion abort if the argument is not a Node.js buffer.
  // I would like to see it throw a js error instead.
  let content = ctx.get::<JsBuffer>(0)?.into_value()?;

  let wrapper = FileWrapper {
    content: Vec::from(content.as_ref()),
  };

  let mut this = ctx.this_unchecked::<JsObject>();

  ctx.env.wrap(&mut this, wrapper)?;
  ctx.env.get_undefined()
}

#[js_function(0)]
pub fn create_file_class(ctx: CallContext) -> Result<JsFunction> {
  ctx.env.define_class(
    "File",
    file_constructor,
    &[
      napi::Property::new(ctx.env, "format")?.with_method(format),
      napi::Property::new(ctx.env, "symbols")?.with_method(symbols),
      napi::Property::new(ctx.env, "dynamic_symbols")?.with_method(dynamic_symbols),
      napi::Property::new(ctx.env, "section_by_name")?.with_method(section_by_name),
      napi::Property::new(ctx.env, "section_by_index")?.with_method(section_by_index),
    ],
  )
}
