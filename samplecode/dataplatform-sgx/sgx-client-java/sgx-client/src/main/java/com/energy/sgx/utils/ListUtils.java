package com.energy.sgx.utils;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import lombok.extern.slf4j.Slf4j;

/**
 * @author Bryan
 * @date 2019-08-30
 */
@Slf4j
public class ListUtils {

    private static final Integer MAX_NUMBER = 150;

    public static <T> List<List<T>> splitGroup(List<T> list) {
        int limit = countStep(list.size());
        return Stream.iterate(0, n -> n + 1)
            .limit(limit)
            .parallel()
            .map(a -> list.stream()
                .skip(a * MAX_NUMBER)
                .limit(MAX_NUMBER)
                .parallel()
                .collect(Collectors.toList()))
            .collect(Collectors.toList());
    }

    private static Integer countStep(Integer size) {
        return (size + MAX_NUMBER - 1) / MAX_NUMBER;
    }
}
