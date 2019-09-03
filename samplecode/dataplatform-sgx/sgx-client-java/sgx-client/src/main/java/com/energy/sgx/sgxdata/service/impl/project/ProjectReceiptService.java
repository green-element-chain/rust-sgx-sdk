package com.energy.sgx.sgxdata.service.impl.project;

import com.energy.sgx.sgxdata.dto.request.ProjectReceiptVo;
import com.energy.sgx.utils.ListUtils;
import com.energy.utils.EntityUtil;
import java.util.List;
import javax.persistence.EntityManager;
import javax.persistence.Query;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;
import org.springframework.util.ObjectUtils;

/**
 * @author Bryan
 * @date 2019-09-03
 */
@Slf4j
@Component
public class ProjectReceiptService {

    @Autowired
    private EntityManager em;

    public List<List<ProjectReceiptVo>> get(Integer time) {
        StringBuilder sql = new StringBuilder()
            .append("select project_id,charge_model")
            .append(",ifnull(card_num,'') as card,ifnull(name,'') as name, ifnull(mobile,'') as mobile")
            .append(" from project_receipt")
            .append(" where update_time >")
            .append(time);

        Query query = em.createNativeQuery(sql.toString());
        List<Object[]> objects = query.getResultList();

        if (!ObjectUtils.isEmpty(objects)) {
            List<ProjectReceiptVo> list = EntityUtil.castEntity(objects, ProjectReceiptVo.class);
            return ListUtils.splitGroup(list);
        }
        return null;
    }
}
