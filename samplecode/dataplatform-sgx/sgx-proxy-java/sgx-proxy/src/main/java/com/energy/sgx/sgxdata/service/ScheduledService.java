package com.energy.sgx.sgxdata.service;

import java.util.Calendar;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

/**
 * @author Bryan
 * @date 2019-08-28
 */
@Slf4j
@Component
public class ScheduledService {

    @Autowired
    private DataService dataService;
    @Autowired
    private TransactionService transactionService;

    @Scheduled(cron = "0 0 2 * * *")
    public void scheduleOfData() {
        log.info("the schedule of data task run at {}", System.currentTimeMillis());
        dataService.transferProjectAssetToSgx();
        dataService.transferProjectLedgerToSgx();
        dataService.transferAssetOrderToSgx();
    }

    @Scheduled(cron = "0 30 2 * * *")
    public void scheduleOfBill() {
        log.info("the schedule of bill task run at {}", System.currentTimeMillis());
        Calendar c = Calendar.getInstance();
        Integer day = c.get(Calendar.DAY_OF_MONTH);
        dataService.generateProjectBillToSgx(day, null);
    }

    @Scheduled(cron = "0 0 3 * * *")
    public void scheduleOfPayment() {
        log.info("the schedule of payment task run at {}", System.currentTimeMillis());
        Calendar c = Calendar.getInstance();
        Integer day = c.get(Calendar.DAY_OF_MONTH);
        transactionService.paymentToSgxServer(null, day);
    }
}
