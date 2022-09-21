#ifndef AtlasManager_h
#define AtlasManager_h

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/uri.hpp>
#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/json.hpp>
#include <iostream>

class AtlasManager {

private:
	static AtlasManager* sInstance;

	mongocxx::instance inst{};
  mongocxx::client _mongoClient{mongocxx::uri{getEnvVar("ATLAS_URI")}};

public:
	static AtlasManager* Instance();
	void TestConnection();

private:
	AtlasManager();
	~AtlasManager();

	std::string getEnvVar( std::string const & key ) const
	{
			char * val = getenv( key.c_str() );
			return val == NULL ? std::string("") : std::string(val);
	}
};


#endif /* AtlasManager_h */