package com.energy.sgx.sgxdata.dto.request;

import java.util.List;
import lombok.AllArgsConstructor;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-08-19
 */
@Data
@AllArgsConstructor
public class ProjectAssetVo {

    private Integer projectId;
    private List<Integer> assets;

    @Data
    @AllArgsConstructor
    public static class ProjectAsset {

        private Integer projectId;
        private Integer assetId;
    }
}
