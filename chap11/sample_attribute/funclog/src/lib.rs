use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, Block, ItemFn, __private::ToTokens};

#[proc_macro_attribute]
pub fn funclog(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn); // parse対象が関数なので
    let ident = &ast.sig.ident;
    let block = ast.block.as_ref(); // 関数本体はItemFnのblockに格納している。
    let block: Block = parse_quote! {{
        println!("Enter: {}", stringify!(#ident));
        let block = || #block;
        let ret = block();
        println!("Exit: {}", stringify!(#ident));
        ret
    }};

    *ast.block = block;
    ast.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
