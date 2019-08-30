package com.energy.sgx.sgxdata.service.impl.order;

import com.energy.sgx.sgxdata.dto.request.AssetOrderVo;
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
public class AssetOrderService {

    @Autowired
    private EntityManager em;

    public List<List<AssetOrderVo>> get(Integer time) {
        StringBuilder sql = new StringBuilder("select")
            .append(" id,asset_type,asset_id,revenue,order_time/1000")
            .append(" from asset_order_attr")
            .append(" where update_time >")
            .append(time);

        Query query = em.createNativeQuery(sql.toString());
        List<Object[]> objects = query.getResultList();

        if (!ObjectUtils.isEmpty(objects)) {
            List<AssetOrderVo> list = EntityUtil.castEntity(objects, AssetOrderVo.class);
            return ListUtils.splitGroup(list);
        }
        return null;
    }


}
