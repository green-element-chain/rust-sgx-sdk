package com.energy.sgx.sgxdata.service.impl.project;

import com.energy.sgx.sgxdata.dto.request.ProjectAssetVo;
import com.energy.sgx.sgxdata.dto.request.ProjectAssetVo.ProjectAsset;
import com.energy.sgx.utils.ListUtils;
import com.energy.utils.EntityUtil;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import javax.persistence.EntityManager;
import javax.persistence.Query;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.ObjectUtils;

/**
 * @author Bryan
 * @date 2019-08-28
 */
@Slf4j
@Service
public class ProjectAssetService {

    @Autowired
    private EntityManager em;

    public List<List<ProjectAssetVo>> get(Integer time) {
        StringBuilder sql = new StringBuilder("select project_id,asset_id")
            .append(" from project_related_assets")
            .append(" where create_time >")
            .append(time);

        Query query = em.createNativeQuery(sql.toString());
        List<Object[]> objects = query.getResultList();

        if (!ObjectUtils.isEmpty(objects)) {
            List<ProjectAssetVo> list = convert(objects);
            return ListUtils.splitGroup(list);
        }
        return null;
    }

    private List<ProjectAssetVo> convert(List<Object[]> objects) {
        List<ProjectAssetVo> assetVos = new ArrayList<>();

        Map<Integer, List<Integer>> projectMap = new HashMap<>();
        for (Object[] object : objects) {
            ProjectAsset asset = EntityUtil.castEntity(object, ProjectAsset.class);
            if (asset == null) {
                throw new RuntimeException("cast entity failed.");
            }
            List<Integer> values = projectMap.get(asset.getProjectId());
            if (values == null) {
                values = new ArrayList<>();
                projectMap.put(asset.getProjectId(), values);
            }
            values.add(asset.getAssetId());
        }

        projectMap.forEach((project, assets) -> {
            assetVos.add(new ProjectAssetVo(project, assets));
        });
        return assetVos;
    }
}
