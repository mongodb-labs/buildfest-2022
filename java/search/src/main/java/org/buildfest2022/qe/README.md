Files in this directory are copied and modified from:
https://github.com/mongodb-university/docs-in-use-encryption-examples/tree/main/queryable-encryption/java/aws/reader/src/main/java/com/mongodb/qe

To set up, create a data key and Queryable Encryption enabled collection with the following:
```
export MONGODB_URI="<Example: mongodb://localhost:27017>"
export SHARED_LIB_PATH="<Path to mongo_crypt_v1.dylib>"
export LOCAL_KEY_BASE64="<96 bytes>"
./gradlew setupQE
```

`openssl rand -base64 96` may be used to create `LOCAL_KEY_BASE64`.