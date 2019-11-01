package com.energy.sgx.utils;

import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.regex.Pattern;
import lombok.extern.slf4j.Slf4j;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-11-01
 */
@Slf4j
public class CommonUtil {

    public static Date checkValidDate(String date) throws RuntimeException {
        if (!StringUtils.isEmpty(date)) {
            String pattern = "\\d{4}(\\-)\\d{1,2}\\1\\d{1,2}";
            boolean isMatch = Pattern.matches(pattern, date);
            if (!isMatch) {
                throw new RuntimeException("Invalid date format");
            }
        }
        return convertStringToDate(date);
    }

    private static Date convertStringToDate(String sdf) {
        SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd");
        try {
            return df.parse(sdf);
        } catch (ParseException e) {
            throw new RuntimeException("parse date error");
        }
    }
}
