#ifndef AtlasManager_h
#define AtlasManager_h

#include <mongocxx/client.hpp>
#include <mongocxx/instance.hpp>
#include <mongocxx/uri.hpp>
#include <mongocxx/pool.hpp>
#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/json.hpp>
#include <iostream>

#include "Vec2.h"

class AtlasManager {

private:
	static AtlasManager* sInstance;

public:
	static AtlasManager* Instance();
	void TestConnection();

	mongocxx::collection getCollection(std::string dbname, std::string collname);
	mongocxx::pool::entry getClient();
	void WritePlayerMove(Vec2 position, Vec2 velocity);

private:
	AtlasManager();
	~AtlasManager();

	mongocxx::pool _connectionPool{mongocxx::uri(getEnvVar("ATLAS_URI"))};

	std::string getEnvVar( std::string const & key ) const
	{
			char * val = getenv( key.c_str() );
			return val == NULL ? std::string("") : std::string(val);
	}
};


#endif /* AtlasManager_h */