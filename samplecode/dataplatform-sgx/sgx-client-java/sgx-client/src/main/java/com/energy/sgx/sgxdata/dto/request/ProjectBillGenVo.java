package com.energy.sgx.sgxdata.dto.request;

import java.util.ArrayList;
import java.util.List;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-08-29
 */
@Data
public class ProjectBillGenVo {

    private Integer day;
    private List<Integer> projects;

    public ProjectBillGenVo(Integer day) {
        this.day = day;
        this.projects = new ArrayList<>();
    }

    public void add(Integer projectId) {
        this.projects.add(projectId);
    }
}
