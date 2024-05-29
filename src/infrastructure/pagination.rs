use super::dto::pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination};

pub struct Pagination {}

impl Pagination {

    pub fn paginate<T: Clone>(params: DTOQueryPagination, collection: Vec<T>) -> DTOPaginatedCollection<T> {
        let size = collection.len();
        let limit = params.limit;
        let offset = params.offset;

        if limit >= size {
            let previous = Pagination::calculate_previous(size, limit, offset);
            return DTOPaginatedCollection::new(size, previous, size, Vec::new());
        }

        if size == 0 || offset == 0 {
            return DTOPaginatedCollection::new(size, limit, limit, Vec::new());
        }

        let mut offset_fixed = offset;
        if limit + offset >= size {
            offset_fixed = size - limit;
        }

        let cursor = limit + offset_fixed;

        let next = Pagination::calculate_next(size, cursor, offset);
        let previous = Pagination::calculate_previous(size, cursor, offset);

        let slice = collection[limit..=cursor-1].to_vec();

        return DTOPaginatedCollection::new(size, previous, next, slice);
    }

    fn calculate_next(size: usize, cursor: usize, offset: usize) -> usize {
        let next = cursor + offset;
        if next >= size {
            return size;
        }
        return next;
    }

    fn calculate_previous(size: usize, cursor: usize, offset: usize) -> usize {
        if cursor.checked_sub(offset).is_none() {
            return 0;
        }
        let previous = cursor - offset;
        if previous > size {
            return Pagination::calculate_previous(size, size, offset);
        }
        return previous;
    }

}