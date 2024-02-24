use parsiphae::{
    dat::{GothicDat, SymbolFlags, SymbolType},
    types::{parsed::zPAR_TYPE, External, PrintableByteVec},
};

pub fn type_to_string(typ: &zPAR_TYPE) -> String {
    match typ {
        zPAR_TYPE::Void => "zPAR_TYPE::Void".into(),
        zPAR_TYPE::Float => "zPAR_TYPE::Float".into(),
        zPAR_TYPE::Int => "zPAR_TYPE::Int".into(),
        zPAR_TYPE::Func => "zPAR_TYPE::Func".into(),
        zPAR_TYPE::String => "zPAR_TYPE::String".into(),
        zPAR_TYPE::Instance(_) => "zPAR_TYPE::Instance(inst.clone())".into(),
    }
}
#[test]
fn extract() {
    let dat_file = include_bytes!("C:/Users/Leon/Downloads/GOTHIC.DAT");
    let dat = GothicDat::parse(dat_file).unwrap();

    for (idx, symbol) in dat.symbols.iter().enumerate() {
        if symbol.flags.contains(SymbolFlags::External) {
            let parameters: Vec<_> = dat
                .symbols
                .iter()
                .skip(idx + 1)
                .take(symbol.count as usize)
                .map(|symb| symb.typ.to_zPAR_TYPE().unwrap())
                .collect();
            let return_type = if symbol.flags.contains(SymbolFlags::Return) {
                SymbolType::from_u32(symbol.off_cls_ret)
                    .unwrap()
                    .to_zPAR_TYPE()
                    .unwrap()
            } else {
                zPAR_TYPE::Void
            };

            let name_length = symbol.name.len();
            let external = External {
                name: PrintableByteVec(symbol.name[..name_length - 1].to_vec()), // Strip newline
                parameters,
                return_type,
                address: symbol.symbol_data.as_ref().unwrap().get_funclike().unwrap(),
            };
            let params = external
                .parameters
                .iter()
                .map(|p| type_to_string(p))
                .collect::<Vec<_>>()
                .join(", ");
            println!("External {{ name: PrintableByteVec(b\"{}\".to_vec()), parameters: vec![{}], return_type: {}, address: {}}},",
                String::from_utf8_lossy(&external.name.0),
                params,
                type_to_string(&external.return_type),
                external.address,
            );
        }
    }
}
