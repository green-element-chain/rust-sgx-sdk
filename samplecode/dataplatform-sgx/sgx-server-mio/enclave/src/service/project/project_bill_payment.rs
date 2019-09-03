use std::sync::Arc;
use utils::db::DbContext;

#[derive(Clone)]
pub struct ProjectBillPaymentMgr {
    db_context: Arc<DbContext>,
}

impl ProjectBillPaymentMgr {
    pub fn new(context: &Arc<DbContext>) -> ProjectBillPaymentMgr {
        let bill_payment_mgr = ProjectBillPaymentMgr {
            db_context: context.clone(),
        };
        bill_payment_mgr.init_table();
        bill_payment_mgr
    }

    fn init_table(&self) {
        let sql = "
        create table if not exists project_bill_payment (
            id integer primary key autoincrement,
            order_no varchar(255) not null,
            amount int not null,
            tran_method smallint not null,
            tran_time datetime null,
            status smallint not null,
            resp_code int not null,
            resp_msg varchar(255) null,
            query_times int not null,
            acq_seq_id varchar(18) null,
            create_at datetime not null default (datetime('now')),
            update_at datetime not null default (datetime('now'))
        );
        create trigger if not exists [BillPaymentLastUpdateTime]
            after update
            on project_bill_payment
            for each row
            when NEW.update_at <= OLD.update_at
        begin
            update project_bill_payment set update_at=(datetime('now')) where id=OLD.id;
        end;";
        self.db_context.execute(sql);
    }
}