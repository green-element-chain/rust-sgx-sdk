package com.energy.sgx.order.dto;

import java.util.ArrayList;
import java.util.List;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-08-19
 */
@Data
public class ProjectAssetVo {

    private Integer projectId;
    private List<Integer> assets = new ArrayList<>();
}
