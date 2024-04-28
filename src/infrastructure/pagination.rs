use super::dto::pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination};

pub struct Pagination {}

impl Pagination {

    pub fn paginate<T: Clone>(params: DTOQueryPagination, collection: Vec<T>) -> DTOPaginatedCollection<T> {
        let size = collection.len();
        let offset = params.offset;
        let limit = params.limit;

        if offset >= size {
            let previous = Pagination::calculate_previous(size, offset, limit);
            return DTOPaginatedCollection::new(size, previous, size, Vec::new());
        }

        if size == 0 || limit == 0 {
            return DTOPaginatedCollection::new(size, offset, offset, Vec::new());
        }

        let mut limit_fixed = limit;
        if offset + limit >= size {
            limit_fixed = size - offset;
        }

        let cursor = offset + limit_fixed;

        let next = Pagination::calculate_next(size, cursor, limit);
        let previous = Pagination::calculate_previous(size, cursor, limit);

        let slice = collection[offset..=cursor-1].to_vec();

        return DTOPaginatedCollection::new(size, previous, next, slice);
    }

    fn calculate_next(size: usize, cursor: usize, limit: usize) -> usize {
        let next = cursor + limit;
        if next >= size {
            return size;
        }
        return next;
    }

    fn calculate_previous(size: usize, cursor: usize, limit: usize) -> usize {
        if cursor.checked_sub(limit).is_none() {
            return 0;
        }
        let previous = cursor - limit;
        if previous > size {
            return Pagination::calculate_previous(size, size, limit);
        }
        return previous;
    }

}