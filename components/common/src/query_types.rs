#[non_exhaustive]
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
struct Schema {
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
struct Tuple {
    val: i32,
}

#[allow(dead_code)]
impl Tuple {
    fn construct_from_schema(value: Schema) -> Self {
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

        Self { val }
    }
}

#[allow(dead_code)]
struct TableHeap {
    data: Vec<TablePage>,
}

#[allow(dead_code)]
impl TableHeap {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn add_table_page(&mut self, page: TablePage) {
        self.data.push(page)
    }
}

#[allow(dead_code)]
struct TablePage {
    data: Vec<Tuple>,
}
#[allow(dead_code)]
impl TablePage {
    fn new(data: Vec<Tuple>) -> Self {
        Self { data }
    }
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
        let tuple = Tuple::construct_from_schema(schema);
        let table_page = TablePage::new(vec![tuple]);
        let mut table_heap = TableHeap::new();
        table_heap.add_table_page(table_page);
    }
}
