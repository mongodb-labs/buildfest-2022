package org.buildfest2022.qe;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Pattern;

public class Credentials {

    private static String getRequiredEnv (String name) {
        String got = System.getenv(name);
        if (null == got) {
            throw new RuntimeException ("Expected environment variable " + name + " to be set");
        }
        return got;
    }
    private static Map<String, String> yourCredentials;
    static {
        yourCredentials = new HashMap<>();
        // Mongo Paths + URI
        yourCredentials.put("MONGODB_URI", getRequiredEnv("MONGODB_URI"));
        yourCredentials.put("SHARED_LIB_PATH", getRequiredEnv("SHARED_LIB_PATH"));
        // AWS Credentials
        yourCredentials.put("AWS_ACCESS_KEY_ID", getRequiredEnv("AWS_ACCESS_KEY_ID"));
        yourCredentials.put("AWS_SECRET_ACCESS_KEY", getRequiredEnv("AWS_SECRET_ACCESS_KEY"));
        yourCredentials.put("AWS_KEY_REGION", getRequiredEnv("AWS_KEY_REGION"));
        yourCredentials.put("AWS_KEY_ARN", getRequiredEnv("AWS_KEY_ARN"));

    }
    private static void checkPlaceholders() throws Exception {
        Pattern p = Pattern.compile("<.*>$");
        ArrayList<String> errorBuffer = new ArrayList<String>();
        for (Map.Entry<String,String> entry : yourCredentials.entrySet()) {
            if(p.matcher(String.valueOf(entry.getValue())).matches()){
                String message = String.format("The value for %s is empty. Please enter something for this value.", entry.getKey());
                errorBuffer.add(message);
            }
        }
        if (!errorBuffer.isEmpty()){
            String message = String.join("\n", errorBuffer);
            throw new Exception(message);
        }
    }
    public static Map<String, String> getCredentials() throws Exception {
        checkPlaceholders();
        return yourCredentials;
    }
}