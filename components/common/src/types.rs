

#[derive(Clone, Debug,PartialEq)]
pub enum TxnResult <T, E>{  
    Ok(T),
    Err(E),
}