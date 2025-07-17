use lumx_types::pagination::{Paged, Pagination};
use sea_orm::{ConnectionTrait, DbErr, Paginator, PaginatorTrait, Selector, SelectorTrait};

pub trait IntoPaged<S>
where
    S: SelectorTrait,
{
    fn into_paged(
        self,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Paged<S::Item>, DbErr>>;
}

pub trait Paginate<'db, C, S>
where
    C: ConnectionTrait,
    S: SelectorTrait,
{
    fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Paged<S::Item>, DbErr>>;
}

impl<'db, C, S> Paginate<'db, C, S> for Selector<S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    async fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> Result<Paged<S::Item>, DbErr> {
        let paginator = PaginatorTrait::paginate(self, conn, pagination.page_size);

        let paged = paginator.into_paged(pagination).await?;

        Ok(paged)
    }
}

impl<'db, C, S> IntoPaged<S> for Paginator<'db, C, S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    async fn into_paged(self, pagination: &Pagination) -> Result<Paged<S::Item>, DbErr> {
        let page_numbers = self.num_items_and_pages().await?;

        let items = self.fetch_page(pagination.page.to_owned()).await?;

        Ok(Paged {
            data: items,
            total: page_numbers.number_of_items,
            page: page_numbers.number_of_pages,
            page_size: pagination.page_size,
        })
    }
}
