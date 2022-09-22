package org.buildfest2022.qe;

import java.util.HashMap;
import java.util.Map;

public class Credentials {

  private static String getRequiredEnv(String name) {
    String got = System.getenv(name);
    if (null == got) {
      throw new RuntimeException("Expected environment variable " + name + " to be set");
    }
    return got;
  }

  private static Map<String, String> yourCredentials;

  static {
    yourCredentials = new HashMap<>();
    // Mongo Paths + URI
    yourCredentials.put("MONGODB_URI", getRequiredEnv("MONGODB_URI"));
    yourCredentials.put("SHARED_LIB_PATH", getRequiredEnv("SHARED_LIB_PATH"));
    yourCredentials.put("LOCAL_KEY_BASE64", getRequiredEnv("LOCAL_KEY_BASE64"));
  }

  public static Map<String, String> getCredentials() throws Exception {
    return yourCredentials;
  }
}
