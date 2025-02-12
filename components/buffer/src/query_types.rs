use rand::random;
#[allow(unused)]
use std::sync::{Arc, Mutex};

use crate::skiplistindex::SkipListIndex;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum TypeId {
    INVALID = 0,
    BOOLEAN,
    TINYINT,
    SMALLINT,
    INTEGER,
    BIGINT,
    DECIMAL,
    VARCHAR,
    TIMESTAMP,
    VECTOR,
}

#[allow(dead_code)]
impl TypeId {
    fn type_size(id: TypeId, length: Option<u32>) -> u32 {
        match id {
            TypeId::BOOLEAN | TypeId::TINYINT => 1,
            TypeId::SMALLINT => 2,
            TypeId::INTEGER => 4,
            TypeId::BIGINT | TypeId::DECIMAL | TypeId::TIMESTAMP => 8,
            TypeId::VARCHAR => length.unwrap(),
            TypeId::VECTOR => length.unwrap_or(0) * std::mem::size_of::<f32>() as u32,
            TypeId::INVALID => 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Column {
    name: String,
    id: TypeId,
    length: u32,
    offset: u32,
}

#[allow(dead_code)]
impl Column {
    pub(crate) fn new(name: String, id: TypeId, length: u32) -> Self {
        Self {
            name,
            id,
            length,
            offset: 0,
        }
    }

    fn get_offset(&self) -> u32 {
        self.offset
    }
}
#[allow(dead_code)]
#[derive(Default, Clone, Debug)]
pub struct Schema {
    columns: Vec<Column>,
    length: usize,
    tuple_is_inlined: bool,
}

#[allow(dead_code)]
impl Schema {
    fn new(columns: Vec<Column>) -> Self {
        let length = columns.len();
        Self {
            columns,
            length,
            tuple_is_inlined: true,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Tuple {
    id: u64,
    val: u64,
    offset: usize,
}

#[allow(dead_code)]
impl Tuple {
    fn construct_from_schema(id: u64, value: Schema) -> Self {
        let mut val = 0;
        for i in value.columns {
            match i.id {
                TypeId::BOOLEAN | TypeId::TINYINT => val += 1,
                TypeId::SMALLINT => val += 2,
                TypeId::INTEGER => val += 4,
                TypeId::BIGINT | TypeId::DECIMAL | TypeId::TIMESTAMP => val += 8,
                TypeId::VARCHAR => val += 100,
                TypeId::VECTOR => val += 100,
                TypeId::INVALID => val += 1,
            }
        }

        Self {
            id,
            val,
            offset: val as usize,
        }
    }
}

// use serde::{Deserialize, Serialize};
//       let mut serialized_tuples = [0u8; 4096];

//         for (index, tuple) in data.iter().enumerate() {
//             let mut tuple_serialized = [0u8; 24];
//             let val_bytes = tuple.val.to_be_bytes();
//             let id_bytes = tuple.id.to_be_bytes();
//             let offset_bytes = tuple.offset.to_be_bytes();
//             tuple_serialized[0..8].copy_from_slice(&id_bytes);
//             tuple_serialized[8..16].copy_from_slice(&val_bytes);
//             tuple_serialized[16..24].copy_from_slice(&offset_bytes);
//             let offset = index * 24;
//             serialized_tuples[offset..offset + 24].copy_from_slice(&tuple_serialized);
//         }

//         Self {
//             data: serialized_tuples,
//         }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TablePage {
    pub data: Vec<Tuple>,
}
#[allow(dead_code)]
impl TablePage {
    fn new(data: Vec<Tuple>) -> Self {
        Self { data }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TableHeap {
    pub data: Vec<TablePage>,
    index: SkipListIndex,
}

#[allow(dead_code)]
impl TableHeap {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            index: SkipListIndex::new(),
        }
    }

    pub fn add_table_page(&mut self, page: TablePage) {
        self.data.push(page)
    }

    pub fn create_index(&mut self) -> Box<SkipListIndex> {
        for i in self.data.iter() {
            for j in i.data.iter() {
                self.index.insert(j.id, j.val, j.offset);
            }
        }
        let skip_list_index = self.index.clone();
        Box::new(skip_list_index)
    }
}

fn get_demo_columns() -> Vec<Column> {
    let c1 = Column::new("name".to_string(), TypeId::VARCHAR, 20);
    let c2 = Column::new("lastname".to_string(), TypeId::VARCHAR, 20);
    let c3 = Column::new("address".to_string(), TypeId::VARCHAR, 20);
    let c4 = Column::new("salary".to_string(), TypeId::BIGINT, 4);
    let c5 = Column::new("age".to_string(), TypeId::SMALLINT, 4);
    vec![c1, c2, c3, c4, c5]
}

pub fn get_demo_schema() -> Schema {
    let columns = get_demo_columns();
    let schema = Schema::new(columns);
    schema
}

fn get_demo_tuple() -> Tuple {
    let tuple = Tuple::construct_from_schema(random(), get_demo_schema());
    tuple
}

fn get_demo_table_page(tuples: Vec<Tuple>) -> TablePage {
    let table_page = TablePage::new(tuples);
    table_page
}

pub fn get_demo_table_heap_with_n_page_m_tuples_each(n: usize, m: usize) -> TableHeap {
    let mut tuples = vec![];
    let mut table_pages = vec![];
    for _ in 0..n {
        for _ in 0..m {
            tuples.push(get_demo_tuple());
        }
        table_pages.push(get_demo_table_page(tuples.to_owned()));
    }
    let mut table_heap = TableHeap::new(n);
    for i in table_pages {
        table_heap.add_table_page(i);
    }
    table_heap
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_schema() {
        let c1 = Column::new("name".to_string(), TypeId::VARCHAR, 20);
        let c2 = Column::new("lastname".to_string(), TypeId::VARCHAR, 20);
        let c3 = Column::new("address".to_string(), TypeId::VARCHAR, 20);
        let c4 = Column::new("salary".to_string(), TypeId::BIGINT, 4);
        let c5 = Column::new("age".to_string(), TypeId::SMALLINT, 4);
        let schema = Schema::new(vec![c1, c2, c3, c4, c5]);
        let tuple = Tuple::construct_from_schema(random(), schema);
        let table_heap = Arc::new(Mutex::new(TableHeap::new(1)));
        for _ in 0..20 {
            let table_page = TablePage::new(vec![tuple.clone()]);
            let fake = Arc::clone(&table_heap);
            std::thread::spawn(move || {
                fake.lock().unwrap().add_table_page(table_page);
            });
        }
    }

    #[test]
    fn test_page_heap_create_index() {
        // TODO: fix the bug in the range query for the table_heap as skip list is not returning range for this test
        // strangely it works in other test above?
        let mut table_heap = get_demo_table_heap_with_n_page_m_tuples_each(5, 20);
        let box_cloned_list = table_heap.create_index();
        let a = table_heap.index.range_query(1, 100);
        // let b = 20u64;
        println!("{:?}; {:?}", a.get(0), box_cloned_list);
    }
}
