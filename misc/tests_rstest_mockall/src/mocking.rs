#[allow(dead_code)]
#[allow(unused_imports)]
use crate::errors::*;
//use errors::*;

#[mockall::automock]
trait DatabaseExecutor {
    fn send_query(&mut self, q: String);
    fn send_queryV2(&mut self, q: String) -> anyhow::Result<()>;
}

fn send_some_query_to_db(mut db: Box<dyn DatabaseExecutor>, id: i32) -> anyhow::Result<()> {
    let q = format!("query {}", id);
    db.send_query(q);
    Ok(())
}

fn send_query_v2(mut db: Box<dyn DatabaseExecutor>, id: i32) -> anyhow::Result<u16> {
    let q = format!("queryv2 {}", id);
    db.send_queryV2(q)?;
    Ok(13_u16)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::rstest;
    #[rstest]
    #[case(42, format!("query {}",42_i32))]
    #[case(555, format!("query {}",555_i32))]
    fn test_initialize(#[case] input: i32, #[case] expected: String) {
        let mut mockdb = Box::new(MockDatabaseExecutor::new());
        mockdb
            .expect_send_query()
            .with(eq(expected))
            .times(1)
            .returning(|_x| ());
        // оно настроило и ожидает, что будет вызвана квери с ид 22
        assert!(send_some_query_to_db(mockdb, input).is_ok());
    }
    //     #[ignore]
    #[rstest]
    #[case(42, format!("queryv2 {}",42_i32))]
    #[case(2, format!("queryv2 {}",2_i32))]
    #[should_panic]
    #[case(1, format!("queryv2 {}",2_i32))]
    fn test_queryv2(#[case] input: i32, #[case] expected: String) {
        let mut mockdb = Box::new(MockDatabaseExecutor::new());
        mockdb
            .expect_send_queryV2()
            .with(eq(expected))
            .times(1)
            .returning(|_x| Ok(())); //  здесь мы указываем, что нам вернет наш мод. объект
                                     //  .returning(|_x| anyhow::bail!("_")); //  а здесь, что мы вернем ошибку
                                     //println!("dbg {:?}", send_query_v2(mockdb, input)); // будем анализировать что вернулся ОК
        assert!(send_query_v2(mockdb, input).is_ok());
        //         assert!(send_query_v2(mockdb, input).is_ok());
    }
    #[rstest]
    #[case(42, format!("queryv2 {}",42_i32))]
    fn test_queryv2_err(#[case] input: i32, #[case] expected: String) {
        let mut mockdb = Box::new(MockDatabaseExecutor::new());
        mockdb
            .expect_send_queryV2()
            .with(eq(expected))
            .times(1)
            .returning(|_x| anyhow::bail!("_")); // вощвращаем ошибку
        assert!(send_query_v2(mockdb, input).is_err());
    }
}
