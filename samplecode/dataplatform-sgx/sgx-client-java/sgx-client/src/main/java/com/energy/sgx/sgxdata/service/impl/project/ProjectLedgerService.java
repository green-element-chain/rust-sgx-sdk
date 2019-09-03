package com.energy.sgx.sgxdata.service.impl.project;

import com.energy.sgx.sgxdata.dto.request.ProjectLedgerVo;
import com.energy.sgx.utils.ListUtils;
import com.energy.utils.EntityUtil;
import java.util.List;
import javax.persistence.EntityManager;
import javax.persistence.Query;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.ObjectUtils;

/**
 * @author Bryan
 * @date 2019-08-29
 */
@Slf4j
@Service
public class ProjectLedgerService {

    @Autowired
    private EntityManager em;

    public List<List<ProjectLedgerVo>> get(Integer time) {
        //上链状态：0是未上链，1是已上链，2是已失效
        //分账模式：LedgerByRate为1，LedgerByMoney为0
        StringBuilder sql = new StringBuilder()
            .append("select project_id,order_date,date_format(first_bill_start_time, '%Y-%m-%d'),cycle,ledger_date")
            .append(",case model when \"LedgerByRate\" then 1 else 0 end as model,message")
            .append(" from project_ledger_account")
            .append(" where is_chain = 1 and (update_time is null or update_time >")
            .append(time).append(")");

        Query query = em.createNativeQuery(sql.toString());
        List<Object[]> objects = query.getResultList();

        if (!ObjectUtils.isEmpty(objects)) {
            List<ProjectLedgerVo> list = EntityUtil.castEntity(objects, ProjectLedgerVo.class);
            return ListUtils.splitGroup(list);
        }
        return null;
    }
}
